# borrow

## partial move

下面例子中，我们先`move`了`Person`中某个`field`，那么这个结构体的值就不完整了，这时候无法再使用该`field`，也**无法获取其引用**否则会触发[E0382错误](https://doc.rust-lang.org/error-index.html#E0382)

```rust
#[derive(Debug)]
struct Person {
    name: String,
    address: String,
}
fn main() {
    let p = Person {
        name: "jim".into(),
        address: "上海".into(),
    };

|   let address = p.address;
|                 --------- value partially moved here
| 
|   println!("{:?}", p.address);
|                    ^^^^^^^^^ value borrowed here after move
|
|   let pr = &p;
|            ^^ value borrowed here after partial move
|
}
```

我们可以在move之后，重新给这个字段bind上某个值：

```rust
fn main() {
    let mut p = Person {
        name: "jim".into(),
        address: "上海".into(),
    };

    let addr = p.address;

    p.address = "北京".into();

    let pr = &p;
    println!("old addr is {}, new is {:?}", addr, pr.address);
}
```


同样的，我们无法通过借用`move`，看下面例子：

```rust
fn main() {
    let mut p = Person {
        name: "jim".into(),
        address: "上海".into(),
    };

    let pr = &mut p;

|
|   let address = pr.address;
|                   ^^^^^^^^^^
|                   move occurs because `pr.address` has type `String`, which does not implement the `Copy` trait
|                   help: consider borrowing here: `&pr.address`}
```

当尝试从可变借用中`move`某个字段时，会触发[error[E0507]](https://doc.rust-lang.org/error-index.html#E0507)错误；

**`move`会导致借用不完整，引用到不安全的内存上**

大多是情况，我们可以通过`std::mem::replace`方法来解决：

```rust
fn main() {
    let mut p = Person {
        name: "jim".into(),
        address: "上海".into(),
    };

    let pr = &mut p;
    let old_addr = std::mem::replace(&mut pr.address, "北京".into());
    println!("old addr is {}, new is {:?}", old_addr, pr.address);
}
```

`replace`会使用新值替换指定内存，并返回旧值
