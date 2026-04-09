use std::collections::HashMap;

use rustc_middle::mir::Local;

/// Interval abstract domain for `i32` values.
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Interval {
    /// A closed interval with optional infinite bounds.
    Range(Option<i32>, Option<i32>),
    /// The empty interval.
    Bot,
}

impl std::fmt::Debug for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Interval::Range(Some(l), Some(u)) => write!(f, "[{l}, {u}]"),
            Interval::Range(Some(l), None) => write!(f, "[{l}, ∞]"),
            Interval::Range(None, Some(u)) => write!(f, "[-∞, {u}]"),
            Interval::Range(None, None) => write!(f, "[-∞, ∞]"),
            Interval::Bot => write!(f, "⊥"),
        }
    }
}

/// Abstract state mapping MIR locals to intervals.
#[derive(Clone)]
pub struct AbsState(pub HashMap<Local, Interval>);

impl std::fmt::Debug for AbsState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut entries = self.0.iter().collect::<Vec<_>>();
        entries.sort_by_key(|(local, _)| *local);
        f.debug_map().entries(entries).finish()
    }
}
