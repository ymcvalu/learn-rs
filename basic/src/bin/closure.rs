/// A closure expression produces a closure value with a unique,
/// anonymous type that cannot be written out.
/// A closure type is approximately equivalent to a struct which contains the captured variables.
/// Fn: capture immutable refrences only;
/// FnMut: capture some mutable refrences;
/// FnOnce: capture moved value, can only be called once;
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
}
