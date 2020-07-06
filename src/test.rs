use std::collections::{HashMap, HashSet, VecDeque};

pub struct IO<R, W: std::io::Write>(R, std::io::BufWriter<W>);

impl<R: std::io::Read, W: std::io::Write> IO<R, W> {
    pub fn new(r: R, w: W) -> IO<R, W> {
        IO(r, std::io::BufWriter::new(w))
    }
    pub fn write<S: ToString>(&mut self, s: S) {
        use std::io::Write;
        self.1.write(s.to_string().as_bytes()).unwrap();
    }
    pub fn read<T: std::str::FromStr>(&mut self) -> T {
        use std::io::Read;
        let buf = self
            .0
            .by_ref()
            .bytes()
            .map(|b| b.unwrap())
            .skip_while(|&b| b == b' ' || b == b'\n' || b == b'\r' || b == b'\t')
            .take_while(|&b| b != b' ' && b != b'\n' && b != b'\r' && b != b'\t')
            .collect::<Vec<_>>();
        unsafe { std::str::from_utf8_unchecked(&buf) }
            .parse()
            .ok()
            .expect("Parse error.")
    }
    pub fn vec<T: std::str::FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.read()).collect()
    }
    pub fn chars(&mut self) -> Vec<char> {
        self.read::<String>().chars().collect()
    }
}

struct Graph {
    vs: Vec<Vec<usize>>,
}

impl Graph {
    pub fn init(vs_c: usize) -> Self {
        let mut vs = Vec::with_capacity(vs_c as usize);
        for i in 1..=vs_c {
            let pair = Vec::new();
            vs.push(pair);
        }
        // let vs = vec![Vec::new(); vs_c];

        Graph { vs }
    }

    pub fn set_link(&mut self, from: usize, to: usize) {
        self.vs.get_mut(from).unwrap().push(to);
        self.vs.get_mut(to).unwrap().push(from);
    }

    pub fn check_link(&self, from: usize, to: usize) -> bool {
        self.vs.get(from).unwrap().contains(&to)
    }

    pub fn get_connected(&self, from: usize, depth: usize) -> usize {
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(from);
        let mut dist: HashMap<usize, usize> = HashMap::new();
        dist.insert(from, 0);

        let mut result: HashSet<usize> = HashSet::new();
        while !q.is_empty() {
            let u = q.pop_front().unwrap();

            let from_n = self.vs.get(u).unwrap();
            from_n.into_iter().for_each(|v| {
                if !dist.contains_key(v) {
                    dist.insert(*v, dist.get(&u).unwrap() + 1);
                    q.push_back(*v);
                }
            });
        }

        result.len()
    }

    pub fn bfs(&self, from: usize, to: usize) -> usize {
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut dest: HashMap<usize, usize> = HashMap::new();

        q.push_back(from);
        dest.insert(from, 0);

        while !q.is_empty() {
            let u = q.pop_front().unwrap();

            let vs = self.vs.get(u).unwrap();

            for v in vs {
                if !dest.contains_key(v) {
                    let l = dest.get(&u).unwrap() + 1;

                    if to == *v {
                        return l;
                    }

                    dest.insert(*v, l);
                    q.push_back(*v);
                }
            }
        }

        unreachable!()
    }
}

fn main() {
    let mut sc = IO::new(std::io::stdin(), std::io::stdout());

    let t: usize = sc.read();

    let answ: Vec<usize> = vec![2, 2];
    for t in 0..t {
        let inputs = sc.vec(2);

        let (v, l) = (inputs[0], inputs[1]);
        let mut graph = Graph::init(v);

        for _ in 0..l {
            let inputs: Vec<usize> = sc.vec(2);
            graph.set_link(inputs[0] - 1, inputs[1] - 1);
        }

        let len = graph.bfs(0, v - 1);
        println!("{}", len);
    }
}

fn is_happy(n: i32) -> bool {
    let mut map: HashMap<i32, i32> = HashMap::new();

    let mut buf: i32 = n;
    loop {
        buf = buf
            .to_string()
            .chars()
            .map(|d| d.to_digit(10).unwrap())
            .collect::<Vec<_>>()
            .into_iter()
            .fold(0 as i32, |acc, curr| acc + (curr.pow(2) as i32));

        let entry = map.entry(buf).or_insert(0);
        *entry += 1;

        if *entry > 1 {
            return false;
        }

        if buf == 1 {
            return true;
        }
    }

    unreachable!()
}

#[test]
fn foo() {
    // let sample = std::fs::File::open("conn.txt").unwrap();
    // let mut sc = IO::new(std::io::BufReader::new(sample), std::io::stdout());

    let a = is_happy(55);
    assert_eq!(a, false);

    let a = is_happy(19);
    assert_eq!(a, true);
}
