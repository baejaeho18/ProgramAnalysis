use std::collections::HashMap;

use rustc_middle::{
    mir::{
        BasicBlock, BinOp, Body, Local, Location, Operand, Place, Rvalue, Statement,
        StatementKind, Terminator, TerminatorKind,
    },
    ty::TyCtxt,
};

use crate::domains::*;

/// Runs interval analysis on the MIR body of the function `f`.
pub fn analyze(tcx: TyCtxt<'_>) -> HashMap<Location, AbsState> {
    for def_id in tcx.hir_body_owners() {
        if tcx.item_name(def_id.to_def_id()).as_str() == "f" {
            let body = tcx.optimized_mir(def_id);

            use crate::rustc_middle::mir::visit::Visitor as _;
            let mut visitor = CmpVisitor::default();
            visitor.visit_body(body);

            let analysis = IntervalAnalysis {
                cmp_assigns: visitor.cmp_assigns,
            };
            let mut states = analysis.find_fixed_point(body);
            analysis.narrow_fixed_point(body, &mut states);

            return states;
        }
    }
    unreachable!()
}

// =====================================================================
// IntervalAnalysis
// =====================================================================

struct IntervalAnalysis {
    cmp_assigns: HashMap<Local, (BinOp, Local, i32)>,
}

fn all_locals(body: &Body<'_>) -> Vec<Local> {
    (0..body.local_decls.len())
        .map(Local::from_usize)
        .collect()
}

fn all_locations(body: &Body<'_>) -> Vec<Location> {
    let mut locs = Vec::new();
    for (bb, bbd) in body.basic_blocks.iter_enumerated() {
        for i in 0..=bbd.statements.len() {
            locs.push(Location {
                block: bb,
                statement_index: i,
            });
        }
    }
    locs
}

fn eval_operand(op: &Operand<'_>, state: &AbsState) -> Interval {
    match op {
        Operand::Copy(place) | Operand::Move(place) => {
            state.0.get(&place.local).copied().unwrap_or(Interval::Bot)
        }
        Operand::Constant(c) => {
            let val = c.const_.try_to_scalar_int().unwrap().to_i32();
            Interval::Range(Some(val), Some(val))
        }
    }
}

impl IntervalAnalysis {
    /// Computes the join of all predecessor transfer outputs for a given location.
    fn compute_incoming(
        &self,
        body: &Body<'_>,
        states: &HashMap<Location, AbsState>,
        loc: Location,
        entry_state: &AbsState,
        entry_loc: Location,
        bot: &AbsState,
    ) -> AbsState {
        let mut result = bot.clone();

        // Entry location gets the initial state
        if loc == entry_loc {
            result = result.join(entry_state);
        }

        let bbd = &body.basic_blocks[loc.block];

        if loc.statement_index > 0 {
            // Predecessor is the previous location in the same block (a statement)
            let pred_idx = loc.statement_index - 1;
            let pred_loc = Location {
                block: loc.block,
                statement_index: pred_idx,
            };
            let pred_state = states.get(&pred_loc).unwrap();
            let out = self.transfer_stmt(&bbd.statements[pred_idx], pred_state);
            result = result.join(&out);
        } else {
            // Block entry: predecessors are terminators of blocks that branch here
            for (bb, pred_bbd) in body.basic_blocks.iter_enumerated() {
                let term_loc = Location {
                    block: bb,
                    statement_index: pred_bbd.statements.len(),
                };
                let pred_state = states.get(&term_loc).unwrap();
                let succs = self.transfer_term(pred_bbd.terminator(), pred_state);
                for (out, target) in succs {
                    if target.block == loc.block && target.statement_index == 0 {
                        result = result.join(&out);
                    }
                }
            }
        }

        result
    }

