/// 在定义和使用 trait 时，我们需要遵循孤儿规则（Orphan Rule）：
/// > **`trait`和实现`trait`的数据类型，至少有一个是在当前`crate`中定义的**，
/// 也就是说，你不能为第三方的类型实现第三方的`trait`
fn main() {
    let api = api;
    api.batch_get(vec![1, 2, 3]);

    // Fully Qualified Syntax for Disambiguation: Calling Methods with the Same Name
    // api 有 Api<i32, i32> 实现和 Api<i64, i64> 实现，
    // 编译器无法推断需要执行哪个实现的方法，需要使用`<api as Api<i32, i32>>`显示指定
    <api as Api<i64, i64>>::batch_do();
    Api::<i64, i64>::batch_get(&api, vec![1, 2, 3]);

    if let Result::<String, ()>::Ok(s) = 123i32.encode() {
        println!("123 encode is {}", s);
    }

    let c1 = Complex::new(1f64, 2f64);
    let c2 = Complex::new(2f64, 2f64);
    let sum = &c1 + &c2;
    println!("{:?} + {:?} = {:?}", c1, c2, sum);

    println!("{:?} + 3.0 = {:?}", c1, &c1 + 3f64);
}

trait Api<P, T: Default> {
    // `trait` 的方法可以有缺省实现
    fn get(&self, _: P) -> T {
        println!("get...");
        T::default()
    }

    fn batch_get(&self, ps: Vec<P>) -> Vec<T> {
        let mut rs = Vec::with_capacity(ps.len());
        for p in ps.into_iter() {
            // 如果实现方重载了`get`方法，那么将会执行具体的实现，而不是默认实现
            let r = self.get(p);
            rs.push(r);
        }
        rs
    }

    fn do_something() {
        println!("do...")
    }
    fn batch_do() {
        for _ in 0..3 {
            // 如果实现方重载了`do_something`方法，那么将会执行具体的实现，而不是默认实现
            Self::do_something();
        }
    }
}

struct api;

impl Api<i32, i32> for api {
    fn get(&self, p: i32) -> i32 {
        println!("api<i32> get...");
        p + 1
    }

    fn do_something() {
        println!("api<i32> do...")
    }
}

impl Api<i64, i64> for api {
    fn get(&self, p: i64) -> i64 {
        println!("api<i64> get...");
        p + 1
    }

    fn do_something() {
        println!("api<i64> do...")
    }
}

/// 有时候一些类型在`trait`定义时并不确定，`trait`的定义者希望能够让实现者来指定，这时候可以使用关联类型；
/// 带有关联类型的`trait`比普通`trait`，更加灵活，抽象度更高;
/// 关联类型和泛型参数很像，但是有一些区别：
/// > 一个类型只能实现同一个`trait`一次，针对某个类型的具体实现，关联类型就只能唯一指定；
/// 而如果使用泛型，比如`Formater<O, E>`，相当于定义了无数个具体的`trait`，就可以针对某个类型提供无数种实现，
/// 比如实现`Formater<String, String>`，实现`Formater<String, ()>`，...
trait Formater {
    type Output: std::fmt::Display; // 关联类型，Output需要实现Display这个trait
    type Error; // 关联类型

    fn format(&self) -> Result<Self::Output, Self::Error>;
}

/// 使用关联类型，针对`i32`类型，只能实现一次`trait`，关联类型是唯一指定的
impl Formater for i32 {
    type Output = String;
    type Error = String;
    fn format(&self) -> Result<Self::Output, Self::Error> {
        Ok(format!("{}", self))
    }
}

impl Formater for i64 {
    type Output = String;
    type Error = String;
    fn format(&self) -> Result<Self::Output, Self::Error> {
        Ok(format!("{}", self))
    }
}

/// 使用泛型，就可以给`i32`类型提供多个不同的实现
trait Encoder<O, E> {
    fn encode(&self) -> Result<O, E>;
}

impl Encoder<String, ()> for i32 {
    fn encode(&self) -> Result<String, ()> {
        Ok(format!("'{}'", self))
    }
}

impl Encoder<i64, ()> for i32 {
    fn encode(&self) -> Result<i64, ()> {
        Ok(*self as i64)
    }
}

#[derive(Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imagine: f64,
}

impl Complex {
    fn new(real: f64, imagine: f64) -> Self {
        Complex { real, imagine }
    }
}

use std::{fmt::Debug, iter::Copied, ops::Add};
impl Add for &Complex {
    type Output = Complex;

    fn add(self, rhs: Self) -> Self::Output {
        Complex::new(self.real + rhs.real, self.imagine + rhs.imagine)
    }
}

// impl Add<f64> for &Complex {
//     type Output = Complex;
//     fn add(self, rhs: f64) -> Self::Output {
//         Complex::new(self.real + rhs, self.imagine)
//     }
// }

// Complex可以和任意实现了Add<f64, Output=f64>的类型相加
impl<T> Add<T> for &Complex
where
    T: Add<f64, Output = f64>,
{
    type Output = Complex;
    fn add(self, rhs: T) -> Self::Output {
        Complex::new(rhs + self.real, self.imagine)
    }
}

trait Parent {
    type T: Debug + Clone + Copy;
    fn do_echo(&self, v: Self::T) {
        println!("{:?}", v);
    }
}

/// 任意实现了`Child`的类型，也必须实现`Parent`这个`trait`，
/// `trait Child` 在定义时可以使用 `trait Parent` 中的关联类型和方法，
/// `Parent`称为`Child`的`supertrait`
trait Child: Parent {
    fn echo(&self, v: Self::T) {
        self.do_echo(v);
    }
}

// TODO static dispatch
// `fn print<T:Debug>(v: T)` <=> `fn print(v: impl Debug)`；
// 在编译时做单态化，静态分发，效率高；

// TODO dynamic dispatch
// `&dyn trait` or `Box<dyn trait>`，`dyn`关键字用于表示这是一个`trait object`；
// `trait object`是`DST`，大小编译时未知，是一个`fat pointer`，栈上存储`pointer`和`vtable`；
// 如果`trait`的所有的方法，返回值是`Self`或者使用了泛型参数，那么这个`trait`就不能产生`trait object`，
// - 类型擦除，无法确定`Self`的具体类型和大小；
// - 泛型在编译的时候做单态化，而`trait object`是运行时的产物，两者不能兼容；
// 如果一个`trait`只有部分方法返回`Self`或者使用了泛型参数，那么这部分方法在`trait object`中不能调用；
