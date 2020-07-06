#[derive(Clone, Debug)]
struct Data {
    sum: i32,
    pref: i32,
    suff: i32,
    ans: i32,
}

struct SegmentTree {
    vs: Vec<Data>,
}

impl Default for Data {
    fn default() -> Self {
        Data {
            sum: 0,
            pref: 0,
            suff: 0,
            ans: 0,
        }
    }
}

impl SegmentTree {
    pub fn init(n: usize) -> Self {
        SegmentTree {
            vs: vec![Data::default(); 4 * n],
        }
    }

    pub fn build(&mut self, a: &Vec<i32>, v: i32, tl: usize, tr: usize) {
        if tl == tr {
            self.vs[v as usize] = make_data(a[tl as usize]);
        } else {
            let tm = (tl + tr) / 2;
            self.build(&a, v * 2, tl, tm);
            self.build(&a, v * 2 + 1, tm + 1, tr);
            self.vs[v as usize] = combine(
                self.vs[(v * 2) as usize].clone(),
                self.vs[(v * 2 + 1) as usize].clone(),
            );
        }
    }

    pub fn query(&self, v: usize, tl: usize, tr: usize, l: usize, r: usize) -> Data {
        if l > r {
            return make_data(0);
        }
        if l == tl && r == tr {
            return self.vs[v].clone();
        }
        let tm = (tl + tr) / 2;

        combine(
            self.query(v * 2, tl, tm, l, std::cmp::min(r, tm)),
            self.query(v * 2 + 1, tm + 1, tr, std::cmp::max(l, tm + 1), r),
        )
    }
}

fn combine(l: Data, r: Data) -> Data {
    let mut res: Data = Data::default();

    res.sum = l.sum + r.sum;
    res.pref = std::cmp::max(l.pref, l.sum + r.pref);
    res.suff = std::cmp::max(r.suff, r.sum + l.suff);
    res.ans = std::cmp::max(std::cmp::max(l.ans, r.ans), l.suff + r.pref);

    res
}

fn make_data(val: i32) -> Data {
    let mut res: Data = Data::default();

    res.sum = val;
    res.pref = std::cmp::max(0, val);
    res.suff = res.pref;
    res.ans = res.pref;

    res
}

#[test]
fn segment_tree_test() {
    // let input = vec![-2, 1, -3, 4, -1, 2, 1, -5, 4];
    let input = vec![1, 2, 3, -2];

    if input.iter().all(|&x| x < 0) {
        input.iter().max().unwrap();
    }
    // let input = vec![-2, -1, -3, -4, -1, -2, -1, -5, -4];

    let mut tree = SegmentTree::init(input.len());
    tree.build(&input, 1, 0, input.len() - 1);
    let ans = tree.query(1, 0, input.len() - 1, 0, input.len() - 1);
    println!("{:?}", ans);

    assert_eq!(6, ans.ans);
}
