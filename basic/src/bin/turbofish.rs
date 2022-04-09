use std::fmt::Display;
use std::fmt::Formatter;
use std::ops::Add;

fn main() {
    let potato = Potato;

    // 在泛型函数后使用 ::<T> 来强制使用类型 T，这种写法被称为 turbofish
    let food = potato.cook::<Chips>();
    println!("{}", food);

    println!("{}", do_add::<i32>(1i32, 2));
    println!("{}", do_add::<u32>(1u32, 2));
    println!("{}", do_add::<i64>(1i64, 2));
}

fn do_add<T: Add<Output = T>>(t1: T, t2: T) -> T {
    t1 + t2
}

trait Food {
    fn cook(p: &Potato) -> Self;
}

struct Chips;
impl Food for Chips {
    fn cook(p: &Potato) -> Self {
        Chips
    }
}

impl Display for Chips {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "this is chips")
    }
}

struct Mash;
impl Food for Mash {
    fn cook(p: &Potato) -> Self {
        Mash
    }
}

impl Display for Mash {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "this is mash")
    }
}

struct Potato;
impl Potato {
    fn cook<T: Food>(&self) -> T {
        T::cook(self)
    }
}
