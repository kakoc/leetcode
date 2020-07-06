pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    let mut res = vec![0; nums.len()];

    let mut p = 1;
    for (i, v) in nums.iter().rev().enumerate() {
        if i == 0 {
            res[nums.len() - 1 - i] = 1;
            p = *v;
            continue;
        }

        res[nums.len() - 1 - i] = p;
        p *= v;
    }

    let mut p = 1;
    for (i, v) in nums.iter().enumerate() {
        if i == 0 {
            p *= v;
            continue;
        }

        res[i] = p * res[i];
        p *= v;
    }

    res
}

#[test]
fn test_except_self() {
    let i = vec![1, 2, 3, 4];
    let a = product_except_self(i);
    assert_eq!(a, vec![24, 12, 8, 6]);

    let i = vec![9, 0, -2];
    let a = product_except_self(i);
    assert_eq!(a, vec![0, -18, 0]);
}
