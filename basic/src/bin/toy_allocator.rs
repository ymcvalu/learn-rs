use std::alloc::{GlobalAlloc, Layout, System};

struct ToyAllocator;

unsafe impl GlobalAlloc for ToyAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let data = System.alloc(layout);
        eprintln!("ALLOC: {:p}, size: {}", data, layout.size());
        data
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        System.dealloc(ptr, layout);
        eprintln!("FREE: {:p}, size: {}", ptr, layout.size())
    }
}

#[global_allocator]
static GLOBAL: ToyAllocator = ToyAllocator;

fn main() {
    println!("===>");
    let _ = Box::<String>::new("xxx".into());

    // Box会先执行内部数据的drop方法，然后再释放内存
    let c = Child { str: "xxxx".into() };
    let p = Parent { child: Box::new(c) };
    let p = Box::new(p);
}

struct Parent {
    child: Box<Child>,
}

struct Child {
    str: String,
}

impl Drop for Parent {
    fn drop(&mut self) {
        println!("drop parent...");
    }
}

impl Drop for Child {
    fn drop(&mut self) {
        println!("drop child...");
    }
}
