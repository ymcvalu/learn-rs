/// A closure expression produces a closure value with a unique,
/// anonymous type that cannot be written out.
/// A closure type is approximately equivalent to a struct which contains the captured variables.
/// `trait Fn: FnMut`: capture immutable refrences only;
/// `trait FnMut: FnOnce`: capture some mutable refrences;
/// `trait FnOnce`: capture moved value, can only be called once;
/// 闭包捕获的变量，都储存在栈上，没有堆内存分配；
/// 因为闭包在创建时会隐式地创建自己的类型，每个闭包都是一个新的类型；
/// 通过闭包自己唯一的类型，Rust 不需要额外的函数指针来运行闭包，所以闭包的调用效率和函数调用几乎一致；
fn main() {
    let mut str: String = "hello, lambda".into();

    let c1 = || println!("in c1: {:?}", str); // c1 capture immut refrence of str
    println!("after c1: {:?}", str);
    // str.push_str("!!!"); // can't has mut refrence now
    c1(); // Fn::call

    str.push_str("!");
    let mut c2 = || str.push_str("!"); // c2 capture mut refrence of str
    c2(); // FnMut::call_mut

    println!("after c2: {:?}", str);
    let mut c3 = move || {
        str.push('!');
        println!("in c3: {:?}", str);
    };
    c3(); // FnOnce::call_once

    // println!("after c2: {:?}", str); // str has been moved

    // clusure as params
    let v = apply1(1, |v| v + 1);
    let v = apply2(v, |v| v * v);
    println!("{}", v);

    let mut s = Into::<String>::into("hello");
    let mut cc1 = || println!("in cc1: {:?}", s);
    call(&cc1);
    // trait Fn: FnMut
    call_mut(&mut cc1);
    // trait FnMut: FnOnce
    call_once(cc1); // cc1 impl `Copy` trait
    cc1(); // impl Copy trait

    let mut cc2 = || {
        s.push_str(" rust");
        println!("in cc2: {:?}", s);
    };

    call_mut(&mut cc2);
    // trait FnMut: FnOnce
    call_once(cc2); // cc2 has been moved

    let cc3 = move || println!("in cc3: {:?}", s);
    call_once(cc3);

    let max = |(a, b)| if a > b { a } else { b };
    println!("max(1, 2) = {}", max.invoke((1, 2)));
}

fn apply1<T, F, O>(t: T, f: F) -> O
where
    T: Sized,
    F: Fn(T) -> O, // lambda type
    O: Sized,
{
    f(t)
}

/// `Fn(T) -> O` is meaning `Fn<(T), Output=O>`
fn apply2<T, O>(t: T, f: impl Fn(T) -> O) -> O {
    f(t)
}

fn call(f: &impl Fn()) {
    f()
}

fn call_mut(f: &mut impl FnMut()) {
    f()
}

fn call_once(f: impl FnOnce()) {
    f()
}

trait Invoker<K, V> {
    fn invoke(&self, k: K) -> V;
}

impl<T, K, V> Invoker<K, V> for T
where
    T: Fn(K) -> V,
{
    fn invoke(&self, k: K) -> V {
        self(k)
    }
}
