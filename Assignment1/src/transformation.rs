use std::collections::{HashMap, HashSet};

use rustc_ast::{
    AttrKind, Crate, FnRetTy, Item, ItemKind, Local, PatKind, Ty, mut_visit,
    mut_visit::MutVisitor as _,
};
use rustc_ast_pretty::pprust;
use rustc_hir as hir;
use rustc_hir::{HirId, def_id::LocalDefId, intravisit};
use rustc_middle::{hir::nested_filter, ty::TyCtxt};
use rustc_span::{Span, sym};

use crate::{analysis, types::Type, utils};

pub fn transform(tcx: TyCtxt<'_>) -> Option<String> {
    let mut krate = expanded_ast(tcx);
    let result = analysis::analyze(tcx)?;
    remove_unnecessary_items_from_ast(&mut krate);
    let mut hir_visitor = HirVisitor {
        tcx,
        functions: HashMap::new(),
        locals: HashMap::new(),
    };
    tcx.hir_visit_all_item_likes_in_crate(&mut hir_visitor);
    let mut visitor = AstVisitor {
        result,
        functions: hir_visitor.functions,
        locals: hir_visitor.locals,
        vars: HashSet::new(),
    };
    visitor.visit_crate(&mut krate);
    for v in visitor.vars {
        let item = utils::parse_item(format!("#[derive(Clone, Copy)] struct TYPEVAR{v};"));
        krate.items.push(Box::new(item));
    }
    Some(pprust::crate_to_string_for_macros(&krate))
}

fn expanded_ast(tcx: TyCtxt<'_>) -> Crate {
    tcx.resolver_for_lowering().borrow().1.as_ref().clone()
}

fn remove_unnecessary_items_from_ast(krate: &mut Crate) {
    krate.items.retain(|item| match item.kind {
        ItemKind::ExternCrate(_, ident) => ident.name != sym::std,
        ItemKind::Use(_) => !item.attrs.iter().any(|attr| {
            let AttrKind::Normal(attr) = &attr.kind else {
                return false;
            };
            attr.item.path.segments.last().unwrap().ident.name == sym::prelude_import
        }),
        _ => true,
    });
}

struct AstVisitor {
    result: analysis::AnalysisResult,
    functions: HashMap<Span, LocalDefId>,
    locals: HashMap<Span, HirId>,
    vars: HashSet<usize>,
}

impl mut_visit::MutVisitor for AstVisitor {
    fn visit_item(&mut self, item: &mut Item) -> Self::Result {
        mut_visit::walk_item(self, item);

        let ItemKind::Fn(f) = &mut item.kind else {
            return;
        };
        let def_id = self.functions[&f.ident.span];
        let ty = &self.result.fn_rets[&def_id];
        ty.find_vars(&mut self.vars);
        let ty = ty.to_ty();
        f.sig.decl.output = FnRetTy::Ty(Box::new(ty));
        for param in &mut f.sig.decl.inputs {
            let PatKind::Ident(_, ident, _) = param.pat.kind else { panic!() };
            let hir_id = self.locals[&ident.span];
            let ty = &self.result.locals[&hir_id];
            ty.find_vars(&mut self.vars);
            let ty = ty.to_ty();
            param.ty = Box::new(ty);
        }
    }

    fn visit_local(&mut self, local: &mut Local) -> Self::Result {
        mut_visit::walk_local(self, local);

        let PatKind::Ident(_, ident, _) = local.pat.kind else { panic!() };
        let hir_id = self.locals[&ident.span];
        let ty = &self.result.locals[&hir_id];
        ty.find_vars(&mut self.vars);
        let ty = ty.to_ty();
        local.ty = Some(Box::new(ty));
    }
}

struct HirVisitor<'tcx> {
    tcx: TyCtxt<'tcx>,
    functions: HashMap<Span, LocalDefId>,
    locals: HashMap<Span, HirId>,
}

impl<'tcx> intravisit::Visitor<'tcx> for HirVisitor<'tcx> {
    type NestedFilter = nested_filter::OnlyBodies;

    fn maybe_tcx(&mut self) -> Self::MaybeTyCtxt {
        self.tcx
    }

    fn visit_item(&mut self, item: &'tcx hir::Item<'tcx>) -> Self::Result {
        intravisit::walk_item(self, item);

        let hir::ItemKind::Fn { ident, .. } = item.kind else {
            return;
        };
        self.functions.insert(ident.span, item.owner_id.def_id);
    }

    fn visit_pat(&mut self, pat: &'tcx hir::Pat<'tcx>) -> Self::Result {
        intravisit::walk_pat(self, pat);

        let hir::PatKind::Binding(_, hir_id, ident, _) = pat.kind else { panic!() };
        self.locals.insert(ident.span, hir_id);
    }
}

impl Type {
    fn find_vars(&self, vars: &mut HashSet<usize>) {
        match self {
            Type::Bool | Type::I32 | Type::Absent => {}
            Type::Ref(t) => t.find_vars(vars),
            Type::Tuple(ts) => {
                for t in ts {
                    t.find_vars(vars);
                }
            }
            Type::FnPtr(params, ret) => {
                for param in params {
                    param.find_vars(vars);
                }
                ret.find_vars(vars);
            }
            Type::Var(i) => {
                vars.insert(*i);
            }
        }
    }

    #[inline]
    fn to_ty(&self) -> Ty {
        utils::parse_ty(self.to_string())
    }
}
