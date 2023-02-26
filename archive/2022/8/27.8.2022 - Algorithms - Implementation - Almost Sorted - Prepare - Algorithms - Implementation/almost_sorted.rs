//{"name":"Almost Sorted","group":"HackerRank - Algorithms - Implementation - Almost Sorted - Prepare - Algorithms - Implementation","url":"https://www.hackerrank.com/challenges/almost-sorted/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"STDIN   Function\n-----   --------\n2       arr[] size n = 2\n4 2     arr = [4, 2]\n","output":"yes\nswap 1 2\n"},{"input":"3\n3 1 2\n","output":"no\n"},{"input":"6\n1 5 4 3 2 6\n","output":"yes\nreverse 2 5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AlmostSorted"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);

    let mut sv = v.clone();
    sv.sort_unstable();
    let mut w = vec![];
    for i in 0..n {
        if v[i] != sv[i] {
            w.push(i);
        }
    }
    if w.len() == 0 {
        out_line!("yes");
    }
    else if w.len() == 2 {
        out_line!("yes");
        out_line!("swap", w[0]+1, w[1]+1);
    }
    else {
        let l = w.last().unwrap()+1;
        let start = &v[..w[0]];
        let mid = &v[w[0]..l];
        let end = &v[l..];
        let nv: Vec<&i32> = start.into_iter().chain(mid.into_iter().rev()).chain(end.into_iter()).collect();
        if nv.into_iter().map(|x| *x).collect::<Vec<i32>>() == sv {
            out_line!("yes");
            out_line!("reverse", w[0]+1, l);
        }
        else {
            out_line!("no");
        }
    }
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
