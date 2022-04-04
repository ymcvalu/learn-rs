fn main() {
    let s = "hello world".to_string();

    println!(
        "addr of ss: {:p}, s: {:p}, len: {}, capacity: {}, size: {}",
        &"hello world",
        &s,
        s.len(),
        s.capacity(),
        std::mem::size_of_val(&s)
    );
}
