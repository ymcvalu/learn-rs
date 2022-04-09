fn main() {
    let mut p = Person {
        name: "name".into(),
        address: "address".into(),
    };

    let name = p.name; // partial move

    // Error: can't borrow from partially moved value: `p`
    // let pref = &p;

    let addrref = &p.address; // borrow other field is ok

    p.name = "new name".into(); // reassign
    let pref = &p; // we can borrow from p after reassigned

    let obj = DropObj { name: "obj".into() };
    // Error: cannot move out of type `DropObj`, which implements the `Drop` trait
    // let n = obj.name;
}

struct Person {
    name: String,
    address: String,
}

struct DropObj {
    name: String,
}

impl Drop for DropObj {
    fn drop(&mut self) {
        println!("drop...");
    }
}
