use std::collections::HashMap;

use rustc_ast::{BinOpKind, LitKind};
use rustc_hir::{
    Block, Expr, ExprKind, HirId, Item, ItemKind, PatKind, QPath, Stmt, StmtKind, UnOp, def::Res,
    def_id::LocalDefId, intravisit,
};
use rustc_middle::{hir::nested_filter, ty::TyCtxt};

use crate::types::Type;

/// Final type information produced by the analysis.
pub struct AnalysisResult {
    /// Types for local variables.
    pub locals: HashMap<HirId, Type>,
    /// Return types for functions.
    pub fn_rets: HashMap<LocalDefId, Type>,
}

/// Runs type analysis over the crate and returns local variable types and return types.
///
/// Returns `None` when constraints are inconsistent.
pub fn analyze<'tcx>(tcx: TyCtxt<'tcx>) -> Option<AnalysisResult> {
    // Phase 1: Collect all function signatures and create type variables
    let mut collector = FnCollector {
        tcx,
        infer: TypeInfer::new(),
        fn_ret_ty: HashMap::new(),
        fn_ty: HashMap::new(),
        local_ty: HashMap::new(),
    };
    tcx.hir_visit_all_item_likes_in_crate(&mut collector);

    // Phase 2: Visit all expressions and generate constraints
    let mut visitor = ConstraintVisitor {
        tcx,
        infer: collector.infer,
        expr_ty: HashMap::new(),
        local_ty: collector.local_ty,
        fn_ret_ty: collector.fn_ret_ty,
        fn_ty: collector.fn_ty,
    };
    tcx.hir_visit_all_item_likes_in_crate(&mut visitor);

    if visitor.infer.failed {
        return None;
    }

    // Resolve types
    let mut locals = HashMap::new();
    for (hir_id, var) in &visitor.local_ty {
        let ty = visitor.infer.resolve(*var)?;
        locals.insert(*hir_id, ty);
    }

    let mut fn_rets = HashMap::new();
    for (def_id, var) in &visitor.fn_ret_ty {
        let ty = visitor.infer.resolve(*var)?;
        fn_rets.insert(*def_id, ty);
    }

    Some(AnalysisResult { locals, fn_rets })
}

// ---------------------------------------------------------------------------
// Type inference engine
// ---------------------------------------------------------------------------

#[derive(Clone, Debug)]
enum TypeInfo {
    Bool,
    I32,
    Ref(usize),
    Tuple(Vec<usize>),
    FnPtr(Vec<usize>, usize),
}

struct TypeInfer {
    parent: Vec<usize>,
    rank: Vec<usize>,
    info: Vec<Option<TypeInfo>>,
    failed: bool,
}

impl TypeInfer {
    fn new() -> Self {
        Self {
            parent: Vec::new(),
            rank: Vec::new(),
            info: Vec::new(),
            failed: false,
        }
    }

    fn fresh(&mut self) -> usize {
        let id = self.parent.len();
        self.parent.push(id);
        self.rank.push(0);
        self.info.push(None);
        id
    }

    fn fresh_with(&mut self, ti: TypeInfo) -> usize {
        let id = self.fresh();
        self.info[id] = Some(ti);
        id
    }

    fn make_bool(&mut self) -> usize {
        self.fresh_with(TypeInfo::Bool)
    }
    fn make_i32(&mut self) -> usize {
        self.fresh_with(TypeInfo::I32)
    }
    fn make_ref(&mut self, inner: usize) -> usize {
        self.fresh_with(TypeInfo::Ref(inner))
    }
    fn make_tuple(&mut self, elems: Vec<usize>) -> usize {
        self.fresh_with(TypeInfo::Tuple(elems))
    }
    fn make_fn_ptr(&mut self, params: Vec<usize>, ret: usize) -> usize {
        self.fresh_with(TypeInfo::FnPtr(params, ret))
    }

    fn find(&self, mut x: usize) -> usize {
        while self.parent[x] != x {
            x = self.parent[x];
        }
        x
    }

