use std::fmt::{Debug, Display, Error, Formatter};

#[derive(Debug)]
pub enum Expr {
    IdentifierAtom(String),
    NatAtom(u32),
    List(Vec<Box<Expr>>),
}

impl Display for Expr {
    fn fmt(&self, fmt: &mut Formatter) -> Result<(), Error> {
        match &*self {
            Expr::IdentifierAtom(s) => write!(fmt, "{}", s),
            Expr::NatAtom(n) => write!(fmt, "{}", n),
            Expr::List(l) => {
                let iter = &mut l.iter();
                iter.next();
                write!(fmt, "({}", &l[0])?;
                for expr in iter {
                    write!(fmt, " {}", *expr)?;
                }
                write!(fmt, ")")?;
                Ok(())
            }
        }
    }
}
