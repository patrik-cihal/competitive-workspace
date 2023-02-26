//{"name":"D. Bit Guessing Game","group":"Codeforces - Codeforces Round #846 (Div. 2)","url":"https://codeforces.com/contest/1780/problem/D","interactive":true,"timeLimit":1000,"tests":[{"input":"3\n\n1\n\n0\n\n1\n\n1\n\n0\n\n2\n\n1\n\n0\n","output":"- 1\n\n! 1\n\n- 1\n\n- 1\n\n! 2\n\n- 2\n\n- 1\n\n! 3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBitGuessingGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

use std::io::stdin;


fn solve(input: &mut Input, _test_case: usize) {

    let mut n = input.read(); 
    let mut answer = 0;

    let mut interact = |x: u32| {
        println!("- {}", x);
        input.read::<u32>()
    };

    let bits_left = n;

    let mut cur_bit = 0;

    for i in 0..bits_left {
        let cnt = interact(1 << cur_bit);

        if cnt < n {
            answer += 1<<cur_bit;
            cur_bit += 1;
        }
        else {
            cur_bit += cnt-n+1;
            answer += 1<<cur_bit;
            cur_bit += 1;
        }

        n = cnt;
    }

    println!("! {}", answer);
    
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
