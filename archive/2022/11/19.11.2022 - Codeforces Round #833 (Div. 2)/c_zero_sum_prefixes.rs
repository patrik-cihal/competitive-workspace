//{"name":"C. Zero-Sum Prefixes","group":"Codeforces - Codeforces Round #833 (Div. 2)","url":"https://codeforces.com/contest/1748/problem/C","interactive":false,"timeLimit":1000,"tests":[{"input":"5\n5\n2 0 1 -1 0\n3\n1000000000 1000000000 0\n4\n0 0 0 0\n8\n3 0 2 -10 10 -30 30 0\n9\n1 0 0 1 -1 0 1 0 -1\n","output":"3\n1\n4\n4\n5\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CZeroSumPrefixes"}}}

use std::collections::BTreeMap;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i64>(n);

    let mut intervals = vec![];

    for i in 0..n {
        if v[i] == 0 {
            intervals.push(i);
        }
    }

    let mut answer = 0;

    let mut sum = 0;
    for i in 0..*intervals.first().unwrap_or(&n) {
        sum += v[i];
        if sum == 0 {
            answer += 1;
        }
    }

    for interval in intervals {
        let mut bm = BTreeMap::new();
        bm.insert(0, 1);
        let mut sum = 0;
        for j in (interval+1)..n {
            if v[j] == 0 {
                break;
            }
            sum += v[j];
            if let Some(val) = bm.get_mut(&sum) {
                *val += 1;
            }
            else {
                bm.insert(sum, 1);
            }
        }
        answer += bm.into_iter().map(|(k, v)| v).max().unwrap();
    }
    out_line!(answer);
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
