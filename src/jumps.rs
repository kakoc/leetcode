pub fn can_jump(nums: Vec<i32>) -> bool {
    if nums.len() <= 1 {
        return true;
    }

    let mut i: usize = 0;
    let mut n: usize = *nums.get(i).unwrap() as usize;
    while i <= n && i < nums.len() {
        if i + *nums.get(i).unwrap() as usize >= n {
            n = i + *nums.get(i).unwrap() as usize;
        }
        i += 1;
    }

    return i - 1 >= nums.len() - 1;
}

#[test]
fn test_jumps() {
    let i = vec![1, 2, 3];
    let a = can_jump(i);
    assert_eq!(a, true);

    let i = vec![3, 2, 1, 0, 4];
    let a = can_jump(i);
    assert_eq!(a, false);

    let i = vec![0, 2, 3];
    let a = can_jump(i);
    assert_eq!(a, false);

    let i = vec![2, 0, 0];
    let a = can_jump(i);
    assert_eq!(a, true);
}
