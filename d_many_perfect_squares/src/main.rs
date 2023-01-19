//{"name":"D. Many Perfect Squares","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, based on VK Cup 2022 - Elimination Round)","url":"https://codeforces.com/contest/1782/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n5\n1 2 3 4 5\n5\n1 6 13 22 97\n1\n100\n5\n2 5 10 17 26\n","output":"2\n5\n1\n2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DManyPerfectSquares"}}}

use std::collections::{BTreeMap, BTreeSet};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn perfect_square(x: u128) -> bool
{
    let (mut left, mut right) = (1, x);
 
    while left <= right {
        let mid: u128= (left + right) / 2;
 
        if mid * mid == x {
            return true;
        }
        else if mid * mid < x {
            left = mid + 1;
        }
        else {
            right = mid - 1;
        }
    }
    return false;
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a: Vec<u64> = input.read_vec(n);
    a.sort();

    let mut results = BTreeMap::new();

    for i in 0..n {
        for j in i+1..n {
            if (a[j]-a[i]) % 2 == 0 {
                continue;
            }
            let offset = ((a[j]-a[i])/2).pow(2);

            if offset < a[i] {
                continue;
            }

            let offset = offset-a[i];

            if !results.contains_key(&offset) {
                let result = a.iter().filter(|&&x| perfect_square((x+offset) as u128)).count();
                results.insert(offset, result);
            }
        }
    }

    let mut answer : Option<(usize, u64)> = None;

    for (result, count) in results.into_iter() {
        if let Some(answer) = &mut answer {
            *answer = *answer.max(&mut (count, result));
        }
        else {
            answer = Some((count, result));
        }
    }

    if let Some(answer) = answer {
        out_line!(answer.0);
    }
    else {
        out_line!(1);
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
