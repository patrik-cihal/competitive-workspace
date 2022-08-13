//{"name":"Abbreviation","group":"HackerRank - Algorithms - Dynamic Programming - Abbreviation - Prepare - Algorithms - Dynamic Programming","url":"https://www.hackerrank.com/challenges/abbr/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"1\ndaBcd\nABC\n","output":"YES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Abbreviation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let a: Vec<char> = input.read::<String>().chars().collect();
    let mut b: Vec<char> = input.read::<String>().chars().collect();
    b.push('1');

    #[derive(Clone, Copy, PartialEq, Eq)]
    enum DPState {
        Possible,
        Imbossible,
        ToExplore
    }

    let mut dp = vec![vec![DPState::ToExplore; b.len()]; a.len()];

    struct RecFn<'s> { f: &'s dyn Fn(&RecFn, usize, usize, &mut Vec<Vec<DPState>>) -> bool }
    let go = RecFn {
        f: &|rf: &RecFn, i: usize, j: usize, dp: &mut Vec<Vec<DPState>>| -> bool {
            if i == a.len() {
                j == b.len()-1
            }
            else {
                match dp[i][j] {
                    DPState::Imbossible => false,
                    DPState::Possible => true,
                    DPState::ToExplore => {
                        let result;
                        if a[i].is_uppercase() {
                            if a[i] != b[j] {
                                result = false;
                            }
                            else {
                                result = (rf.f)(rf, i+1, j+1, dp);
                            }
                        }
                        else {
                            result = (a[i].to_uppercase().next().unwrap() == b[j] && (rf.f)(&rf, i+1, j+1, dp))
                                | (rf.f)(&rf, i+1, j, dp);
                        }

                        dp[i][j] = if result {DPState::Possible} else {DPState::Imbossible};
                        result
                    }
                }
            }
        }
    };
    if (go.f)(&go, 0, 0, &mut dp) {
        out_line!("YES");
    }else {
        out_line!("NO");
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
