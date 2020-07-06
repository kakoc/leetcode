// cache.put(1, 1);
// cache.put(2, 2);
// cache.get(1);       // returns 1
// cache.put(3, 3);    // evicts key 2
// cache.get(2);       // returns -1 (not found)
// cache.put(4, 4);    // evicts key 1
// cache.get(1);       // returns -1 (not found)
// cache.get(3);       // returns 3
// cache.get(4);       // returns 4

use std::collections::{HashMap, LinkedList};
use std::rc::Rc;
struct Node {
    prev: Option<*mut Node>,
    next: Option<*mut Node>,
    val: i32,
}
struct LRUCache {
    m: HashMap<i32, (i32, *mut Node)>,
    c: i32,

    head: *mut Node,
    tail: *mut Node,
}

impl LRUCache {
    fn new(capacity: i32) -> Self {
        let nh = Box::new(Node {
            prev: None,
            next: None,
            val: -1,
        });
        let nt = Box::new(Node {
            prev: None,
            next: None,
            val: -1,
        });
        let mut n = LRUCache {
            m: HashMap::new(),
            c: capacity,

            head: Box::into_raw(nh),
            tail: Box::into_raw(nt),
        };

        unsafe {
            (*n.head).next = Some(n.tail);
            (*n.tail).prev = Some(n.head);
        }

        n
    }

    fn get(&mut self, key: i32) -> i32 {
        if self.m.is_empty() || !self.m.contains_key(&key) {
            -1
        } else {
            let n = self.m.get(&key).unwrap().clone();
            self.remove(n.1);
            self.append(n.1);

            n.0
        }
    }

    fn put(&mut self, key: i32, value: i32) {
        if self.m.contains_key(&key) {
            let mut n = self.m.get_mut(&key).unwrap().clone();
            self.remove(n.1);
            self.append(n.1);

            self.m.entry(key).and_modify(|e| {
                e.0 = value;
            });
        } else {
            if self.m.keys().len() == self.c as usize {
                unsafe {
                    let v = (*(*self.tail).prev.unwrap()).val;
                    self.m.remove(&v);
                    self.remove((*self.tail).prev.unwrap());
                }
            }
            let node = Node {
                prev: None,
                next: None,
                val: key,
            };
            let r = Box::into_raw(Box::new(node));

            self.m.insert(key, (value, r));
            self.append(r);
        }
    }

    fn append(&mut self, node: *mut Node) {
        unsafe {
            let head_next = (*self.head).next;
            (*node).next = head_next;
            (*head_next.unwrap()).prev = Some(node);
            (*self.head).next = Some(node);
            (*node).prev = Some(self.head);
        }
    }

    fn remove(&mut self, node: *mut Node) {
        unsafe {
            let mut prev = (*node).prev.unwrap();
            let mut next = (*node).next.unwrap();
            (*prev).next = Some(next);
            (*next).prev = Some(prev);
        }
    }
}

