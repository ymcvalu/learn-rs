fn main() {
    use Gender::*;
    let alice = User {
        id: UserID(1),
        gender: Female,
        name: "alice".into(),
    };
    let bob = User {
        id: UserID(2),
        gender: Male,
        name: "bob".into(),
    };

    let topic = Topic {
        id: TopicID(1),
        name: "rust".into(),
        owner: alice.id,
    };

    let event1 = Event::Join(alice.id, topic.id);
    let event2: Event = Event::Join(bob.id, topic.id);
    let event3 = Event::Message(alice.id, topic.id, "hello!".into());
    let event4 = Event::Leave(bob.id, topic.id);
    // {:?} 打印实现了 Debug trait 的类型值
    println!("{:?} {:?} {:?} {:?}", event1, event2, event3, event4);

    let t1 = Topic {
        id: TopicID(2),
        ..topic // 其他字段从topic获取，topic.name is partial moved
    };

    // Error: topic.name has patial moved
    // print!("{:?}", topic);
    println!("{:?}", t1);
}

// 派生宏，自动实现trait
#[derive(Debug, Clone, Copy)]
enum Gender {
    Unspecified = 0,
    Female = 1,
    Male = 2,
}
#[derive(Debug, Clone, Copy)]
struct UserID(u64); // tuple struct

#[derive(Debug, Clone)]
struct User {
    id: UserID,
    gender: Gender,
    name: String,
}

#[derive(Debug, Clone, Copy)]
struct TopicID(u64);

#[derive(Debug)]
struct Topic {
    id: TopicID,
    name: String,
    owner: UserID,
}

#[derive(Debug)]
enum Event {
    Join(UserID, TopicID),
    Leave(UserID, TopicID),
    Message(UserID, TopicID, String),
}
