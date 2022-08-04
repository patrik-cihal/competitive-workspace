//{"name":"C. Virus","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"8\n10 3\n3 6 8\n6 2\n2 5\n20 3\n3 7 12\n41 5\n1 11 21 31 41\n10 5\n2 4 6 8 10\n5 5\n3 2 5 4 1\n1000000000 1\n1\n1000000000 4\n1 1000000000 10 16\n","output":"7\n5\n11\n28\n9\n5\n2\n15\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CVirus"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();
    let mut a: Vec<i64> = input.read_vec(m);
    a.sort();

    if m == 1 {
        out_line!(2);
        return;
    }

    let mut d: Vec<i64> = vec![(n as i64)-a.last().unwrap()+a[0]-1];

    for i in 1..m {
        d.push(a[i]-a[i-1]-1);
    }
    d.sort();
    d.reverse();
    let mut t = 0;
    let mut result = 0;
    for i in 0..d.len() {
        let h = d[i]-t*2;
        if h<=0 {
            break;
        }
        if h == 1 {
            result += 1;
            t += 1;
        }
        else {
            result += h-1;
            t += 2;
        }
    }
    out_line!(n as i64-result);
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
