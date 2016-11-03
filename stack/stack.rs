mod ds {
    pub struct Stack {
        size: usize,
        data: [i32; 10],
    }

    impl Stack {
        pub fn new() -> Stack {
            Stack { size: 0, data: [0; 10] }
        }

        pub fn push(&mut self, elem: i32) {
            if self.size < 10 {
                self.data[self.size] = elem;
                self.size += 1;
            }
            else {
                panic!("Stack full!")
            }
        }

        pub fn pop(&mut self) -> i32 {
            if self.size > 0 {
                let temp = self.data[self.size - 1];
                self.size -= 1;
                temp
            }
            else {
                panic!("Stack is empty!");
            }
        }

        pub fn top(&self) -> i32 {
            if self.size > 0 {
                self.data[self.size - 1]
            }
            else {
                panic!("Stack is empty!");
            }
        }

        pub fn empty(&self) -> bool {
            self.size == 0
        }

        pub fn size(&self) -> usize {
            self.size
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
