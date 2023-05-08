pub fn master(show: bool) {
    if show {
        println!("-- Working with Structs");

        let developer_friend = Human::new("Caroline Morton", 30);

        let developer = Human::new("Maxwell Flitton", 32)
            .with_thought("I love Rust!")
            .with_friend(Box::new(developer_friend));

        println!("{:?}", developer);
        println!("{}", developer.name);
        println!("{}", developer.age);
    }
}

#[derive(Debug)]
struct Human {
    name: String,
    age: i8,
    current_thought: Option<String>,
    friend: Friend
}

impl Human {
    fn new(name: &str, age: i8) -> Human {
        return Human{
            name: name.to_string(),
            age,
            current_thought: None,
            friend: Friend::NIL
        }
    }

    fn with_thought(mut self, thought: &str) -> Human {
        self.current_thought = Some(thought.to_string());
        return self
    }

    fn with_friend(mut self, friend: Box<Human>) -> Human {
        self.friend = Friend::HUMAN(friend);
        return self
    }
}

#[derive(Debug)]
enum Friend {
    HUMAN(Box<Human>),
    NIL
}