/// If `T` implements `Deref<Target = U>`, and `x` is a value of type `T`, then:
/// * In immutable contexts, `*x` (where `T` is neither a reference nor a raw pointer)
///   is equivalent to `*Deref::deref(&x)`.
/// * Values of type `&T` are coerced to values of type `&U`
/// * `T` implicitly implements all the (immutable) methods of the type `U`.
fn main() {
    let ms = MyString {
        str: "hello rust!".into(),
    };

    // `MyString` implicitly implements all the immutable methods of `String`
    println!("{:?}", ms.as_bytes());
    // `String.into`方法会转移所有权，因此`MyString`并不会实现该方法，需要手动强转类型
    let str: String = (&ms as &str).into();

    print_mystr(&ms);

    let r = &ms;
    // `&s1`可以转为类型为`&str`的值
    let _: &str = &ms;
    let _ = &ms as &str;
    let _ = &&&&ms as &str;

    // `*ms`等价于`*(ms.deref())`
    let _: &str = &*ms;
}

fn print_mystr(str: &str) {
    println!("before print...");
    println!("{}", str.deref());
    println!("after print...");
}

#[derive(Debug)]
struct MyString {
    str: String,
}

use std::ops::Deref;
impl Deref for MyString {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        println!("deref {:?}", self);
        self.str.deref()
    }
}
