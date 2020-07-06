pub fn check_valid_string(s: String) -> bool {
    // let mut o = 0;
    // let mut c = 0;
    // let mut st = 0;
    // for v in s.chars() {
    //     match v {
    //         ')' => c += 1,
    //         '(' => o += 1,
    //         '*' => st += 1,
    //         _ => unreachable!(),
    //     }
    // }
    // if ((o - c) as i32).abs() > st {
    //     return false;
    // }

    tr(&s, vec![])
}

pub fn tr(sl: &str, mut stack: Vec<char>) -> bool {
    if stack.len() > sl.len() {
        return false;
    }
    for (i, ch) in sl.chars().enumerate() {
        // println!("{:?}", stack);

        match ch {
            '(' => stack.push(ch),
            ')' => {
                if let Some('(') = stack.pop() {
                    continue;
                } else {
                    return false;
                }
            }
            '*' => {
                let res = tr(&mut format!("{}{}", '(', &sl[i + 1..]), stack.clone());
                if res == true {
                    return true;
                } else if tr(&mut format!("{}{}", ')', &sl[i + 1..]), stack.clone()) == true {
                    return true;
                } else if tr(&mut format!("{}", &sl[i + 1..]), stack.clone()) == true {
                    return true;
                } else {
                    return false;
                }
            }
            _ => continue,
        }
    }

    stack.len() == 0
}

#[test]
fn test_parens() {
    // let i = "(*))".to_string();
    // let a = check_valid_string(i);
    // assert_eq!(a, true);

    // let i = "(*()".to_string();
    // let a = check_valid_string(i);
    // assert_eq!(a, true);

    // let i = "*()(())*()(()()((()(()()*)(*(())((((((((()*)(()(*)".to_string();
    // let a = check_valid_string(i);
    // assert_eq!(a, true);

    // let i = "(())((())()()(*)(*()(())())())()()((()())((()))(*".to_string();
    // let a = check_valid_string(i);
    // assert_eq!(a, false);

    // let i = "(((******))".to_string();
    // let a = check_valid_string(i);
    // assert_eq!(a, true);

    // let i = "(((((*(()((((*((**(((()()*)()()()*((((**)())*)*)))))))(())(()))())((*()()(((()((()*(())*(()**)()(())".to_string();
    // let a = check_valid_string(i);
    // assert_eq!(a, true);
}
