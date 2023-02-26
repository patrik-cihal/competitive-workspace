//{"name":"C. Robot in a Hallway","group":"Codeforces - Educational Codeforces Round 133 (Rated for Div. 2)","url":"https://codeforces.com/contest/1716/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n0 0 1\n4 3 2\n5\n0 4 8 12 16\n2 6 10 14 18\n4\n0 10 10 10\n10 10 10 10\n2\n0 0\n0 0\n","output":"5\n19\n17\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CRobotInAHallway"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::BTreeMap;

fn solve(input: &mut Input, _test_case: usize) {
    let m: usize = input.read();
    let a: Vec<i32> = input.read_vec(m);
    let b: Vec<i32> = input.read_vec(m);


    let mut v = a.clone();
    for i in (0..m).rev() {
        v.push(b[i]);
    }

    let mut lp = vec![v[0]];
    for i in 1..m*2 {
        lp.push((v[i]-i as i32).max(lp[i-1]));
    }
    let mut rp: Vec<i32> = vec![*v.last().unwrap()];
    for i in 1..m*2 {
        rp.push(*rp.last().unwrap().max(&(v[m*2-i-1]-i as i32)));
    }

    let mut app = vec![0; m];

    for i in 0..m {
        if i%2 == 0 {
            app[i] = rp[m*2-i-1];
        }else {
            app[i] = lp[m*2-i-1];
        }
    }

    let mut result = vec![0; m];
    let mut ans = i32::MAX;
    let mut time = -1;
    for i in 0..m {
        time += 1;
        if i%2 == 0 {
            time += (a[i]-time).max(0);
            time += 1;
            time += (b[i]-time).max(0);
        }
        else {
            time += (b[i]-time).max(0);
            time += 1;
            time += (a[i]-time).max(0);
        }
        result[i] = time+(app[i]-time).max(0);
    }
    out_line!(*result.iter().min().unwrap());
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
