//{"name":"C. Sequence Master","group":"Codeforces - Codeforces Round 858 (Div. 2)","url":"https://codeforces.com/contest/1806/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"4\n1\n6 9\n2\n1 2 2 1\n2\n-2 -2 2 2\n4\n-3 -2 -1 0 1 2 3 4\n","output":"3\n2\n5\n13\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CSequenceMaster"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, minim};

fn dist(p: &Vec<i64>, q: &Vec<i64>) -> i64 {
    let mut result = 0;
    for i in 0..p.len() {
        result += (p[i]-q[i]).abs();
    }
    result
}


fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let n = n*2;

    let mut p = input.read_vec::<i64>(n);

    p.sort();
    p.reverse();


    let mut ans = dist(&p, &vec![0; n]);

    if n == 2 {
        minim!(ans, (p[0]-p[1]).abs());
    }
    if n == 4 {
        minim!(ans, dist(&p, &vec![2, 2, 2, 2]));
    }
    if n % 4 == 0 {
        let mut q = vec![(n/2) as i64];
        q.extend(vec![-1; n-1]);

        minim!(ans, dist(&q, &p));
    }
    out_line!(ans);
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
