struct Stack {
    size: usize,
    data: [i32; 10],
}

impl Stack {
    fn new() -> Stack {
        Stack { size: 0, data: [0; 10] }
    }

    fn push(&mut self, elem: i32) {
        if self.size < 10 {
            self.data[self.size] = elem;
            self.size += 1;
        }
        else {
            panic!("Stack full!")
        }
    }

    fn pop(&mut self) -> i32 {
        if self.size > 0 {
            let temp = self.data[self.size - 1];
            self.size -= 1;
            temp
        }
        else {
            panic!("Stack is empty!");
        }
    }

    fn top(&self) -> i32 {
        if self.size > 0 {
            self.data[self.size - 1]
        }
        else {
            panic!("Stack is empty!");
        }
    }

    fn empty(&self) -> bool {
        self.size == 0
    }

    fn size(&self) -> usize {
        self.size
    }
}

fn main() {
    let mut stack = Stack::new();

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
