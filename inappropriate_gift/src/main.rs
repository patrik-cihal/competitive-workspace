//{"name":"inappropriate_gift","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"inappropriate_gift"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let c: u32 = input.read();
    let l: usize = input.read();
    let r: usize = input.read();

    // make a dp based on number of divisions
    // for each calculate the number of zeros and ones
    // using the relationship a(n) = 2*a(n+1)

    struct DPUnit {
        val: u32,
        count0: u32,
    }

    let mut dp: Vec<DPUnit> = vec![];
    let mut pc = c;

    // divide c while higher than two and fill dp

    // once dp is completed we find the number of 1s until the start and number of 1s after the end
    // answer is the entire count - total number of 1s after/before intervals

    while pc > 0 {
        dp.push(DPUnit {
            val: pc,
            count0: 0,
        });
        pc /= 2;
    }

    for i in (0..dp.len() - 1).rev() {
        dp[i].count0 = dp[i + 1].count0 * 2 + 1 - (dp[i].val % 2);
    }

    let bounds = vec![l-1, (dp[0].count0+dp[0].val) as usize-r];

    // count left offset and number of 1s until we reach start
    // go from highest dp to lowest
    // whenever value lower or equal move offset

    let mut rev_answer = 0;


    for i in 0..2 {
        let mut o = bounds[i];
        let mut j = 1;
        
        while o > 0 {
            while dp[j].count0+dp[j].val > o as u32 {
                j+=1;
            }
            o -= (dp[j].count0+dp[j].val) as usize;
            rev_answer += dp[j].val;
            if o != 0 {
                o -= 1;
                rev_answer += dp[j-1].val%2;
            }
        }
    }

    out_line!(dp[0].val-rev_answer);
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
