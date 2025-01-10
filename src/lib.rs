use std::boxed::Box;
use std::fmt::Debug;
use std::unimplemented;
use std::collections::HashMap;

pub struct Environment {
    parent: Option<Box<Environment>>,
    map: HashMap<String, Box<dyn Expr>>
}

impl Environment {
    pub fn empty() -> Self {
        Self {
            parent: None,
            map: HashMap::new()
        }
    }
    
    pub fn lookup(&self, id: &str) -> Option<&Box<dyn Expr>> {
        match self.map.get(id) {
            Some(v) => Some(v),
            None => {
                if let Some(env) = &self.parent {
                    env.lookup(id)
                } else {
                    None
                }
            }
        }
    }

    /// Bind new variables, creating a new environment
    pub fn bind(self, var: String, value: Box<dyn Expr>) -> Environment {
        let mut e = Environment { parent : Some(Box::new(self)),
                                  map: HashMap::new() };
        e.map.insert(var, value);
        e
    }
}



pub trait Expr : Debug {
    fn eval(& self, env: &Environment) -> Box<dyn Expr>;
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

#[derive(Debug, Clone)]
pub struct Identifier {
    inner: String
}

impl Identifier {
    pub fn new(s: String) -> Self {
        Identifier { inner: s }
    }
}

impl Expr for Identifier {
    fn eval(&self, env: &Environment) -> Box<dyn Expr> {
        unimplemented!();
    }

    fn display(&self) -> String {
        format!("{}", self.inner)
    }
}


#[derive(Debug)]
pub struct Lambda<A:Expr, B:Expr> {
    params: A,
    body: B
}

impl<A:Expr, B:Expr> Lambda<A, B> {
    pub fn lambda(a: A, b: B) -> Lambda<A, B> {
        Lambda { params: a,
                 body: b
        }
    }
}

impl<A:Expr, B:Expr> Expr for Lambda<A, B> {
    fn display(&self) -> String {
        format!("λ {} -> {}", self.params.display(), self.body.display())
    }

    fn eval(&self, env: &Environment) -> Box<dyn Expr> {
        unimplemented!{}
    }
}


/// The empty list, a.k.a ()
#[derive(Debug)]
pub struct Empty {
}

impl Expr for Empty {
    fn eval(& self, _: &Environment) -> Box<dyn Expr> {
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
    fn eval(&self, _: &Environment) -> Box<dyn Expr> {
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
    fn eval(&self, env: &Environment) -> Box<dyn Expr> {
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
