use std::collections::{HashMap, HashSet};

struct FirstUnique {
    v: Vec<i32>,
    fu: HashMap<usize, usize>,
    u: HashMap<usize, usize>,
    d: HashSet<i32>,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl FirstUnique {
    fn new(nums: Vec<i32>) -> Self {
        let mut fu: HashMap<usize, usize> = HashMap::new();
        let mut u: HashMap<usize, usize> = HashMap::new();
        let mut d: HashSet<i32> = HashSet::new();

        for (i, n) in nums.iter().enumerate() {
            if d.contains(n) {
                continue;
            }
            if u.contains_key(&(*n as usize)) {
                let i = u.remove(&(*n as usize)).unwrap();
                fu.remove(&i);
                d.insert(*n);
            } else {
                u.insert(*n as usize, i);
                fu.insert(i, *n as usize);
            }
        }

        FirstUnique { v: nums, fu, u, d }
    }

    fn show_first_unique(&self) -> i32 {
        if self.fu.is_empty() {
            return -1;
        };
        let key = self.fu.keys().min().unwrap();
        *self.fu.get(&key).unwrap() as i32
    }

    fn add(&mut self, value: i32) {
        if self.d.contains(&value) {
            return;
        }
        if self.u.contains_key(&(value as usize)) {
            let i = self.u.remove(&(value as usize)).unwrap();
            self.fu.remove(&i);
            self.d.insert(value);
        } else {
            self.u.insert(value as usize, self.v.len());
            self.fu.insert(self.v.len(), value as usize);
        }
    }
}

#[test]
fn test_first_unique() {
    let mut f = FirstUnique::new(vec![2, 3, 5]);
    assert_eq!(2, f.show_first_unique());
    f.add(5);
    assert_eq!(2, f.show_first_unique());
    f.add(2);
    assert_eq!(3, f.show_first_unique());
    f.add(3);
    assert_eq!(-1, f.show_first_unique());
}