    fn find_compress(&mut self, x: usize) -> usize {
        let root = self.find(x);
        let mut v = x;
        while self.parent[v] != v {
            let p = self.parent[v];
            self.parent[v] = root;
            v = p;
        }
        root
    }

    fn union_sets(&mut self, x: usize, y: usize) {
        let xr = self.find(x);
        let yr = self.find(y);
        if xr == yr {
            return;
        }
        if self.rank[xr] < self.rank[yr] {
            self.parent[xr] = yr;
        } else if self.rank[xr] > self.rank[yr] {
            self.parent[yr] = xr;
        } else {
            self.parent[yr] = xr;
            self.rank[xr] += 1;
        }
    }

    fn unify(&mut self, a: usize, b: usize) {
        if self.failed {
            return;
        }
        let ra = self.find_compress(a);
        let rb = self.find_compress(b);
        if ra == rb {
            return;
        }

        let ia = self.info[ra].take();
        let ib = self.info[rb].take();

        self.union_sets(ra, rb);
        let rep = self.find(ra);

        match (ia, ib) {
            (None, None) => {}
            (Some(info), None) | (None, Some(info)) => {
                self.info[rep] = Some(info);
            }
            (Some(ia), Some(ib)) => {
                self.merge_info(rep, ia, ib);
            }
        }
    }

    fn merge_info(&mut self, rep: usize, ia: TypeInfo, ib: TypeInfo) {
        match (ia, ib) {
            (TypeInfo::Bool, TypeInfo::Bool) => {
                let r = self.find(rep);
                self.info[r] = Some(TypeInfo::Bool);
            }
            (TypeInfo::I32, TypeInfo::I32) => {
                let r = self.find(rep);
                self.info[r] = Some(TypeInfo::I32);
            }
            (TypeInfo::Ref(a), TypeInfo::Ref(b)) => {
                let r = self.find(rep);
                self.info[r] = Some(TypeInfo::Ref(a));
                self.unify(a, b);
            }
            (TypeInfo::Tuple(mut as_), TypeInfo::Tuple(mut bs)) => {
                while as_.len() < bs.len() {
                    as_.push(self.fresh());
                }
                while bs.len() < as_.len() {
                    bs.push(self.fresh());
                }
                let r = self.find(rep);
                self.info[r] = Some(TypeInfo::Tuple(as_.clone()));
                for i in 0..as_.len() {
                    self.unify(as_[i], bs[i]);
                }
            }
            (TypeInfo::FnPtr(ap, ar), TypeInfo::FnPtr(bp, br)) => {
                if ap.len() != bp.len() {
                    self.failed = true;
                    return;
                }
                let r = self.find(rep);
                self.info[r] = Some(TypeInfo::FnPtr(ap.clone(), ar));
                for i in 0..ap.len() {
                    self.unify(ap[i], bp[i]);
                }
                self.unify(ar, br);
            }
            _ => {
                self.failed = true;
            }
        }
    }

    fn ensure_tuple(&mut self, var: usize, min_len: usize) -> Option<Vec<usize>> {
        if self.failed {
            return None;
        }
        let rep = self.find_compress(var);
        let info = self.info[rep].take();
        match info {
            None => {
                let elems: Vec<usize> = (0..min_len).map(|_| self.fresh()).collect();
                let r = self.find(var);
                self.info[r] = Some(TypeInfo::Tuple(elems.clone()));
                Some(elems)
            }
            Some(TypeInfo::Tuple(mut elems)) => {
                while elems.len() < min_len {
                    elems.push(self.fresh());
                }
                let r = self.find(var);
                self.info[r] = Some(TypeInfo::Tuple(elems.clone()));
                Some(elems)
            }
            Some(other) => {
                let r = self.find(var);
                self.info[r] = Some(other);
                self.failed = true;
                None
            }
        }
    }

    fn resolve(&self, var: usize) -> Option<Type> {
        self.resolve_inner(var, &mut Vec::new())
    }

