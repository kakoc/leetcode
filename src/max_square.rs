pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
    if matrix.len() == 0 {
        return 0;
    }

    let mut max = 0;
    let n = matrix.len();
    let m = matrix[0].len();

    for i in 0..n {
        for j in 0..m {
            if matrix[i][j] == '1' {
                let calc = check(&matrix, (i, j));
                max = std::cmp::max(calc, max);
                if max == 4 {
                    println!("{} {}", i, j);
                }
            }
        }
    }

    max
}

pub fn check(matrix: &Vec<Vec<char>>, start: (usize, usize)) -> i32 {
    let mut max = 1 as usize;
    let mut i = (start.0 + 1, start.1 + 1);
    if let None = matrix.get(i.0) {
        return max as i32;
    }
    if let None = matrix.get(i.0).unwrap().get(i.1) {
        return max as i32;
    }
    // println!("here");
    loop {
        let mut u = i.0;
        let mut l = i.1;
        if matrix[i.0][i.1] != '1' {
            return max as i32;
        }

        // println!("{} {}", u, l);
        // println!("{} {} {:?}", u, l, i);
        while let (Some('1'), Some('1')) = (
            matrix.get(u - 1).unwrap().get(i.1),
            matrix.get(i.0).unwrap().get(l - 1),
        ) {
            u -= 1;
            l -= 1;
            if u == start.0 && l == start.1 {
                break;
            }
            // println!("{} {}", u, l);
        }

        // println!("{} {}", u, l);
        if u != start.0 || l != start.1 {
            return max as i32;
        }
        // println!("{} {} {:?}", u, l, i);
        max = (i.0 - u + 1) * (i.1 - l + 1);

        if let Some(r) = matrix.get(i.0 + 1) {
            if let Some('1') = r.get(i.1 + 1) {
                // println!("{} {}", u, l);
                // max = i.0 - u * i.1 - l;
                i = (i.0 + 1, i.1 + 1);
            } else {
                return max as i32;
            }
        } else {
            return max as i32;
        }
        // println!("{} {} {:?}", u, l, i);
    }
}

#[test]
fn test_max_square() {
    // let a = vec!['1', '0', '1', '0', '0'];
    // let b = vec!['1', '0', '1', '1', '1'];
    // let c = vec!['1', '1', '1', '1', '1'];
    // let d = vec!['1', '0', '0', '1', '0'];
    // let i = vec![a, b, c, d];

    // let a = maximal_square(i);
    // assert_eq!(a, 4);

    let a = vec![
        vec!['0', '0', '0', '1', '0', '1', '1', '1'],
        vec!['0', '1', '1', '0', '0', '1', '0', '1'],
        vec!['1', '0', '1', '1', '1', '1', '0', '1'],
        vec!['0', '0', '0', '1', '0', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0', '0', '1', '0'],
        vec!['1', '1', '1', '0', '0', '1', '1', '1'],
        vec!['1', '0', '0', '1', '1', '0', '0', '1'],
        vec!['0', '1', '0', '0', '1', '1', '0', '0'],
        vec!['1', '0', '0', '1', '0', '0', '0', '0'],
    ];
    let a = maximal_square(a);
    assert_eq!(a, 1);
}
