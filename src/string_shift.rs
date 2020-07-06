pub fn string_shift(s: String, shift: Vec<Vec<i32>>) -> String {
    let mut l: usize = 0;
    let mut r: usize = 0;

    for v in shift {
        if v[0] == 0 {
            l += (v[1] as usize);
        } else {
            r += (v[1] as usize);
        }
    }

    if l == r {
        s
    } else if l < r {
        let diff = (r - l) % s.len();
        format!(
            "{}{}",
            s[s.len() - diff..].to_string(),
            s[0..s.len() - diff].to_string(),
        )
    } else {
        let diff = (l - r) % s.len();
        format!("{}{}", s[diff..].to_string(), s[0..diff].to_string())
    }
}

#[test]
fn test_shift() {
    let s = "abcdefg".to_string();
    let shift: Vec<Vec<i32>> = vec![
        [1, 1].to_vec(),
        [1, 1].to_vec(),
        [0, 2].to_vec(),
        [1, 3].to_vec(),
    ];

    let a = string_shift(s, shift);
    assert_eq!(a, "efgabcd".to_string());

    let s = "wpdhhcj".to_string();
    let shift: Vec<Vec<i32>> = vec![
        [0, 7].to_vec(),
        [1, 7].to_vec(),
        [1, 0].to_vec(),
        [1, 3].to_vec(),
        [0, 3].to_vec(),
        [0, 6].to_vec(),
        [1, 2].to_vec(),
    ];
    let a = string_shift(s, shift);
    assert_eq!(a, "hcjwpdh".to_string());
}
