use std::fmt::Debug;

// Trait object is `DST`, and use `dyn` keyword to indicate it's a trait object;
// keywords: dynamic dispatch, fat pointer, ptr and vtable;
fn main() {
    let b = Bean {
        str: "trait object, dynamic dispatch".into(),
    };

    print1(&b);
    print2(Box::new(b));

    let b = Bean {
        str: "static dispatch".into(),
    };

    print4(&b);
    print3(b);
}

#[derive(Debug)]
struct Bean {
    str: String,
}

// trait object, dynamic dispatch
fn print1(obj: &dyn Debug) {
    println!("obj: {:?}", obj);
}

// trait object, dynamic dispatch
fn print2(obj: Box<dyn Debug>) {
    println!("obj: {:?}", obj);
}

// static dispatch
fn print3<T>(obj: T)
where
    T: Echo,
{
    obj.echo();
}

// static dispatch
fn print4(obj: impl Echo) {
    obj.echo();
}

trait Echo
where
    Self: Debug,
{
    fn echo(&self);
}

impl Echo for Bean {
    fn echo(&self) {
        // <&Self as Echo>::echo(&self); // 这里`self`会被认为是`Bean`类型，需要使用&self
        <&Self>::echo(&self); // 这里`self`会被认为是`Bean`类型，需要使用&self
    }
}

impl Echo for &Bean {
    fn echo(&self) {
        println!("echo: {:?}", self);
    }
}
