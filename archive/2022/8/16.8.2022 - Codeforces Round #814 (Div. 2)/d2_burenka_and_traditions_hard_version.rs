//{"name":"D2. Burenka and Traditions (hard version)","group":"Codeforces - Codeforces Round #814 (Div. 2)","url":"https://codeforces.com/contest/1719/problem/D2","interactive":false,"timeLimit":1000,"tests":[{"input":"7\n4\n5 5 5 5\n3\n1 3 2\n2\n0 0\n3\n2 5 7\n6\n1 2 3 3 2 1\n10\n27 27 34 32 2 31 23 56 52 4\n5\n1822 1799 57 23 55\n","output":"2\n2\n0\n2\n4\n7\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"D2BurenkaAndTraditionsHardVersion"}}}

use std::collections::{BTreeMap, BTreeSet};
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let v: Vec<i32> = input.read_vec(n);

    let mut mp = BTreeSet::new();
    let mut result = 0;

    // create map where we store v[i]^passed
    // if new element is in map then add to result

    let mut p = 0;
    mp.insert(0);
    for i in 0..n {
        if mp.contains(&(v[i]^p)) {
            result += 1;
            mp = BTreeSet::new();
            mp.insert(0);
            p = 0;
        }
        else {
            p ^= v[i];
            mp.insert(p);
        }
    }

    out_line!(n-result);
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
