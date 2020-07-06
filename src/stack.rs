struct MinStack {
    v: Vec<(i32, i32)>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {
    /** initialize your data structure here. */
    fn new() -> Self {
        MinStack { v: Vec::new() }
    }

    fn push(&mut self, x: i32) {
        let mut min = x;
        if let Some(v) = self.v.last() {
            if v.1 < x {
                min = v.1;
            }
        }
        self.v.push((x, min));
    }

    fn pop(&mut self) {
        self.v.pop();
    }

    fn top(&self) -> i32 {
        self.v.last().unwrap().0
    }

    fn get_min(&self) -> i32 {
        self.v.last().unwrap().1
    }
}

// * Your MinStack object will be instantiated and called as such:
// * let obj = MinStack::new();
// * obj.push(x);
// * obj.pop();
// * let ret_3: i32 = obj.top();
// * let ret_4: i32 = obj.get_min();

#[test]
fn test_stack() {
    let mut minStack = MinStack::new();
    minStack.push(-2);
    minStack.push(0);
    minStack.push(-3);
    let m = minStack.get_min(); //--> Returns -3.
    assert_eq!(m, -3);
    minStack.pop();
    minStack.top(); //--> Returns 0.
    let m = minStack.get_min(); //--> Returns -2
    assert_eq!(m, -2);
}
