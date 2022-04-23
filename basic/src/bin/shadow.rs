fn main() {
    let s = MyString {
        v: "shadow demo".into(),
    };
    let s = &s;
    println!("value is {:?}", s);

    // `s: MyString` drop here
}

#[derive(Debug)]
struct MyString {
    v: String,
}

impl Drop for MyString {
    fn drop(&mut self) {
        println!("MyString drop: {:?}", self);
    }
}
