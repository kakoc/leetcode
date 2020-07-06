use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
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
        // println!("{} {}", from, to);
        if !self.vs.get_mut(from).unwrap().contains(&to) {
            self.vs.get_mut(from).unwrap().push(to);
        }
        // self.vs.get_mut(from).unwrap().push(to);
        if !self.vs.get_mut(to).unwrap().contains(&from) {
            self.vs.get_mut(to).unwrap().push(from);
        }
    }

    pub fn check_link(&self, from: usize, to: usize) -> bool {
        self.vs.get(from).unwrap().contains(&to)
    }

    pub fn get_connected(&self, from: usize, depth: usize) -> Vec<usize> {
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

        // result.len()
        result.into_iter().collect()
    }

    pub fn bfs(&self, from: usize, mut set: HashSet<usize>) -> HashSet<usize> {
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

                    // if to == *v {
                    //     return l;
                    // }
                    set.insert(*v);

                    dest.insert(*v, l);
                    q.push_back(*v);
                }
            }
        }

        set
    }
}
pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
    let mut graph = Graph::init(grid.len() * grid.get(0).unwrap().len());

    // println!("{:#?}", graph);
    let mut total = 0;
    let row_len = grid.get(0).unwrap().len();
    for i in 0..grid.len() {
        for j in 0..grid.get(i).unwrap().len() {
            let mut l = false;
            let mut r = false;
            let mut t = false;
            let mut b = false;

            if let Some('0') = grid.get(i).unwrap().get(j) {
                continue;
            }
            if j > 0 {
                if let Some('1') = grid.get(i).unwrap().get(j - 1) {
                    graph.set_link(i * row_len + j, i * row_len + j - 1);
                } else {
                    l = true;
                }
            }
            if i > 0 {
                if let Some(row) = grid.get(i - 1) {
                    if let Some('1') = row.get(j) {
                        graph.set_link(i * row_len + j, (i - 1) * row_len + j);
                    } else {
                        t = true;
                    }
                }
            }
            if let Some('1') = grid.get(i).unwrap().get(j + 1) {
                graph.set_link(i * row_len + j, i * row_len + j + 1);
            } else {
                r = true;
            }
            if let Some(row) = grid.get(i + 1) {
                if let Some('1') = row.get(j) {
                    graph.set_link(i * row_len + j, (i + 1) * row.len() + j);
                } else {
                    b = true;
                }
            }

            // if t == true && l == true && r == true && b == true {
            //     total += 1;
            // }
        }
    }

    let mut visited: HashSet<usize> = HashSet::new();
    for i in 0..grid.len() {
        for j in 0..grid.get(i).unwrap().len() {
            if let Some('1') = grid.get(i).unwrap().get(j) {
                if !visited.contains(&(i * row_len + j)) {
                    visited = graph.bfs(i * row_len + j, visited);
                    total += 1;
                }
            }
        }
    }

    // println!("{}", total);
    // println!("{:#?}", graph);
    // println!("{}", total);

    total
}

#[test]
fn test_islands() {
    let i = vec![
        vec!['1', '1', '0', '0', '0'],
        vec!['1', '1', '0', '0', '0'],
        vec!['0', '0', '1', '0', '0'],
        vec!['0', '0', '0', '1', '1'],
    ];

    let a = num_islands(i);
    assert_eq!(a, 3);
}
