//{"name":"Journey to the Moon","group":"HackerRank - Algorithms - Graph Theory - Journey to the Moon - Prepare - Algorithms - Graph Theory","url":"https://www.hackerrank.com/challenges/journey-to-the-moon/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"5 3\n0 1\n2 3\n0 4\n","output":"6\n"},{"input":"4 1\n0 2\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"JourneyToTheMoon"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn dfs(g: &Vec<Vec<usize>>, visited: &mut Vec<bool>, v: usize) -> i64 {
    if visited[v] {
        return 0;
    }
    visited[v] = true;
    let mut result = 1;
    for n in &g[v] {
        result += dfs(g, visited, *n);
    }
    return result;
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();
    let m = input.read();
    let mut g = vec![vec![]; n];
    let mut visited = vec![false; n];
    for i in 0..m {
        let id1: usize = input.read();
        let id2: usize = input.read();
        g[id1].push(id2);
        g[id2].push(id1);
    }
    let mut groups = vec![];
    for i in 0..n {
        if !visited[i] {
            groups.push(dfs(&g, &mut visited, i));
        }
    }
    let mut size = n as i64;
    let mut answer = 0;
    for i in 0..groups.len() {
        size -= groups[i];
        answer += groups[i]*size;
    }
    out_line!(answer);
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
