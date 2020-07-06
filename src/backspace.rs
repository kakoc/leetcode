pub fn backspace_compare(s: String, t: String) -> bool {
    let mut s_last_ptr: i32 = (s.len() - 1) as i32;
    let mut t_last_ptr: i32 = (t.len() - 1) as i32;
    let mut s_l = s.chars().nth(s_last_ptr as usize).unwrap() != '#';
    let mut t_l = t.chars().nth(t_last_ptr as usize).unwrap() != '#';

    let mut s_sw: i32 = 0;
    if !s_l {
        s_sw = 1;
    }

    let mut t_sw: i32 = 0;
    if !t_l {
        t_sw = 1;
    }

    let mut ll;
    let mut tl;
    for i in 0..s.len() {
        while s_sw > 0 && s_last_ptr >= 0 {
            s_last_ptr -= 1;
            s_sw -= 1;

            if is_sw(&s, s_last_ptr) {
                s_sw += 1;
            }
            s_last_ptr -= 1;
            if is_sw(&s, s_last_ptr) {
                s_sw += 1;
            }
        }

        while t_sw > 0 && t_last_ptr >= 0 {
            t_last_ptr -= 1;
            t_sw -= 1;

            if is_sw(&t, t_last_ptr) {
                t_sw += 1;
            }
            t_last_ptr -= 1;
            if is_sw(&t, t_last_ptr) {
                t_sw += 1;
            }
        }

        if s_last_ptr < 0 {
            ll = '#';
        } else {
            ll = s.chars().nth(s_last_ptr as usize).unwrap();
        }

        if t_last_ptr < 0 {
            tl = '#';
        } else {
            tl = t.chars().nth(t_last_ptr as usize).unwrap();
        }

        if ll != tl {
            return false;
        }

        s_last_ptr -= 1;
        t_last_ptr -= 1;
        if is_sw(&s, s_last_ptr) {
            s_sw = 1;
        }
        if is_sw(&t, t_last_ptr) {
            t_sw = 1;
        }
    }

    true
}

pub fn is_l(s: &String, n: i32) -> bool {
    if n < 0 {
        return false;
    }

    if let Some(ch) = s.chars().nth(n as usize) {
        return ch != '#';
    }

    false
}

pub fn is_sw(s: &String, n: i32) -> bool {
    !is_l(&s, n)
}

#[test]
fn test_backspace() {
    // let S = "ab##".to_string();
    // let T = "c#d#".to_string();

    // let a = backspace_compare(S, T);

    // assert_eq!(a, false);

    // let S = "ab#c".to_string();
    // let T = "ad#c".to_string();

    // let a = backspace_compare(S, T);
    // assert_eq!(a, true);

    // let S = "ab##".to_string();
    // let T = "c#d#".to_string();

    // let a = backspace_compare(S, T);
    // assert_eq!(a, true);

    let S = "xywrrmp".to_string();
    let T = "xywrrmu#p".to_string();

    let a = backspace_compare(S, T);
    assert_eq!(a, true);
}
