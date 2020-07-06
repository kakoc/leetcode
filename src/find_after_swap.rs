pub fn search(nums: Vec<i32>, target: i32) -> i32 {
    let mut l = 0;
    let mut r = nums.len() - 1;
    for i in 0..nums.len() {
        if l > r {
            return -1;
        }

        let m = (l + r) / 2;
        if nums[m] == target {
            return m as i32;
        }

        if nums[l] <= nums[m] {
            if nums[l] <= target && target <= nums[m] {
                r = m - 1;
            } else {
                l = m + 1;
            }
            continue;
        }

        if nums[m + 1] <= target && target <= nums[r] {
            l = m + 1;
            continue;
        }

        r = m - 1;
    }

    -1
}

#[test]
fn test_binary_search() {
    let i = vec![5, 1, 3];
    let a = search(i, 3);

    assert_eq!(a, 2);
}
