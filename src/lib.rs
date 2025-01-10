use std::sync::Arc;
use std::fmt::Debug;
use std::unimplemented;
use std::collections::HashMap;
use mopa::mopafy;

pub struct Environment {
    parent: Option<Arc<Environment>>,
    map: HashMap<String, Arc<dyn Expr>>
}

impl Environment {
    pub fn empty(parent: Option<Arc<Environment>>) -> Self {
        Self {
            parent: parent,
            map: HashMap::new()
        }
    }
    
    pub fn lookup(&self, id: &str) -> Option<&Arc<dyn Expr>> {
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
    pub fn bind(self: Arc<Self>, var: String, value: Arc<dyn Expr>) -> Arc<Environment> {
        let mut e = Environment::empty(Some(self.clone()));
        e.map.insert(var, value);
        Arc::new(e)
    }
}



pub trait Expr : Debug + mopa::Any + 'static {
    fn eval(& self, env: &Environment) -> Arc<dyn Expr>;
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

mopafy!(Expr);

#[derive(Debug, Clone)]
pub struct Identifier {
    inner: String
}

impl Identifier {
    pub fn new(s: String) -> Arc<Self> {
        Arc::new(Identifier { inner: s })
    }
}

impl Expr for Identifier {
    fn eval(&self, env: &Environment) -> Arc<dyn Expr> {
        unimplemented!();
    }

    fn display(&self) -> String {
        format!("{}", self.inner)
    }
}


#[derive(Debug)]
pub struct Lambda<B:Expr> {
    params: Arc<Identifier>,
    body: Arc<B>,
}

impl<B:Expr> Lambda<B> {
    pub fn lambda(a: Arc<Identifier>, b: Arc<B>) -> Lambda<B> {
        Lambda { params: a,
                 body: b
        }
    }

    /// Apply the lambda with bindings to the value
    pub fn apply(&self, value: Arc<dyn Expr>, env: Arc<Environment>) -> Arc<dyn Expr> {
        let e = env.bind(self.params.inner.clone(), value.clone());
        self.body.eval(&e)
    }
}

impl<B:Expr> Expr for Lambda<B> {
    fn display(&self) -> String {
        format!("λ {} -> {}", self.params.display(), self.body.display())
    }

    fn eval(&self, env: &Environment) -> Arc<dyn Expr> {
        unimplemented!{}
    }
}


/// The empty list, a.k.a ()
#[derive(Debug)]
pub struct Empty {
}

impl Empty {
    pub fn empty() -> Arc<Empty> {
        Arc::new(Empty{})
    }
}

impl Expr for Empty {
    fn eval(& self, _: &Environment) -> Arc<dyn Expr> {
        Arc::new(Empty {})
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
    pub fn new(n: i64) -> Arc<Self> {
        Arc::new(Number { inner: n })
    }
}

impl Expr for Number {
    fn eval(&self, _: &Environment) -> Arc<dyn Expr> {
        Arc::new(Number{inner: self.inner})
    }

    fn display(& self) -> String {
        format!("{}", self.inner)
    }
}

/// More or less anything
#[derive(Debug)]
pub struct Cons<A:Expr, B:Expr> {
    car: Arc<A>,
    cdr: Arc<B>
}

impl<A:Expr, B:Expr> Cons<A, B> {
    pub fn cons(a: Arc<A>, b: Arc<B>) -> Arc<Cons<A,B>> {
        Arc::new(Cons {car: a.clone(),
              cdr: b.clone()})
    }


}

impl<A:Expr, B:Expr> Expr for Cons<A,B> {
    fn eval(&self, env: &Environment) -> Arc<dyn Expr> {
        Identifier::new(self.display())
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
        let e = Arc::new(Empty{});
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