    /// Iterative widening fixed point (Gauss-Seidel style).
    fn find_fixed_point(&self, body: &Body<'_>) -> HashMap<Location, AbsState> {
        let locals = all_locals(body);
        let all_locs = all_locations(body);
        let bot = AbsState::bot(&locals);

        let entry_loc = Location {
            block: BasicBlock::from_u32(0),
            statement_index: 0,
        };
        let entry_state = {
            let mut s = bot.clone();
            for i in 1..=body.arg_count {
                s.0.insert(Local::from_usize(i), Interval::Range(None, None));
            }
            s
        };

        let mut states: HashMap<Location, AbsState> = HashMap::new();
        for &loc in &all_locs {
            states.insert(loc, bot.clone());
        }

        loop {
            let mut changed = false;
            for &loc in &all_locs {
                let incoming =
                    self.compute_incoming(body, &states, loc, &entry_state, entry_loc, &bot);
                let old = states.get(&loc).unwrap();
                let widened = old.widen(&incoming);
                if old != &widened {
                    states.insert(loc, widened);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }

        states
    }

    /// Iterative narrowing (Gauss-Seidel style).
    fn narrow_fixed_point(&self, body: &Body<'_>, states: &mut HashMap<Location, AbsState>) {
        let locals = all_locals(body);
        let all_locs = all_locations(body);
        let bot = AbsState::bot(&locals);

        let entry_loc = Location {
            block: BasicBlock::from_u32(0),
            statement_index: 0,
        };
        let entry_state = {
            let mut s = bot.clone();
            for i in 1..=body.arg_count {
                s.0.insert(Local::from_usize(i), Interval::Range(None, None));
            }
            s
        };

        let max_iters = all_locs.len() * 3;
        for _ in 0..max_iters {
            let mut changed = false;
            for &loc in &all_locs {
                let incoming =
                    self.compute_incoming(body, states, loc, &entry_state, entry_loc, &bot);
                let old = states.get(&loc).unwrap();
                let narrowed = old.narrow(&incoming);
                if old != &narrowed {
                    states.insert(loc, narrowed);
                    changed = true;
                }
            }
            if !changed {
                break;
            }
        }
    }

    fn transfer_stmt(&self, stmt: &Statement<'_>, state: &AbsState) -> AbsState {
        if let StatementKind::Assign(box (place, r)) = &stmt.kind {
            let target = place.local;
            let mut new_state = state.clone();

            let result = match r {
                Rvalue::Use(op) => eval_operand(op, state),
                Rvalue::BinaryOp(op, box (lhs, rhs)) => {
                    let l = eval_operand(lhs, state);
                    let r = eval_operand(rhs, state);
                    match op {
                        BinOp::Add => l.add(&r),
                        BinOp::Sub => l.sub(&r),
                        BinOp::Mul => l.mul(&r),
                        BinOp::Div => l.div(&r),
                        BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge | BinOp::Eq
                        | BinOp::Ne => {
                            return state.clone();
                        }
                        _ => panic!("unsupported"),
                    }
                }
                _ => panic!("unsupported"),
            };

            new_state.0.insert(target, result);
            new_state
        } else {
            panic!("unsupported")
        }
    }

    fn transfer_term(
        &self,
        term: &Terminator<'_>,
        state: &AbsState,
    ) -> Vec<(AbsState, Location)> {
        match &term.kind {
            TerminatorKind::Goto { target } => {
                vec![(
                    state.clone(),
                    Location {
                        block: *target,
                        statement_index: 0,
                    },
                )]
            }
            TerminatorKind::SwitchInt { discr, targets } => {
                let discr_local = discr.place().unwrap().local;
                let (binop, tested_local, constant) = self.cmp_assigns[&discr_local];
                let true_branch = targets.target_for_value(1);
                let false_branch = targets.otherwise();

                let cur = state
                    .0
                    .get(&tested_local)
                    .copied()
                    .unwrap_or(Interval::Bot);

                let true_iv = match binop {
                    BinOp::Lt => cur.filter_lt(constant),
                    BinOp::Le => cur.filter_le(constant),
                    BinOp::Gt => cur.filter_gt(constant),
                    BinOp::Ge => cur.filter_ge(constant),
                    BinOp::Eq => cur.filter_eq(constant),
                    BinOp::Ne => cur.filter_ne(constant),
                    _ => panic!("unsupported"),
                };
                let false_iv = match binop {
                    BinOp::Lt => cur.filter_ge(constant),
                    BinOp::Le => cur.filter_gt(constant),
                    BinOp::Gt => cur.filter_le(constant),
                    BinOp::Ge => cur.filter_lt(constant),
                    BinOp::Eq => cur.filter_ne(constant),
                    BinOp::Ne => cur.filter_eq(constant),
                    _ => panic!("unsupported"),
                };

                let mut results = Vec::new();

                if true_iv != Interval::Bot {
                    let mut s = state.clone();
                    s.0.insert(tested_local, true_iv);
                    results.push((
                        s,
                        Location {
                            block: true_branch,
                            statement_index: 0,
                        },
                    ));
                }
                if false_iv != Interval::Bot {
                    let mut s = state.clone();
                    s.0.insert(tested_local, false_iv);
                    results.push((
                        s,
                        Location {
                            block: false_branch,
                            statement_index: 0,
                        },
                    ));
                }

                results
            }
            TerminatorKind::Return => vec![],
            _ => panic!("unsupported"),
        }
    }
}

// =====================================================================
// AbsState
// =====================================================================

impl AbsState {
    fn bot(locals: &[Local]) -> Self {
        let mut map = HashMap::new();
        for &l in locals {
            map.insert(l, Interval::Bot);
        }
        AbsState(map)
    }

    fn join(&self, other: &Self) -> Self {
        let mut result = HashMap::new();
        for (&local, &iv) in &self.0 {
            let o = other.0.get(&local).copied().unwrap_or(Interval::Bot);
            result.insert(local, iv.join(&o));
        }
        for (&local, &iv) in &other.0 {
            result.entry(local).or_insert(iv);
        }
        AbsState(result)
    }

    fn widen(&self, other: &Self) -> Self {
        let mut result = HashMap::new();
        for (&local, &old_iv) in &self.0 {
            let new_iv = other.0.get(&local).copied().unwrap_or(Interval::Bot);
            result.insert(local, old_iv.widen(&new_iv));
        }
        for (&local, &iv) in &other.0 {
            result.entry(local).or_insert(iv);
        }
        AbsState(result)
    }

    fn narrow(&self, other: &Self) -> Self {
        let mut result = HashMap::new();
        for (&local, &old_iv) in &self.0 {
            let new_iv = other.0.get(&local).copied().unwrap_or(Interval::Bot);
            result.insert(local, old_iv.narrow(&new_iv));
        }
        for (&local, &iv) in &other.0 {
            result.entry(local).or_insert(iv);
        }
        AbsState(result)
    }
}

impl PartialEq for AbsState {
    fn eq(&self, other: &Self) -> bool {
        for (&local, &iv) in &self.0 {
            let o = other.0.get(&local).copied().unwrap_or(Interval::Bot);
            if iv != o {
                return false;
            }
        }
        for (&local, &iv) in &other.0 {
            if !self.0.contains_key(&local) && iv != Interval::Bot {
                return false;
            }
        }
        true
    }
}

// =====================================================================
// Interval operations
// =====================================================================

fn opt_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.wrapping_add(y)),
        _ => None,
    }
}

