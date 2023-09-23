use std::mem::swap;

/// A first-in, first-out queue of characters.
#[derive(Debug)]
pub struct Queue {
    older: Vec<char>, // older elements, eldest last.
    younger: Vec<char>, // younger elements, youngest last.
}

impl Queue {
    /// Constructor functions using type-associated functions
    pub fn new() -> Queue {
        Queue {
            older: Vec::new(),
            younger: Vec::new(),
        }
    }

    /// Push a character onto the back of a queue.
    pub fn push(&mut self, c: char) {
        self.younger.push(c);
    }

    /// Pop a character off the front of a queue. Return `Some(c)`if there
    /// was a character to pop, or `None`if the queue was empty.
    pub fn pop(&mut self) -> Option<char> {
        if self.older.is_empty() {
            if self.younger.is_empty() {
                return None;
            }

            // Bring the elements in younger over to older, and put them in
            // the promised order.
            swap(&mut self.older, &mut self.younger);
            self.older.reverse();
        }

        // Now older is guaranteed to have something. Vec's pop method
        // already returns an Option, so we're set.
        self.older.pop()
    }

    pub fn is_empty(&self) -> bool {
        self.older.is_empty() && self.younger.is_empty()
    }

    pub fn split(self) -> (Vec<char>, Vec<char>) {
        (self.older, self.younger)
    }
}

pub fn master(show: bool) {
    if show {
        println!("\n-- Queue Struct");

        queue_without_construct(false);

        queue_with_construct(false);
    }
}

fn queue_without_construct(show: bool) {
    if show {
        println!("\n--- Queue Struct Without A Construct");

        let younger = vec!['a', 'b', 'c'];
        let older = vec!['x', 'y', 'z'];

        let mut queue = Queue {
            younger,
            older,
        };

        println!("{:?}", queue);

        queue.push('d');

        println!("{:?}", queue);

        queue.pop();

        println!("{:?}", queue);

        if !queue.is_empty() {
            println!("{:?}", queue);
        }

        let (older_values, younger_values) = queue.split();
        println!("Older Values: {:?}", older_values);
        println!("Younger Values: {:?}", younger_values);
    }
}

fn queue_with_construct(show: bool) {
    if show {
        println!("\n--- Queue Struct With Construct");

        let mut queue = Queue::new();

        queue.push('*');

        println!("{:?}", queue);
    }
}