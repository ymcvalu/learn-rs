fn main() {
    // 所有权机制：
    // 一个值只能有一个owner：解决重复释放问题；
    // 不能同时拥有写借用和只读借用：解决悬浮指针问题；
    // 一个值同时只能有个一个写借用，并且存在写借用时，owner对应的变量也不能被使用，防止修改竞争；
    // NLL：借用生命期直到最后一次使用，不和文法作用域绑定
    immut_borrow_demo();
    mut_borrow_demo();
    reborrow_demo();
    reborrow_demo();
    struct_demo();
}

fn immut_borrow_demo() {
    // 一个值允许同时存在多个immut ref
    // 并且immut ref和值可以同时使用
    let v = 123;
    let r1 = &v;
    let r2 = &v;
    println!("{}", r1,);
    println!("{}", v);
    println!("{}", r2,);
}

fn mut_borrow_demo() {
    let mut i = 456;
    let mr = &mut i;
    // let mr2 = &mut i; // 一个值同一时间只能有1个有效的mut ref
    // let r = &i; // 不能同时存在有效的mut ref和immut ref
    // println!("{}", i); // mut ref存活期间，i不允许使用
    *mr = 2;
    println!("{}", mr);

    // NLL: 后续mr不再被使用，mr引用无效了
    let r = &i;
    println!("{}", i);
    println!("{}", r);
}

fn reborrow_demo() {
    // reborrow
    {
        let mut s = 1;
        let mr = &mut s;
        // reborrow from mr
        // mrr不会破坏mr对s的借用规则
        // mr在mrr的生命周期内不可以使用
        // 对mr的reborrow需要满足借用检查
        let mrr: &mut i32 = &mut *mr;
        println!("{}", mrr);
        println!("{}", mr);
    }

    // implicit reborrow
    {
        let mut s = String::from("hello");
        let mr = &mut s;
        push_str(&mut *mr, " rust!"); // implicit reborrow
        println!("{}", mr);
    }
}

fn push_str(s: &mut String, v: &str) {
    s.push_str(v);
}

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

fn reborrow_rule() {
    let mut p = Point { x: 0, y: 0 };
    let r = &mut p;
    let rr: &Point = &*r; // 基于可变引用构造不可变引用
                          // let mrr: &mut Point = &mut *r; // 已经存在immut ref rr，不允许再有mut ref
    println!("{:?}", rr);
    r.move_to(10, 10);
    println!("{:?}", r);
}

fn struct_demo() {
    let mut p = Person {
        name: "小明".into(),
        address: "中国".into(),
    };

    // struct的字段可以同时分开引用
    let name = &mut p.name;
    let address = &mut p.address;

    // Error：p上具有可变借用，不能同时有只读借用
    // println!("{:?}", &p);

    name.push_str("明");
    address.push_str("福建");
    println!("{:?}", p);
}

#[derive(Debug)]
struct Person {
    name: String,
    address: String,
}