fn opt_sub(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.wrapping_sub(y)),
        _ => None,
    }
}

fn opt_mul(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(x), Some(y)) => Some(x.wrapping_mul(y)),
        (Some(0), None) | (None, Some(0)) => Some(0),
        _ => None,
    }
}

fn opt_div(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (Some(0), _) => Some(0),
        (Some(x), Some(y)) if y != 0 => Some(x / y),
        (Some(_), None) => Some(0), // finite / ±∞ → 0
        (None, Some(_)) => None,    // ±∞ / finite → ±∞
        _ => None,
    }
}

fn opt_min(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (None, _) | (_, None) => None,
        (Some(x), Some(y)) => Some(x.min(y)),
    }
}

fn opt_max(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    match (a, b) {
        (None, _) | (_, None) => None,
        (Some(x), Some(y)) => Some(x.max(y)),
    }
}

impl Interval {
    fn join(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, x) | (x, Interval::Bot) => *x,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                let l = match (l1, l2) {
                    (Some(a), Some(b)) => Some(*a.min(b)),
                    _ => None,
                };
                let h = match (h1, h2) {
                    (Some(a), Some(b)) => Some(*a.max(b)),
                    _ => None,
                };
                Interval::Range(l, h)
            }
        }
    }

    fn widen(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, y) => *y,
            (x, Interval::Bot) => *x,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                let l3 = match (l1, l2) {
                    (Some(a), Some(b)) if *a <= *b => Some(*a),
                    (None, _) => None,
                    _ => None,
                };
                let h3 = match (h1, h2) {
                    (Some(a), Some(b)) if *a >= *b => Some(*a),
                    (None, _) => None,
                    _ => None,
                };
                Interval::Range(l3, h3)
            }
        }
    }

    fn meet(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, _) | (_, Interval::Bot) => Interval::Bot,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                let l = match (l1, l2) {
                    (Some(a), Some(b)) => Some(*a.max(b)),
                    (Some(a), None) => Some(*a),
                    (None, Some(b)) => Some(*b),
                    (None, None) => None,
                };
                let h = match (h1, h2) {
                    (Some(a), Some(b)) => Some(*a.min(b)),
                    (Some(a), None) => Some(*a),
                    (None, Some(b)) => Some(*b),
                    (None, None) => None,
                };
                match (l, h) {
                    (Some(lo), Some(hi)) if lo > hi => Interval::Bot,
                    _ => Interval::Range(l, h),
                }
            }
        }
    }

    fn narrow(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, _) => Interval::Bot,
            (_, Interval::Bot) => Interval::Bot,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                let l3 = if l1.is_none() { *l2 } else { *l1 };
                let h3 = if h1.is_none() { *h2 } else { *h1 };
                Interval::Range(l3, h3)
            }
        }
    }

    fn add(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, _) | (_, Interval::Bot) => Interval::Bot,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                Interval::Range(opt_add(*l1, *l2), opt_add(*h1, *h2))
            }
        }
    }

    fn sub(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, _) | (_, Interval::Bot) => Interval::Bot,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                Interval::Range(opt_sub(*l1, *h2), opt_sub(*h1, *l2))
            }
        }
    }

    fn mul(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, _) | (_, Interval::Bot) => Interval::Bot,
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                let corners = [
                    opt_mul(*l1, *l2),
                    opt_mul(*l1, *h2),
                    opt_mul(*h1, *l2),
                    opt_mul(*h1, *h2),
                ];
                let lo = corners.iter().copied().reduce(opt_min).unwrap();
                let hi = corners.iter().copied().reduce(opt_max).unwrap();
                Interval::Range(lo, hi)
            }
        }
    }

    fn div(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Bot, _) | (_, Interval::Bot) => Interval::Bot,
            (_, Interval::Range(l2, h2)) => {
                let lo2 = l2.unwrap_or(i32::MIN);
                let hi2 = h2.unwrap_or(i32::MAX);
                if lo2 <= 0 && hi2 >= 0 {
                    Interval::Range(None, None)
                } else {
                    self.div_nonzero(other)
                }
            }
        }
    }

    fn div_nonzero(&self, other: &Self) -> Self {
        match (self, other) {
            (Interval::Range(l1, h1), Interval::Range(l2, h2)) => {
                let corners = [
                    opt_div(*l1, *l2),
                    opt_div(*l1, *h2),
                    opt_div(*h1, *l2),
                    opt_div(*h1, *h2),
                ];
                let lo = corners.iter().copied().reduce(opt_min).unwrap();
                let hi = corners.iter().copied().reduce(opt_max).unwrap();
                Interval::Range(lo, hi)
            }
            _ => Interval::Bot,
        }
    }

    fn filter_lt(&self, n: i32) -> Self {
        self.meet(&Interval::Range(None, Some(n - 1)))
    }

    fn filter_le(&self, n: i32) -> Self {
        self.meet(&Interval::Range(None, Some(n)))
    }

    fn filter_gt(&self, n: i32) -> Self {
        self.meet(&Interval::Range(Some(n + 1), None))
    }

    fn filter_ge(&self, n: i32) -> Self {
        self.meet(&Interval::Range(Some(n), None))
    }

    fn filter_eq(&self, n: i32) -> Self {
        self.meet(&Interval::Range(Some(n), Some(n)))
    }

    fn filter_ne(&self, n: i32) -> Self {
        match self {
            Interval::Bot => Interval::Bot,
            Interval::Range(lo, hi) => {
                if *lo == Some(n) && *hi == Some(n) {
                    Interval::Bot
                } else {
                    *self
                }
            }
        }
    }
}

// =====================================================================
// CmpVisitor
// =====================================================================

#[derive(Default, Debug, Clone)]
struct CmpVisitor {
    cmp_assigns: HashMap<Local, (BinOp, Local, i32)>,
}

impl<'tcx> rustc_middle::mir::visit::Visitor<'tcx> for CmpVisitor {
    fn visit_assign(&mut self, place: &Place<'tcx>, rvalue: &Rvalue<'tcx>, location: Location) {
        self.super_assign(place, rvalue, location);
        if let Rvalue::BinaryOp(
            op @ (BinOp::Lt | BinOp::Le | BinOp::Gt | BinOp::Ge | BinOp::Eq | BinOp::Ne),
            box (l, r),
        ) = rvalue
            && let Some(l) = l.place()
            && let Some(r) = r.constant()
        {
            self.cmp_assigns.insert(
                place.local,
                (*op, l.local, r.const_.try_to_scalar_int().unwrap().to_i32()),
            );
        }
    }
}
