use apocalisp::Empty;
use apocalisp::Expr;
use apocalisp::Cons;
use apocalisp::Number;
use apocalisp::Identifier;
use apocalisp::Lambda;
use apocalisp::Environment;

use std::boxed::Box;


fn main() {
    let env = Environment::empty();
    let e = Box::new(Empty{});
    println!("{:?}", e.eval(&env));

    let b = Cons::cons(Number::new(1), Cons::cons(Number::new(2), Empty{}));
    println!("{:?} {:?}", b.display(), b.is_list());

    let x = Identifier::new(String::from("x"));
    let body = Cons::cons(Identifier::new(String::from("+")), Cons::cons(Identifier::new(String::from("x")), Cons::cons(Number::new(1), Empty{})));
    println!("{:?} {:?}", body.display(), body.is_list());

    let l = Lambda::lambda(x, body);
    println!("{:?} {:?}", l.display(), l.is_list());

    let env = env.bind(String::from("x"), Box::new(Number::new(1)));
    let t = env.lookup("x").unwrap();
    println!("{:?}", t.display());

    let r = l.apply(Box::new(Number::new(2)), env);
    println!("{:?}", r.display());
}
