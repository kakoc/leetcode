use std::collections::HashMap;
pub fn find_max_length(nums: Vec<i32>) -> i32 {
    if nums.len() < 2 {
        return 0;
    }

    let mut map: HashMap<i32, i32> = HashMap::new();
    map.insert(0, -1);

    let mut sum = 0;
    let mut max = 0;
    for (i, v) in nums.iter().enumerate() {
        if *v == 0 {
            sum -= 1;
        } else {
            sum += 1;
        }
        if map.contains_key(&sum) {
            let entry = map.get(&sum).unwrap();
            max = std::cmp::max((i as i32) - *entry, max);
        } else {
            map.insert(sum, i as i32);
        }
    }

    max
}

#[test]
fn test_binary() {
    let i = vec![0, 1, 0];
    let a = find_max_length(i);
    assert_eq!(a, 2);

    let i = vec![0, 1];
    let a = find_max_length(i);
    assert_eq!(a, 2);

    let i = vec![0, 1, 0, 1];
    let a = find_max_length(i);
    assert_eq!(a, 4);

    let i = vec![0];
    let a = find_max_length(i);
    assert_eq!(a, 0);

    let i = vec![0, 0, 0, 1, 1, 1, 0];
    let a = find_max_length(i);
    assert_eq!(a, 6);

    let i = vec![0, 0, 0, 1, 1];
    let a = find_max_length(i);
    assert_eq!(a, 4);

    let i = vec![0, 1, 1, 1, 1, 0, 0, 0, 0];
    let a = find_max_length(i);
    assert_eq!(a, 8);

    let i = vec![0, 0, 0, 1, 1, 1, 1, 0, 0, 0, 0];
    let a = find_max_length(i);
    assert_eq!(a, 8);

    let i = vec![0, 1, 1];
    let a = find_max_length(i);
    assert_eq!(a, 2);
}