#[test]
fn test_lru_cache() {
    // let i = []
    let mut cache = LRUCache::new(2); // /* capacity */ );

    cache.put(1, 1);
    cache.put(2, 2);
    cache.get(1); // returns 1
    cache.put(3, 3); // evicts key 2
    let a = cache.get(2); // returns -1 (not found)
                          // assert_eq!(a, -1);
    cache.put(4, 4); // evicts key 1
    let a = cache.get(1); // returns -1 (not found)
    assert_eq!(a, -1);
    cache.get(3); // returns 3
    cache.get(4); // returns 4

    // let mut cache = LRUCache::new(2); // /* capacity */ );

    // cache.get(2);
    // cache.put(2, 6);
    // cache.get(1); // returns 1
    // cache.put(1, 5); // evicts key 2
    // cache.put(1, 2); // returns -1 (not found)
    // cache.get(1); // evicts key 1
    // cache.get(2); // returns -1 (not found)
    // cache.get(3); // returns 3
    // cache.get(4); // returns 4

    // // ["LRUCache","get","put","get","put","put","get","get"]
    // // [[2],[2],[2,6],[1],[1,5],[1,2],[1],[2]]

    // let mut cache = LRUCache::new(2); // /* capacity */ );

    // cache.get(2);
    // cache.put(2, 1);
    // cache.put(1, 1); // returns 1
    // cache.put(2, 3); // evicts key 2
    // cache.put(4, 1); // returns -1 (not found)
    // let a = cache.get(1); // evicts key 1
    // let b = cache.get(2); // returns -1 (not found)

    // assert_eq!(a, -1);
    // assert_eq!(b, 3);

    // // //     ["LRUCache","put","put","put","put","get","get"]
    // // // [[2],[2,1],[1,1],[2,3],[4,1],[1],[2]]
    // let mut cache = LRUCache::new(2); // /* capacity */ );

    // cache.put(2, 1);
    // cache.put(3, 2);
    // cache.get(3); // returns 1
    // cache.get(2); // evicts key 2
    // cache.put(4, 3); // returns -1 (not found)
    // cache.get(2); // evicts key 1
    // cache.get(3); // returns -1 (not found)
    // cache.get(4); // returns 3

    // // // ["LRUCache","put","put","get","get","put","get","get","get"]
    // // // [[2],[2,1],[3,2],[3],[2],[4,3],[2],[3],[4]]

    // let mut cache = LRUCache::new(2); // /* capacity */ );

    // cache.put(2, 1);
    // cache.put(2, 2);
    // // cache.put(2, 2); // returns 1
    // cache.get(2); // evicts key 2
    // cache.put(1, 1); // returns -1 (not found)
    // cache.put(4, 1); // evicts key 1
    // cache.get(2); // returns -1 (not found)

    // // ["LRUCache","put","put","get","put","put","get"]
    // // [[2],[2,1],[2,2],[2],[1,1],[4,1],[2]]

    // let mut cache = LRUCache::new(10); // /* capacity */ );

    // cache.put(10, 13);
    // cache.put(3, 17);
    // cache.put(6, 11); // evicts key 2
    // cache.put(10, 5); // returns -1 (not found)
    // cache.put(9, 10); // evicts key 1
    // cache.get(13); // returns -1 (not found)
    // cache.put(2, 19); // returns -1 (not found)
    // cache.get(2); // returns -1 (not found)
    // cache.get(3); // returns -1 (not found)
    // cache.put(5, 25); // returns -1 (not found)
    // cache.get(8); // returns -1 (not found)
    // cache.put(9, 22); // returns -1 (not found)
    // cache.put(5, 5); // returns -1 (not found)
    // cache.put(1, 30); // returns -1 (not found)
    // cache.get(11); // returns -1 (not found)
    // cache.put(9, 12); // returns -1 (not found)
    // cache.get(7); // returns -1 (not found)
    // cache.get(5); // returns -1 (not found)
    // cache.get(8); // returns -1 (not found)
    // cache.get(9); // returns -1 (not found)
    // cache.put(4, 30); // returns -1 (not found)
    // cache.put(9, 3); // returns -1 (not found)
    // cache.get(9); // returns -1 (not found)
    // cache.get(10); // returns -1 (not found)
    // cache.get(10); // returns -1 (not found)
    // cache.put(6, 14); // returns -1 (not found)
    // cache.put(3, 1); // returns -1 (not found)
    // cache.get(3); // returns -1 (not found)
    // cache.put(10, 11); // returns -1 (not found)

    // // ["LRUCache","put","put","put","put","put","get","put","get","get","put","get","put","put","put","get","put","get","get","get","get","put","put","get","get","get","put","put","get","put","get","put","get","get","get","put","put","put","get","put","get","get","put","put","get","put","put","put","put","get","put","put","get","put","put","get","put","put","put","put","put","get","put","put","get","put","get","get","get","put","get","get","put","put","put","put","get","put","put","put","put","get","get","get","put","put","put","get","put","put","put","get","put","put","put","get","get","get","put","put","put","put","get","put","put","put","put","put","put","put"]
    // // [[10],[10,13],[3,17],[6,11],[10,5],[9,10],[13],[2,19],[2],[3],[5,25],[8],[9,22],[5,5],[1,30],[11],[9,12],[7],[5],[8],[9],[4,30],[9,3],[9],[10],[10],[6,14],[3,1],[3],[10,11],
    // [8],[2,14],[1],[5],[4],[11,4],[12,24],[5,18],[13],[7,23],[8],[12],[3,27],[2,12],[5],[2,9],[13,4],[8,18],[1,7],[6],[9,29],[8,21],[5],[6,30],[1,12],[10],[4,15],[7,22],[11,26],[8,17],[9,29],[5],[3,4],[11,30],[12],[4,29],[3],[9],[6],[3,4],[1],[10],[3,29],[10,28],[1,20],[11,13],[3],[3,12],[3,8],[10,9],[3,26],[8],[7],[5],[13,17],[2,27],[11,15],[12],[9,19],[2,15],[3,16],[1],[12,17],[9,1],[6,19],[4],[5],[5],[8,1],[11,7],[5,2],[9,28],[1],[2,2],[7,4],[4,22],[7,24],[9,26],[13,28],[11,26]]
}
