//{"name":"stavba_portalu","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"stavba_portalu"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let mut p = vec![vec![]; 1000];
    let mut v = vec![];
    for _ in 0..n {
        let l = input.read::<i32>();
        let t = input.read::<usize>() - 1;
        p[t].push(l);
        v.push((t, l));
    }

    for v in &mut p {
        v.sort();
    }

    for (t, length) in v {
        let mut nb = (0, 0);
        for i in 0..1000 {
            if i == t {
                continue;
            }
            let j = p[i].partition_point(|&x| x <= length);
            if j == 0 {
                continue;
            }
            nb = nb.max((p[i][j - 1], i));
        }
        let mut nnb = (0, 0);
        for i in 0..1000 {
            if i == t || i == nb.1 {
                continue;
            }
            let j = p[i].partition_point(|&x| x < length);
            if j == 0 {
                continue;
            }
            nnb = nnb.max((p[i][j - 1], i));
        }
        if nnb.0 + nb.0 > length {
            println!("ANO");
            return;
        }
    }
    println!("NE");
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
