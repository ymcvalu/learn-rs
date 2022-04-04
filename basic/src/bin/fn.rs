fn main() {
    let mut str = String::from("hello");
    apply(&mut str, |v| {
        v.push_str(" world");
        v
    });
    println!("str = '{}'", str);

    println!("square(5)= {}", square(5));
    println!("cube(5) = {}", cube(5));
}

fn apply<T>(v: T, f: fn(i: T) -> T) -> T {
    f(v) // 最后一个表达式的值为整个block的值
}

fn square(v: i32) -> i32 {
    let s = v * v; // stmt，隐含值为unit
    return s; // 使用return语句返回
}

fn cube(v: i32) -> i32 {
    v * v * v
}
