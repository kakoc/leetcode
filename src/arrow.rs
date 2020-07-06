fn one(a: i32, b: i32) -> i32 {
    a + b
}

fn two(a: i32, b: i32) -> i32 {
    a + b
}

fn three(a: i32) -> i32 {
    a
}

fn four(a: i32, b: i32) -> i32 {
    b - a as i32
}

macro_rules! pipe {
    (-> $init:expr , $($f:expr ; ($($var:expr),*))*) => {
	{
	    let mut result = $init;

	    $(
		result = $f(result, $($var, )*);
	    )*

	    result
	}
    };
    (->> $init:expr , $($f:expr ; ($($var:expr),*))*) => {
	{
	    let mut result = $init;

	    $(
		result = $f($($var),* , result);
	    )*

	    result
	}
    };
}

#[test]
fn test_arrow() {
    let initial_data = 5;
    let a = 5;
    let b = 7;

    let v = pipe![
    -> initial_data,

       one   ; (a)
       two   ; (b)
       three ; ()
    ];

    let v = pipe![
    ->> initial_data,

        one  ; (1)
        four ; (b)
    ];

    assert_eq!(v, -1);
}
