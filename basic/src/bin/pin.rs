use std::marker::PhantomPinned;
use std::pin::Pin;
fn main() {
    move_creates_issue();
}

#[derive(Debug)]
struct SelfReference {
    name: String,
    name_ptr: *const String,
    /// If a type contains a `PhantomPinned`, it will not implement `Unpin` by default.
    /// `impl !Unpin for SelfReference`
    _marker: PhantomPinned,
}

impl SelfReference {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            name_ptr: std::ptr::null(),
            _marker: PhantomPinned,
        }
    }

    pub fn init(self: Pin<&mut Self>) {
        let name_ptr = &self.name as *const String;
        let this = unsafe { self.get_unchecked_mut() };
        this.name_ptr = name_ptr;
    }

    pub fn print_name(self: Pin<&Self>) {
        println!(
            "struct {:p}: (name: {:p} name_ptr: {:p}), name: {}, name_ref: {}",
            self, // impl `fmt::Pointer` for Pin<P>
            &self.name,
            self.name_ptr,
            self.name,
            unsafe { &*self.name_ptr }
        )
    }
}

fn move_creates_issue() {
    // `data: SelfRefrence` 的生命周期会被extend
    let mut data = SelfReference::new("Tyr");
    let mut data = unsafe { Pin::new_unchecked(&mut data) };
    SelfReference::init(data.as_mut());

    // 不 move，一切正常
    data.as_ref().print_name();

    // 现在只能拿到 pinned 后的数据，所以 move 不了之前
    move_pinned(data.as_mut());
    println!("{:?} ({:p})", data, &data);

    // `SelfReference`没有实现`Unpin`这个trait，无法借用可变引用
    // impl<P: DerefMut<Target: Unpin>> DerefMut for Pin<P> { ... }
    // let mut_ref: &mut SelfReference = &mut *data;

    // 你无法拿回 Pin 之前的 SelfReference 结构，所以调用不了 move_it
    // move_it(data);
}

fn move_pinned(data: Pin<&mut SelfReference>) {
    println!("{:?} ({:p})", data, &data);
}

#[allow(dead_code)]
fn move_it(data: SelfReference) {
    println!("{:?} ({:p})", data, &data);
}
