// use std::collections::hash_map::Entry;
use std::collections::HashSet;

pub fn count_elements(arr: Vec<i32>) -> i32 {
    let mut map: HashSet<i32> = HashSet::new();

    let mut answer = 0;
    for i in arr.iter() {
        map.insert(*i);
    }

    for i in arr {
        if map.contains(&(i + 1)) {
            answer += 1;
        }
    }

    answer
}

#[test]
fn counting() {
    let i = vec![1, 2, 3];
    let a = count_elements(i);
    assert_eq!(a, 2);

    let i = vec![1, 1, 3, 3, 5, 5, 7, 7];
    let a = count_elements(i);
    assert_eq!(a, 0);

    let i = vec![1, 1, 2, 2];
    let a = count_elements(i);
    assert_eq!(a, 2);

    let i = vec![1];
    let a = count_elements(i);
    assert_eq!(a, 0);

    let i = vec![1, 1, 2];
    let a = count_elements(i);
    assert_eq!(a, 2);
}
