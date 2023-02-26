//{"name":"Password Cracker","group":"HackerRank - Algorithms - Recursion - Password Cracker - Prepare - Algorithms - Recursion","url":"https://www.hackerrank.com/challenges/password-cracker/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n6\nbecause can do must we what\nwedowhatwemustbecausewecan\n2\nhello planet\nhelloworld\n3\nab abcd cd\nabcd\n","output":"we do what we must because we can\nWRONG PASSWORD\nab cd\n"},{"input":"3\n4\nozkxyhkcst xvglh hpdnb zfzahm\nzfzahm\n4\ngurwgrb maqz holpkhqx aowypvopu\ngurwgrb\n10\na aa aaa aaaa aaaaa aaaaaa aaaaaaa aaaaaaaa aaaaaaaaa aaaaaaaaaa\naaaaaaaaaab\n","output":"zfzahm\ngurwgrb\nWRONG PASSWORD\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"PasswordCracker"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::misc::recursive_function::{Callable, RecursiveFunction};


#[derive(Copy, Clone, Eq, PartialEq)]
enum DPState {
    ToExplore,
    Possible(usize),
    Impossible
}


fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let p = input.read_vec::<String>(n);

    let attempt = &input.read::<String>();
    let m = attempt.len();

    let mut ps = vec![vec![]; m];
    for i in 0..p.len() {
        if p[i].len() > attempt.len() {
            continue;
        }
        for j in 0..(attempt.len()-p[i].len()+1) {
            if p[i] == attempt[j..p[i].len()+j] {
                ps[j].push(i);
            }
        }
    }

    let mut dp = vec![DPState::ToExplore; m+1];
    dp[m] = DPState::Possible(n);

    let mut go = RecursiveFunction::new(|f, i: usize| -> bool {
        match dp[i] {
            DPState::Possible(_) => true,
            DPState::Impossible => false,
            DPState::ToExplore => {
                for pi in &ps[i] {
                    if f.call(i+p[*pi].len()) {
                        dp[i] = DPState::Possible(*pi);
                        return true;
                    }
                }
                dp[i] = DPState::Impossible;
                false
            }
        }
    });

    if go.call(0) {
        let mut i = 0;
        if let DPState::Possible(j) = dp[i] {
            out!(p[j]);
            i += p[j].len();
        }
        while i != m {
            if let DPState::Possible(j) = dp[i] {
                out!("", p[j]);
                i += p[j].len();
            }
        }
        out_line!();
    }
    else {
        out_line!("WRONG PASSWORD");
    }
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
