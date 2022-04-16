// attr位于作用目标内部，比如doc是针对crate生效，位于crate内，使用`#![...]`
#![doc = "this is docs..."]
fn main() {
    sholud_use(); // 方法有`must_use`属性，如果返回值没有被使用会产生warning
}

// attr位于作用目标外，这里对其后面的函数生效，使用`#[...]`
#[must_use = "the return value shoud used"]
fn sholud_use() -> i32 {
    return 0;
}
