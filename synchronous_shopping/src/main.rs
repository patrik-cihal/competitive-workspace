//{"name":"Synchronous Shopping","group":"HackerRank - Algorithms - Graph Theory","url":"https://www.hackerrank.com/challenges/synchronous-shopping/problem","interactive":false,"timeLimit":4000,"tests":[{"input":"5 5 5\n1 1\n1 2\n1 3\n1 4\n1 5\n1 2 10\n1 3 10\n2 4 10\n3 5 10\n4 5 10\n","output":"30\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"SynchronousShopping"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::cmp::Ordering;
use std::collections::BinaryHeap;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let k: usize = input.read();

    #[derive(Clone)]
    struct Edge {
        weight: i32,
        target: usize
    }

    #[derive(Default, Clone)]
    struct Node {
        mask: usize,
        conn: Vec<Edge>
    }

    let mut g = vec![Node::default(); n];

    for i in 0..n {
        let t: usize = input.read();
        for _ in 0..t {
            let ft: usize = input.read();
            g[i].mask += 1 << ft-1;
        }
    }

    for _ in 0..m {
        let v1 = input.read::<usize>()-1;
        let v2 = input.read::<usize>()-1;
        let weight: i32 = input.read();

        g[v1].conn.push(Edge {target: v2, weight});
        g[v2].conn.push(Edge {target: v1, weight});
    }

    let mut dist = vec![vec![i32::MAX; 2usize.pow(k as u32)]; n];
    let mut heap = BinaryHeap::new();

    #[derive(PartialEq, Eq)]
    struct DjikstraNode {
        dist: i32,
        index: usize,
        mask: usize,
    }

    impl Ord for DjikstraNode {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.dist.cmp(&other.dist).reverse()
        }
    }

    impl PartialOrd for DjikstraNode {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    

    dist[0][g[0].mask] = 0;
    heap.push(DjikstraNode {dist: 0, index: 0, mask: g[0].mask});


    while let Some(dnode) = heap.pop() {
        if dnode.dist > dist[dnode.index][dnode.mask] {
            continue;
        }

        let node = &g[dnode.index];
        for edge in &node.conn {
            let new_dnode = DjikstraNode {
                mask: dnode.mask | g[edge.target].mask,
                dist: dnode.dist+edge.weight,
                index: edge.target
            };
            if new_dnode.dist < dist[edge.target][new_dnode.mask] {
                dist[edge.target][new_dnode.mask] = new_dnode.dist;
                heap.push(new_dnode);
            }
        }
    }

    let mut ans = i32::MAX;
    let opt = 2usize.pow(k as u32);
    for i in 0..opt {
        for j in 0..opt {
            if i|j == opt-1 {
                ans = ans.min(dist[n-1][i].max(dist[n-1][j]));
            }
        }
    }
    out_line!(ans);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}


//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
