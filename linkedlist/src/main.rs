mod list;
use list::List;

fn main() {
    let mut list = List::new();

    list.push_back(1);
    list.push_back(2);
    list.push_back(3);
    list.push_back(4);
    list.push_back(5);

    println!("print from front:");
    list.print_from_front();

    println!("print from back:");
    list.print_from_back();
}
