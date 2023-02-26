//{"name":"C. Hamiltonian Wall","group":"Codeforces - Educational Codeforces Round 139 (Rated for Div. 2)","url":"https://codeforces.com/contest/1766/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3\nWBB\nBBW\n1\nB\nB\n5\nBWBWB\nBBBBB\n2\nBW\nWB\n5\nBBBBW\nBWBBB\n6\nBWBBWB\nBBBBBB\n","output":"YES\nYES\nNO\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CHamiltonianWall"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Eq, PartialEq)]
enum PathType {
    Road,
    Bridge(bool)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read(); 
    let t: Vec<bool> = input.read::<String>().chars().map(|x| x=='B').collect();
    let b: Vec<bool> = input.read::<String>().chars().map(|x| x=='B').collect();

    let mut pos = 0;
    for i in 1..n {
        if t[i] != t[i-1] && !b[i] || b[i] != b[i-1] && !t[i] {
            out_line!("NO");
            return;
        }
    }

    let mut pos = 0;
    while pos < n && t[pos] && b[pos] {
        pos += 1;
    }

    let mut top = t[pos.min(t.len()-1)];
    while pos < n {
        if top != t[pos] {
            out_line!("NO");
            return;
        }
        while pos < n && !(t[pos] && b[pos]) {
            pos +=1;
        }
        while pos < n && (t[pos] && b[pos]) {
            top = !top;
            pos += 1;
        }
    }

    out_line!("YES");

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
