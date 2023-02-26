//{"name":"D. 2+ doors","group":"Codeforces - Codeforces Round #816 (Div. 2)","url":"https://codeforces.com/contest/1715/problem/D","interactive":false,"timeLimit":1500,"tests":[{"input":"4 3\n1 2 3\n1 3 2\n4 1 2\n","output":"0 3 2 2\n"},{"input":"1 0\n","output":"0\n"},{"input":"2 1\n1 1 1073741823\n","output":"1073741823 0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2Doors"}}}

use std::mem::swap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Default, Clone)]
struct Edge {
    target: usize,
    weight: i32
}

#[derive(Default, Clone)]
struct Vertex {
    l_edges: Vec<Edge>,
    r_edges: Vec<Edge>,
    max_val: i32,
    min_val: i32
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let q: usize = input.read();


    let mut v = vec![Vertex::default(); n];

    for _ in 0..q {
        let mut j = input.read::<usize>()-1;
        let mut k = input.read::<usize>()-1;

        if j>k {
            swap(&mut j, &mut k);
        }

        let weight = input.read::<i32>();

        v[j].r_edges.push(Edge {
            target: k,
            weight
        });
        v[k].l_edges.push(Edge {
            target: j,
            weight
        });
    }

    for i in 0..n {
        let mut max_val = i32::MAX;
        let mut min_val = 0;
        for edge in v[i].l_edges.iter().chain(v[i].r_edges.iter()) {
            if edge.target == i {
                max_val = edge.weight;
                min_val = edge.weight;
                break;
            }
            max_val &= edge.weight;
        }
        v[i].max_val = max_val;
        v[i].min_val = min_val;
    }

    for i in 0..n {
        for edge in v[i].r_edges.clone() {
            v[i].min_val |= edge.weight & !v[edge.target].max_val;
        }
        for edge in v[i].r_edges.clone() {
            v[edge.target].min_val |= edge.weight & !v[i].min_val;
        }
    }

    let mut answer = vec![0;n];
    for i in 0..n {
        answer[i] = v[i].min_val;
    }
    out_line!(answer);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = 1;
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
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
