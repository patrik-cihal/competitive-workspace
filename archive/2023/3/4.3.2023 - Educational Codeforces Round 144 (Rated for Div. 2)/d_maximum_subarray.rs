//{"name":"D. Maximum Subarray","group":"Codeforces - Educational Codeforces Round 144 (Rated for Div. 2)","url":"https://codeforces.com/contest/1796/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n4 1 2\n2 -1 2 3\n2 2 3\n-1 2\n3 0 5\n3 2 4\n6 2 -8\n4 -1 9 -3 7 -8\n","output":"5\n7\n0\n44\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMaximumSubarray"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut k: usize = input.read();
    let mut x = input.read::<i64>();
    let a = input.read_vec::<i64>(n);

    if x > 0 { 
        x = -x;
    }
    else {
        k = n-k;
    }


    // start summing up
    // if at any point the left side of the subsequence becomes inefficient remove it
    // if the sequence is negative then restart

    let mut start = 0;
    let mut sum = 0;


    let mut answer = 0;
    let mut leftsum = 0;

    for i in 0..n {
        answer = answer.max(sum);

        sum += a[i]-x;

        if i-start+1 > k {
            let mut extra = i-start+1-k;
            leftsum += a[start+extra-1] + x;
            sum += 2*x;
            while (leftsum < 0 || a[start] < -x) && extra > 0 {
                leftsum -= x;
                leftsum -= a[start];
                sum -= x;
                sum -= a[start];
                start += 1;
                extra -= 1;
            }
            if extra == 0 {
                while start < n && start < i+1 && a[start]-x < 0 {
                    sum -= a[start];
                    sum += x;
                    start += 1;
                }
            }
        }

        if sum < 0 {
            start = i+1;
            sum = 0;
            leftsum = 0;
        }
    }

    answer = answer.max(sum);

    out_line!(answer);

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
