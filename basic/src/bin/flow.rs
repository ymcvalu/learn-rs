fn main() {
    println!("10th fib is {}", fib_loop(10));
    println!("10th fib is {}", fib_while(10));
    println!("10th fib is {}", fib_for(10));
}

fn fib_loop(n: u8) -> i32 {
    let mut a = 1;
    let mut b = 1;
    let mut i = 2u8;
    // loop是一个表达式
    loop {
        let c = a + b;
        a = b;
        b = c;
        i += 1;

        if i >= n {
            break b;
        }
    }
}

fn fib_while(n: u8) -> i32 {
    let mut i = 2u8;
    let mut a = 1;
    let mut b = 1;
    while i < n {
        let c = a + b;
        a = b;
        b = c;
        i += 1;
    }
    b
}

fn fib_for(n: u8) -> i32 {
    let mut a = 1;
    let mut b = 1;
    // for循环可以用于任何实现了 IntoIterator trait 的数据结构；
    // 在执行过程中，IntoIterator 会生成一个迭代器，
    // for 循环不断从迭代器中取值，直到迭代器返回 None 为止。
    // 因而，for 循环实际上只是一个语法糖，编译器会将其展开使用 loop 循环对迭代器进行循环访问，直至返回 None
    for _ in 2..n {
        let c = a + b;
        a = b;
        b = c;
    }
    b
}
