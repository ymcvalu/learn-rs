# borrow qa

## reborrow

```rust
#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn move_to(&mut self, x: i32, y: i32) {
        self.x = x;
        self.y = y;
    }
}

fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r; // 基于可变引用构造不可变引用

    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}
```

在上面代码中，通过可变引用构造了一个不可变引用，这时候同时存在一个可变引用和一个不可变引用，代码是可以正常编译执行的，但是下面代码却编译失败：

```rust
fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &p; // 直接从p借用不可变引用

    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}
```

关于第一段代码，`&*r`在Rust中被称为`reborrow`， 实际上你可以将`let rr: &Point = &*r` 改成`let rr: &Point = &r`，因为编译器会自动修改将`&r`修改为`&*r`。

对于`reborrow`而言，`rr`在借用时不会破坏原先`r`对`p`的借用规则，但是需要对`r`做借用检查：

```rust
fn main() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r; // 基于可变引用构造不可变引用
    let mrr: &mut Point = &mut *r; // error: 已经有immut ref，不允许同时存在mut ref
    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}

failed: 
 |     let rr: &Point = &*r; // 基于可变引用构造不可变引用
 |                      --- immutable borrow occurs here
 |     let mrr: &mut Point = &mut *r; 
 |                           ^^^^^^^ mutable borrow occurs here
 |     println!("{:?}", rr);
 |                      -- immutable borrow later used here
```

对于`reborrow`的生命周期：

```rust
let x: &'x i32 = ...;
let y: &'y i32 = &*x;
```

In such cases, there is a connection between the lifetime 'y of the borrow and the lifetime 'x of the original reference. In particular, 'x must outlive 'y ('x: 'y).

### implicit reborrow

在某些场景下，rust会自动隐式`reborrow`，比如下面例子：

```rust
fn main() {
    let mut s = String::from("hello");

    let mr = &mut s;

    push_str(mr, " rust!"); // implicit reborrow
    // push_str(&mut *mr, " rust!");
    println!("{}", mr);
}

fn push_str(s: &mut String, v: &str) {
    s.push_str(v);
}
```

目前关于`reborrow`的[文档比较少](https://github.com/rust-lang/reference/issues/788)，也可以参考`rfc nll`中的[reborrow-constraints](https://rust-lang.github.io/rfcs/2094-nll.html#reborrow-constraints)一节
