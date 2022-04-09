fn main() {
    let s: String = "str".into();
    let r = Ref { v: &s };
}

struct Ref<'a, T: 'a> {
    v: &'a T,
}
