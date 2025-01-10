use std::boxed::Box;
use std::fmt::Debug;
use std::unimplemented;

pub trait Expr : Debug {
    fn eval(& self) -> Box<dyn Expr>;
    fn display(& self) -> String;
    fn is_empty(&self) -> bool {
        false
    }
    fn is_list(&self) -> bool {
        false
    }
    fn is_tuple(&self) -> bool {
        false
    }
}


/// The empty list, a.k.a ()
#[derive(Debug)]
pub struct Empty {
}

impl Expr for Empty {
    fn eval(& self) -> Box<dyn Expr> {
        Box::new(Empty {})
    }

    fn display(& self) -> String {
        format!("()")
    }

    fn is_empty(&self) -> bool {
        true
    }

    fn is_list(&self) -> bool {
        true
    }
}



/// Number
#[derive(Debug)]
pub struct Number {
    inner: i64
}

impl Number {
    pub fn new(n: i64) -> Self {
        Number { inner: n }
    }
}

impl Expr for Number {
    fn eval(&self) -> Box<dyn Expr> {
        Box::new(Number{inner: self.inner})
    }

    fn display(& self) -> String {
        format!("{}", self.inner)
    }
}

// pub fn cons<A:Expr, B:List>(a:A, b: B) -> List<A, B> {
//     List::Pair(a, b)
// }

/// More or less anything
#[derive(Debug)]
pub struct Cons<A:Expr, B:Expr> {
    car: A,
    cdr: B
}

impl<A:Expr, B:Expr> Cons<A, B> {
    pub fn cons(a: A, b: B) -> Cons<A,B> {
        Cons {car: a,
              cdr: b}
    }


}

impl<A:Expr, B:Expr> Expr for Cons<A,B> {
    fn eval(&self) -> Box<dyn Expr> {
        unimplemented!();
    }

    fn display(&self) -> String {
        if !self.cdr.is_empty() {
            format!("{}, {}", self.car.display(), self.cdr.display())
        } else {
            format!("{})", self.car.display())
        }
    }

    fn is_list(&self) -> bool {
        if self.cdr.is_empty() {
            true
        } else {
            self.cdr.is_list()
        }
    }

    fn is_tuple(&self) -> bool {
        !self.is_list()
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let e = Box::new(Empty{});
        assert!(e.is_list());
    }

    #[test]
    fn list() {
        let a = Cons::cons(Number::new(1), Cons::cons(Number::new(2), Empty{}));
        assert!(a.is_list());
        assert!(!a.is_tuple());
        let b = Cons::cons(Number::new(1), Cons::cons(Number::new(2), Number::new(3)));
        assert!(!b.is_list());
        assert!(b.is_tuple());
    }
}
