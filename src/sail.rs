fn solve(prices: Vec<i32>) -> i32 {
    if prices.len() <= 1 {
        return 0;
    }

    let mut total = 0;
    let mut i = 0;

    while i < prices.len() - 1 {
        while i < prices.len() - 1 && prices[i + 1] <= prices[i] {
            i += 1;
        }

        if i == prices.len() - 1 {
            break;
        }

        let start = i;
        i += 1;

        while i < prices.len() && prices[i] >= prices[i - 1] {
            i += 1;
        }

        let end = i - 1;

        total += prices[end] - prices[start];
    }

    total
}

#[test]
fn test_sail() {
    let i = vec![7, 1, 5, 3, 6, 4];

    let res = solve(i);

    assert_eq!(res, 7);
}
