fn main() {
    let event = Event::Action {
        dir: Dir::Right,
        step: 3,
    };

    match event {
        Event::Action { dir, step } => (println!("dir {}, step {}", dir, step)),
        Event::Attack(dir) => (println!("attack toward {}", dir)),
        _ => (println!("stop!")),
    }

    // if let 是 match 的语法糖，如果模式匹配成功则执行对应分支
    if let Event::Action { dir, step } = event {
        println!("dir {}, step {}", dir, step)
    }
}

#[derive(Clone, Copy)]
enum Dir {
    Left = 1,
    Right = 2,
    Up = 3,
    Down = 4,
}
enum Event {
    Action { dir: Dir, step: i32 },
    Attack(Dir),
    Stop,
}

impl std::fmt::Display for Dir {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Self::Left => write!(f, "LEFT"),
            &Self::Right => write!(f, "RIGHT"),
            &Self::Up => write!(f, "UP"),
            &Self::Down => write!(f, "DOWN"),
        }
    }
}
