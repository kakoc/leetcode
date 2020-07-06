use std::collections::HashMap;

pub fn longest_common_subsequence(text1: String, text2: String) -> i32 {
    let n = text1.len();
    let m = text2.len();
    let mut v: Vec<Vec<usize>> = vec![vec![0; text2.len() + 1]; text1.len() + 1];

    println!("{:?}", v);
    for i in 0..n {
        for j in 0..m {
            if text1.chars().nth(i).unwrap() == text2.chars().nth(j).unwrap() {
                v[i + 1][j + 1] = v[i][j] + 1;
            } else {
                v[i + 1][j + 1] = std::cmp::max(v[i][j + 1], v[i + 1][j]);
            }
        }
    }

    // println!("{:?}", v);
    v[n][m] as i32
}

#[test]
fn test_longest_common_subseq() {
    let i = "abcde".to_string();
    let ii = "ace".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 3);

    let i = "abc".to_string();
    let ii = "def".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 0);

    let i = "bsbininm".to_string();
    let ii = "jmjkbkjkv".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 1);

    let i = "pmjghexybyrgzczy".to_string();
    let ii = "hafcdqbgncrcbihkd".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 4);

    let i = "acy;gbn".to_string();
    let ii = "ktpbwac".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 2);

    let i = "oxcpqrsvwf".to_string();
    let ii = "shmtulqrypy".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 2);

    let i = "ylqpejqbalahwr".to_string();
    let ii = "yrkzavgdmdgtqpg".to_string();

    let a = longest_common_subsequence(i, ii);
    assert_eq!(a, 3);
}
