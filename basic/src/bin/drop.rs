struct DropItem;

struct DropDemo {
    tag: String,
    item: DropItem,
}

fn main() {
    let leak_ptr;
    {
        let b1 = Box::new(DropDemo {
            tag: "ss".into(),
            item: DropItem {},
        });
        let b2 = b1; // move

        leak_ptr = Box::leak(b2); // b2作为参数，move之后，drop方法不会被调用；返回的是具有'static的引用
    }

    unsafe {
        println!("{}", (&mut *leak_ptr).tag);
        Box::from_raw(leak_ptr);
        // 这里触发drop
        // 先执行DropDemo.drop
        // 然后执行DropItem.drop
    }

    let d1 = DropDemo {
        tag: "tag".into(),
        item: DropItem,
    };

    // Error[E0509]: cannot move out of type `B`, which implements the `Drop` trait
    // let tag = d1.tag;

    println!("end...")
}

impl Drop for DropDemo {
    fn drop(&mut self) {
        println!("drop {}...", self.tag);
        // 不需要手动调用item.drop，编译器会自动生成
    }
}

impl Drop for DropItem {
    fn drop(&mut self) {
        println!("drop item...");
    }
}
