#[derive(Debug)]
struct Person {
    name: String,
    address: String,
}

fn main() {
    let p1 = Person {
        name: "小明".into(),
        address: "中国".into(),
    };

    let p2 = Person {
        name: "小红".into(),
        ..p1 // 其他字段从p1获取，p1.address is partial moved
    };

    // Error: p1.address has been partial moved
    // println!("{:?}", p1);
    println!("{:?}", p2);

    // pattern match, use `..` to ignore other fields
    let Person { name, .. } = p2;
    // Error: p2.name has been moved
    // println!("{:?}", p2);
    println!("name: {}", name);
}