    fn resolve_inner(&self, var: usize, stack: &mut Vec<usize>) -> Option<Type> {
        let rep = self.find(var);
        if stack.contains(&rep) {
            return None; // cycle
        }
        stack.push(rep);
        let result = match &self.info[rep] {
            None => Some(Type::Var(rep)),
            Some(TypeInfo::Bool) => Some(Type::Bool),
            Some(TypeInfo::I32) => Some(Type::I32),
            Some(TypeInfo::Ref(inner)) => {
                let inner = *inner;
                Some(Type::Ref(Box::new(self.resolve_inner(inner, stack)?)))
            }
            Some(TypeInfo::Tuple(elems)) => {
                let elems = elems.clone();
                let mut types = Vec::new();
                for e in &elems {
                    types.push(self.resolve_inner(*e, stack)?);
                }
                Some(Type::Tuple(types))
            }
            Some(TypeInfo::FnPtr(params, ret)) => {
                let params = params.clone();
                let ret = *ret;
                let mut param_types = Vec::new();
                for p in &params {
                    param_types.push(self.resolve_inner(*p, stack)?);
                }
                let ret_type = self.resolve_inner(ret, stack)?;
                Some(Type::FnPtr(param_types, Box::new(ret_type)))
            }
        };
        stack.pop();
        result
    }
}

// ---------------------------------------------------------------------------
// Phase 1: Collect function signatures
// ---------------------------------------------------------------------------

struct FnCollector<'tcx> {
    tcx: TyCtxt<'tcx>,
    infer: TypeInfer,
    fn_ret_ty: HashMap<LocalDefId, usize>,
    fn_ty: HashMap<LocalDefId, usize>,
    local_ty: HashMap<HirId, usize>,
}

impl<'tcx> intravisit::Visitor<'tcx> for FnCollector<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx Item<'tcx>) -> Self::Result {
        let ItemKind::Fn { body, .. } = item.kind else {
            return;
        };

        let def_id = item.owner_id.def_id;
        let body = self.tcx.hir_body(body);

        let ret_var = self.infer.fresh();
        self.fn_ret_ty.insert(def_id, ret_var);

        let mut param_vars = Vec::new();
        for param in body.params {
            let PatKind::Binding(_, hir_id, _, _) = param.pat.kind else {
                panic!("unsupported")
            };
            let var = self.infer.fresh();
            self.local_ty.insert(hir_id, var);
            param_vars.push(var);
        }

        let fn_var = self.infer.make_fn_ptr(param_vars, ret_var);
        self.fn_ty.insert(def_id, fn_var);
    }
}

// ---------------------------------------------------------------------------
// Phase 2: Constraint generation
// ---------------------------------------------------------------------------

struct ConstraintVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    infer: TypeInfer,
    expr_ty: HashMap<HirId, usize>,
    local_ty: HashMap<HirId, usize>,
    fn_ret_ty: HashMap<LocalDefId, usize>,
    fn_ty: HashMap<LocalDefId, usize>,
}

