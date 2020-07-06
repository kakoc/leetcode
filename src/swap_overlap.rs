fn solve(nums: &mut Vec<i32>) {
    if nums.len() == 1 {
        return;
    }
    if nums.len() == 2 {
        if nums[0] == 0 {
            nums[0] = nums[1];
            nums[1] = 0;
        }
    }

    let mut swap_start = 0;
    let mut swap_end = 0;

    for i in 0..nums.len() {
        if nums[i] == 0 {
            swap_start = i;
            swap_end = i;

            break;
        }
    }

    if swap_start > nums.len() - 2 || (swap_start == 0 && nums[0] != 0) {
        return;
    }

    loop {
        if swap_start == nums.len() - 1 || swap_end == nums.len() - 1 {
            return;
        }
        if nums[swap_end + 1] != 0 {
            let mut offset = swap_end + 1 + (swap_end - swap_start);
            if (swap_end + 1 - swap_start) > nums.len() - (swap_end + 1) {
                offset = swap_end + 1 + ((nums.len() - 1) - (swap_end + 1));
            }

            let len: usize = nums[swap_end + 1..=offset]
                .iter()
                .take_while(|x| **x != 0)
                .count();
            swap(nums, len, swap_start, swap_end + 1);
            swap_start += len;
            swap_end = swap_start;
        } else {
            swap_end += 1;
        }
    }
}

fn swap<T>(data: &mut Vec<T>, len: usize, a: usize, b: usize) {
    if b + len > data.len() {
        swap_range(data, data.len() - b, a, b);
    } else {
        swap_range(data, len, a, b);
    }
}

pub fn swap_range<T>(data: &mut Vec<T>, len: usize, a: usize, b: usize) {
    let (first, second) = if a > b {
        data[b..].split_at_mut(a - b)
    } else {
        data[a..].split_at_mut(b - a)
    };

    first[..len].swap_with_slice(&mut second[..len]);
}

#[test]
fn test_overlapping_swap() {
    let mut i = vec![1, 2, 3, 4];
    swap_range(&mut i, 2, 0, 2);
    assert_eq!(i, vec![3, 4, 1, 2]);

    let mut i = vec![1, 2, 3, 4];
    swap_range(&mut i, 1, 0, 3);
    assert_eq!(i, vec![4, 2, 3, 1]);

    let mut i = vec![1, 2];
    swap_range(&mut i, 1, 0, 1);
    assert_eq!(i, vec![2, 1]);
}

#[test]
fn test_solve_swap() {
    let mut i = vec![0, 1, 0, 3, 12];
    solve(&mut i);
    assert_eq!(i, vec![1, 3, 12, 0, 0]);

    let mut i = vec![1, 2];
    solve(&mut i);
    assert_eq!(i, vec![1, 2]);

    let mut i = vec![0, 1, 0, 3, 12];
    solve(&mut i);
    assert_eq!(i, vec![1, 3, 12, 0, 0]);

    let mut i = vec![1];
    solve(&mut i);
    assert_eq!(i, vec![1]);

    let mut i = vec![0, 1];
    solve(&mut i);
    assert_eq!(i, vec![1, 0]);

    let mut i = vec![0, 0, 1];
    solve(&mut i);
    assert_eq!(i, vec![1, 0, 0]);

    let mut i = vec![1, 0, 1];
    solve(&mut i);
    assert_eq!(i, vec![1, 1, 0]);

    let mut i = vec![4, 2, 4, 0, 0, 3, 0, 5, 1, 0];
    solve(&mut i);
    assert_eq!(i, vec![4, 2, 4, 3, 5, 1, 0, 0, 0, 0]);
}
