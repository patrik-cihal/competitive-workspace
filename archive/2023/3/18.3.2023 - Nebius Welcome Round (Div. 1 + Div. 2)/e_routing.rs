//{"name":"E. Routing","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/E","interactive":false,"timeLimit":2000,"tests":[{"input":"6 7\n1 2\n2 3\n3 1\n4 5\n5 6\n4 6\n2 5\n","output":"Yes\n2 5 2 5 2 5\n"},{"input":"3 2\n1 2\n2 3\n","output":"Yes\n2 1 2\n"},{"input":"4 4\n1 3\n2 3\n4 1\n4 2\n","output":"Yes\n3 3 1 1\n"},{"input":"6 5\n1 2\n2 3\n3 4\n4 5\n5 6\n","output":"No\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ERouting"}}}

use std::collections::{BTreeSet, BTreeMap};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn go(start: usize, cur: usize, dp: &mut Vec<Vec<BTreeMap<usize, (bool, usize)>>>, mut visited: usize, parent: usize, g: &Vec<Vec<usize>>) -> bool {
    visited |= 1<<cur;

    let visited_all = visited.count_ones() as usize == g.len();

    if let Some(val) = dp[cur][start].get(&visited) {
        val.0
    }
    else {
        let mut result = (false, g.len());
        for &v in &g[cur] {
            if visited & (1<<v) == 0 {
                if go(start, v, dp, visited, cur, g) {
                    result = (true, v);
                    break;
                }
            }
            else if v == parent {
                if go(start, parent, dp, visited, g.len(), &g) {
                    result = (true, parent);
                    break;
                }
            }
            if visited_all && v == start {
                result = (true, start);
                break;
            }
        }
        dp[cur][start].insert(visited, result);
        result.0
    }
}

fn construct_path(start: usize, cur: usize, dp: &Vec<Vec<BTreeMap<usize, (bool, usize)>>>, mut visited: usize, path: &mut Vec<usize>) {
    visited |= 1<<cur;

    if cur == start && visited.count_ones() as usize == dp.len() {
        return;
    }

    let dp_result = dp[cur][start][&visited];

    path[cur] = dp_result.1;

    if dp_result.0 == false {
        panic!("wth");
    }

    construct_path(start, dp_result.1, &dp, visited, path);
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();

    let mut g = vec![vec![]; n];

    for i in 0..m {
        let (v1, v2): (usize, usize) = input.read();

        g[v1-1].push(v2-1);
        g[v2-1].push(v1-1);
    }

    let mut dp = vec![vec![BTreeMap::<usize, (bool, usize)>::new(); n]; n];

    for v1 in 0..n {
        for v2 in &g[v1] {
            if go(v1, *v2, &mut dp, 1<<v1, v1, &g) {
                out_line!("YES");
                let mut path = vec![0; n];
                path[v1] = *v2;
                construct_path(v1, *v2, &dp, 1<<v1, &mut path);
                out_line!(path.into_iter().map(|x| x+1).collect::<Vec<usize>>());
                return;
            }
        } 
    } 
    
    out_line!("NO");
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
