//{"name":"D. Tournament Countdown","group":"Codeforces - Codeforces Round #812 (Div. 2)","url":"https://codeforces.com/problemset/problem/1713/D","interactive":true,"timeLimit":2000,"tests":[{"input":"1\n3\n\n2\n\n0\n\n2\n","output":"\n? 1 4\n\n? 1 6\n\n? 5 7\n\n! 7\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DTournamentCountdown"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = 2usize.pow(input.read::<u32>());
    let mut v: Vec<usize> = (1..n+1).collect();

    // make interactions until length of v is higher than 2

    while v.len() > 2 {
        let mut nv = vec![];
        // perform interaction for all given quadrlets and assign the winner to new v
        for i in (0..v.len()).step_by(4) {
            // check if v[i] beats v[i+2]
            println!("? {} {}", v[i], v[i+2]);
            let outcome: u8 = input.read();
            if outcome == 1 {
                // push winner between v[i] and v[i+3]
                println!("? {} {}", v[i], v[i+3]);
                if input.read::<u8>() == 1 {
                    nv.push(v[i]);
                }
                else {
                    nv.push(v[i+3]);
                }
            }
            else if outcome == 2 {
                // otherwise winner between v[i+1] and v[i+2]
                println!("? {} {}", v[i+1], v[i+2]);
                if input.read::<u8>() == 1 {
                    nv.push(v[i+1]);
                }
                else {
                    nv.push(v[i+2]);
                }
            }
            else {
                // if no one won push winner between v[i+1] and v[i+3]
                println!("? {} {}", v[i+1], v[i+3]);
                if input.read::<u8>() == 1 {
                    nv.push(v[i+1]);
                }
                else {
                    nv.push(v[i+3]);
                }
            }
        }
        v = nv;
    }


    // if length of v is equal to 2 shrink it too 1
    if v.len() == 2 {
        println!("? {} {}", v[0], v[1]);
        if input.read::<u8>() == 1 {
            v = vec![v[0]];
        }
        else {
            v = vec![v[1]];
        }
    }

    // print v[0]
    println!("! {}", v[0]);
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
