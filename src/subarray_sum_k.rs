pub fn subarray_sum(nums: Vec<i32>, k: i32) -> i32 {
    let mut res = 0;
    let mut sum = 0;

    let mut m: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
    for v in nums {
        sum += v;

        if sum == k {
            res += 1;
        }

        {
            if let Some(v) = m.get(&(sum - k)) {
                res += v;
            }
        }

        let mut e = m.entry(sum).or_insert(0);
        *e += 1;
    }

    res as i32
}

#[test]
fn test_subarray_sum() {
    let i = vec![-1, -2, 1, 2, -1, -2, 1, 2, -1, -2];
    let a = subarray_sum(i, 3);

    assert_eq!(a, 3);
}