impl<'tcx> intravisit::Visitor<'tcx> for ConstraintVisitor<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx Item<'tcx>) -> Self::Result {
        let ItemKind::Fn { body, .. } = item.kind else {
            intravisit::walk_item(self, item);
            return;
        };

        // Walk the body (visits all expressions inside)
        intravisit::walk_item(self, item);

        // Unify body expression type with return type
        let def_id = item.owner_id.def_id;
        let ret_var = self.fn_ret_ty[&def_id];
        let body = self.tcx.hir_body(body);
        let body_ty = self.expr_ty[&body.value.hir_id];
        self.infer.unify(body_ty, ret_var);
    }

    fn visit_stmt(&mut self, stmt: &'tcx Stmt<'tcx>) -> Self::Result {
        match stmt.kind {
            StmtKind::Let(l) => {
                let PatKind::Binding(_, hir_id, _, _) = l.pat.kind else {
                    panic!("unsupported")
                };
                let var = self.infer.fresh();
                self.local_ty.insert(hir_id, var);

                intravisit::walk_stmt(self, stmt);

                if let Some(init) = l.init {
                    let init_ty = self.expr_ty[&init.hir_id];
                    self.infer.unify(var, init_ty);
                }
            }
            StmtKind::Expr(_) | StmtKind::Semi(_) => {
                intravisit::walk_stmt(self, stmt);
            }
            _ => panic!("unsupported"),
        }
    }

    fn visit_expr(&mut self, expr: &'tcx Expr<'tcx>) -> Self::Result {
        intravisit::walk_expr(self, expr);

        let ty = self.infer.fresh();
        self.expr_ty.insert(expr.hir_id, ty);

        if self.infer.failed {
            return;
        }
        self.expr_ty.insert(expr.hir_id, ty);

        match expr.kind {
            ExprKind::Call(callee, args) => {
                let callee_ty = self.expr_ty[&callee.hir_id];
                let arg_tys: Vec<usize> =
                    args.iter().map(|a| self.expr_ty[&a.hir_id]).collect();
                let fn_ty = self.infer.make_fn_ptr(arg_tys, ty);
                self.infer.unify(callee_ty, fn_ty);
            }
            ExprKind::Tup(elems) => {
                let elem_tys: Vec<usize> =
                    elems.iter().map(|e| self.expr_ty[&e.hir_id]).collect();
                let tup_ty = self.infer.make_tuple(elem_tys);
                self.infer.unify(ty, tup_ty);
            }
            ExprKind::Binary(op, lhs, rhs) => {
                let lhs_ty = self.expr_ty[&lhs.hir_id];
                let rhs_ty = self.expr_ty[&rhs.hir_id];
                match op.node {
                    BinOpKind::Add
                    | BinOpKind::Sub
                    | BinOpKind::Mul
                    | BinOpKind::Div
                    | BinOpKind::Rem
                    | BinOpKind::BitXor
                    | BinOpKind::BitAnd
                    | BinOpKind::BitOr
                    | BinOpKind::Shl
                    | BinOpKind::Shr => {
                        let i = self.infer.make_i32();
                        self.infer.unify(lhs_ty, i);
                        let i = self.infer.make_i32();
                        self.infer.unify(rhs_ty, i);
                        let i = self.infer.make_i32();
                        self.infer.unify(ty, i);
                    }
                    BinOpKind::And | BinOpKind::Or => {
                        let b = self.infer.make_bool();
                        self.infer.unify(lhs_ty, b);
                        let b = self.infer.make_bool();
                        self.infer.unify(rhs_ty, b);
                        let b = self.infer.make_bool();
                        self.infer.unify(ty, b);
                    }
                    BinOpKind::Eq
                    | BinOpKind::Ne
                    | BinOpKind::Lt
                    | BinOpKind::Le
                    | BinOpKind::Gt
                    | BinOpKind::Ge => {
                        let i = self.infer.make_i32();
                        self.infer.unify(lhs_ty, i);
                        let i = self.infer.make_i32();
                        self.infer.unify(rhs_ty, i);
                        let b = self.infer.make_bool();
                        self.infer.unify(ty, b);
                    }
                }
            }
            ExprKind::Unary(op, operand) => {
                let op_ty = self.expr_ty[&operand.hir_id];
                match op {
                    UnOp::Not => {
                        let b = self.infer.make_bool();
                        self.infer.unify(op_ty, b);
                        let b = self.infer.make_bool();
                        self.infer.unify(ty, b);
                    }
                    UnOp::Neg => {
                        let i = self.infer.make_i32();
                        self.infer.unify(op_ty, i);
                        let i = self.infer.make_i32();
                        self.infer.unify(ty, i);
                    }
                    UnOp::Deref => {
                        let r = self.infer.make_ref(ty);
                        self.infer.unify(op_ty, r);
                    }
                }
            }
            ExprKind::Lit(lit) => match lit.node {
                LitKind::Int(..) => {
                    let i = self.infer.make_i32();
                    self.infer.unify(ty, i);
                }
                LitKind::Bool(_) => {
                    let b = self.infer.make_bool();
                    self.infer.unify(ty, b);
                }
                _ => panic!("unsupported"),
            },
            ExprKind::DropTemps(inner) => {
                let inner_ty = self.expr_ty[&inner.hir_id];
                self.infer.unify(ty, inner_ty);
            }
            ExprKind::If(cond, then_expr, else_opt) => {
                let cond_ty = self.expr_ty[&cond.hir_id];
                let b = self.infer.make_bool();
                self.infer.unify(cond_ty, b);

                let then_ty = self.expr_ty[&then_expr.hir_id];

                match else_opt {
                    Some(else_expr) => {
                        let else_ty = self.expr_ty[&else_expr.hir_id];
                        self.infer.unify(ty, then_ty);
                        self.infer.unify(ty, else_ty);
                    }
                    None => {
                        let unit = self.infer.make_tuple(vec![]);
                        self.infer.unify(ty, unit);
                        let unit = self.infer.make_tuple(vec![]);
                        self.infer.unify(then_ty, unit);
                    }
                }
            }
            ExprKind::Loop(_, _, _, _) => {
                let unit = self.infer.make_tuple(vec![]);
                self.infer.unify(ty, unit);
            }
            ExprKind::Block(block, _) => {
                if let Some(expr) = block.expr {
                    let expr_ty = self.expr_ty[&expr.hir_id];
                    self.infer.unify(ty, expr_ty);
                } else {
                    let unit = self.infer.make_tuple(vec![]);
                    self.infer.unify(ty, unit);
                }
            }
            ExprKind::Assign(lhs, rhs, _) => {
                let lhs_ty = self.expr_ty[&lhs.hir_id];
                let rhs_ty = self.expr_ty[&rhs.hir_id];
                self.infer.unify(lhs_ty, rhs_ty);
                let unit = self.infer.make_tuple(vec![]);
                self.infer.unify(ty, unit);
            }
            ExprKind::AssignOp(_, lhs, rhs) => {
                let lhs_ty = self.expr_ty[&lhs.hir_id];
                let rhs_ty = self.expr_ty[&rhs.hir_id];
                let i = self.infer.make_i32();
                self.infer.unify(lhs_ty, i);
                let i = self.infer.make_i32();
                self.infer.unify(rhs_ty, i);
                let unit = self.infer.make_tuple(vec![]);
                self.infer.unify(ty, unit);
            }
            ExprKind::Field(base, ident) => {
                let index = ident.name.as_str().parse::<usize>().expect("unsupported");
                let base_ty = self.expr_ty[&base.hir_id];
                if let Some(elems) = self.infer.ensure_tuple(base_ty, index + 1) {
                    self.infer.unify(ty, elems[index]);
                }
            }
            ExprKind::Path(QPath::Resolved(_, path)) => match path.res {
                Res::Local(hir_id) => {
                    let local_var = self.local_ty[&hir_id];
                    self.infer.unify(ty, local_var);
                }
                Res::Def(_, def_id) => {
                    let def_id = def_id.expect_local();
                    let fn_var = self.fn_ty[&def_id];
                    self.infer.unify(ty, fn_var);
                }
                _ => panic!("unsupported"),
            },
            ExprKind::AddrOf(_, _, inner) => {
                let inner_ty = self.expr_ty[&inner.hir_id];
                let r = self.infer.make_ref(inner_ty);
                self.infer.unify(ty, r);
            }
            ExprKind::Break(_, e) => {
                assert!(e.is_none(), "unsupported");
                // Divergent expression - type is unconstrained
            }
            ExprKind::Continue(_) => {
                // Divergent expression - type is unconstrained
            }
            ExprKind::Ret(ret_val) => {
                let f = expr.hir_id.owner.def_id;
                let ret_ty = self.fn_ret_ty[&f];
                match ret_val {
                    Some(e) => {
                        let e_ty = self.expr_ty[&e.hir_id];
                        self.infer.unify(ret_ty, e_ty);
                    }
                    None => {
                        let unit = self.infer.make_tuple(vec![]);
                        self.infer.unify(ret_ty, unit);
                    }
                }
                // return expression itself is divergent
            }
            _ => panic!("unsupported"),
        }
    }

    fn visit_block(&mut self, block: &'tcx Block<'tcx>) -> Self::Result {
        intravisit::walk_block(self, block);
    }
}
