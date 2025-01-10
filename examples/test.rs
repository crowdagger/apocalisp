use apocalisp::Empty;
use apocalisp::Expr;
use apocalisp::Cons;
use apocalisp::Number;

use std::boxed::Box;


fn main() {
    let e = Box::new(Empty{});
    println!("{:?}", e.eval());

    let b = Cons::cons(Number::new(1), Cons::cons(Number::new(2), Empty{}));
    println!("{:?} {:?}", b.display(), b.is_list());
}
