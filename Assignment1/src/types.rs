/// Types used by the type analysis and transformation pipeline.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Type {
    /// The boolean type `bool`.
    Bool,
    /// The 32-bit signed integer type `i32`.
    I32,
    /// A shared reference type `&T`.
    Ref(Box<Type>),
    /// A tuple type `(T1, ..., Tn)`.
    Tuple(Vec<Type>),
    /// A function pointer type `fn(T1, ..., Tn) -> R`.
    FnPtr(Vec<Type>, Box<Type>),
    /// A type variable identified by an index.
    Var(usize),
    /// An absent field
    Absent,
}

impl Type {
    /// Builds a shared reference type.
    #[inline]
    pub fn ref_(t: Type) -> Self {
        Type::Ref(Box::new(t))
    }

    /// Builds a tuple type from two component types.
    #[inline]
    pub fn tuple(ts: Vec<Type>) -> Self {
        Type::Tuple(ts)
    }

    /// Builds a function pointer type from parameter and return types.
    #[inline]
    pub fn fn_ptr(params: Vec<Type>, ret: Type) -> Self {
        Type::FnPtr(params, Box::new(ret))
    }

    /// Returns the variable id if this is `Type::Var`.
    #[inline]
    pub fn var_id(&self) -> Option<usize> {
        match self {
            Type::Var(i) => Some(*i),
            _ => None,
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::Bool => write!(f, "bool"),
            Type::I32 => write!(f, "i32"),
            Type::Ref(t) => write!(f, "&{t}"),
            Type::Tuple(ts) => {
                write!(f, "(")?;
                let mut len = 0;
                for t in ts {
                    if t == &Type::Absent {
                        continue;
                    }
                    if len > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{t}")?;
                    len += 1;
                }
                if len == 1 {
                    write!(f, ",")?;
                }
                write!(f, ")")
            }
            Type::FnPtr(ps, r) => {
                write!(f, "fn(")?;
                for (i, p) in ps.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{p}")?;
                }
                write!(f, ") -> {r}")
            }
            Type::Var(i) => write!(f, "TYPEVAR{i}"),
            Type::Absent => write!(f, "ABSENT"),
        }
    }
}
