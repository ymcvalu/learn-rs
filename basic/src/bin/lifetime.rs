/// 编译器会通过一些简单的规则为函数自动添加标注：
/// - 所有引用类型的参数都有独立的生命周期 'a 、'b 等；
/// - 如果只有一个引用型输入，它的生命周期会赋给所有输出；
/// - 如果有多个引用类型的参数，其中一个是 self，那么它的生命周期会赋给所有输出；

fn main() {
    println!("{}", max(&1, &2));

    let s = &mut "hello rust";
    let prefix = strtok(s, ' ');
    println!("{} {}", prefix, s);

    let mut s: String = "extend lifetime".into();
    let v = extend_lifetime(&s); // immutable borrow occurs here

    // ERROR: cannot borrow `s` as mutable because it is also borrowed as immutable
    // s.push_str("!!!"); // mutable borrow occurs here

    println!("{}", v); // immutable borrow later used here
    s.push_str("!!!");
    println!("{}", s);
}

// 生命周期标注的目的是，在参数和返回值之间建立联系或者约束，并不会改变原有的生命周期；
// 通过lifetime约束：caller对于返回值的使用，不能超过a或者b的生命周期，或者说参数a和b的
// 生命周期，必须大于返回值的有效期；
fn max<'a, T: Ord>(a: &'a T, b: &'a T) -> &'a T {
    if a > b {
        return a;
    }
    return b;
}

fn strtok<'a>(s: &mut &'a str, delimiter: char) -> &'a str {
    match s.find(delimiter) {
        Some(i) => {
            let prefix = &(*s)[..i];
            *s = &s[i + delimiter.len_utf8()..];
            prefix
        }
        _ => {
            let prefix = *s;
            *s = "";
            prefix
        }
    }
}

/// **The borrow-checker doesn’t actually understand the returned borrow.**
/// rust不知道函数内部的细节，而是根据函数的 `lifetime annotation`，来维持借用规则；
/// `extend_lifetime` 方法的参数和返回值，使用相同的 `lifetime annotation`，
/// 相当于告诉编译器，返回值跟参数有关系，参数的生命期需要比返回值的还长，
/// 那么参数的借用生命期就会被返回值extend；尽管实际上，我们的返回值和参数没有任何关联
fn extend_lifetime<'a>(_: &'a String) -> &'a str {
    return "vvv";
}

fn struct_demo() {
    let item;
    {
        let s1 = "key".into();
        let s2 = "val".into();
        item = Item {
            key: &s1,
            val: &s2,
            expire: 10,
        }

        // s1和s2在被释放了
    }

    // Error: key和val引用的值s1和s2已经被释放，item无法被使用了
    // println!("{}", item.expire);
}

/// 使用数据结构时，数据结构自身的生命周期，需要小于等于其内部字段的所有引用的生命期;
/// Item有 lifetime annotation标注，
/// 则约束key和val所引用的值的生命期至少需要和Item一样长
/// This annotation means an instance of `Item` can’t outlive the reference it holds in its `key` or `val` field.
struct Item<'a, 'b> {
    key: &'a String,
    val: &'b String,
    expire: i64,
}
