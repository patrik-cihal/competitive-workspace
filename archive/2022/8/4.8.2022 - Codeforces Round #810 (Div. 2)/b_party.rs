//{"name":"B. Party","group":"Codeforces - Codeforces Round #810 (Div. 2)","url":"https://codeforces.com/problemset/problem/1711/B","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n1 0\n1\n3 1\n2 1 3\n1 3\n5 5\n1 2 3 4 5\n1 2\n1 3\n1 4\n1 5\n2 3\n5 5\n1 1 1 1 1\n1 2\n2 3\n3 4\n4 5\n5 1\n","output":"0\n2\n3\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BParty"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let a: Vec<i32> = input.read_vec(n);
    let mut count = vec![0; n];
    let mut edges = vec![];
    for i in 0..m {
        let v1: usize = input.read();
        let v2: usize = input.read();
        count[v1-1] += 1;
        count[v2-1] += 1;
        edges.push((v1-1, v2-1));
    }

    let mut result = 0;
    let sum = count.iter().sum::<i32>()/2;
    if sum % 2 == 0 {
        out_line!(result);
    }else {
        result = 100000;
        for i in 0..n {
            if count[i]%2 == 1 {
                result = result.min(a[i]);
            }
        }
        for i in 0..m {
            if count[edges[i].0]%2 == 0 && count[edges[i].1]%2 == 0 {
                result = result.min(a[edges[i].0] + a[edges[i].1]);
            }
        }
        out_line!(result);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
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
