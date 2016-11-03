mod ds {
    pub struct Stack<T> {
        stack: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Stack<T> {
            Stack { stack: Vec::new() }
        }

        pub fn push(&mut self, elem: T) {
            self.stack.push(elem);
        }

        pub fn pop(&mut self) -> T {
            self.stack.pop().unwrap()
        }

        pub fn top(&self) -> &T {
            &self.stack[self.stack.len() - 1]
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
    let mut stack_str = ds::Stack::new();
    stack_str.push("Hello");
    println!("{}", stack2.top());

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
