mod ds {
    pub struct Stack {
        stack: Vec<i32>,
    }

    impl Stack {
        pub fn new() -> Stack {
            Stack { stack: Vec::new() }
        }

        pub fn push(&mut self, elem: i32) {
            self.stack.push(elem);
        }

        pub fn pop(&mut self) -> i32 {
            self.stack.pop().unwrap()
        }

        pub fn top(&self) -> i32 {
            self.stack[self.stack.len() - 1]
        }

        pub fn empty(&self) -> bool {
            self.stack.is_empty()
        }

        pub fn size(&self) -> usize {
            self.stack.len()
        }
    }
}

fn main() {
    let mut stack = ds::Stack::new();

    println!("Empty: {}", stack.empty());
    println!("Size: {}", stack.size());
    stack.push(3);
    stack.push(2);
    stack.push(1);
    println!("Empty: {}", stack.empty());
    println!("Size: {}", stack.size());

    while !stack.empty() {
        println!("Top: {}", stack.top());
        stack.pop();
    }

    println!("Empty: {}", stack.empty());
}
