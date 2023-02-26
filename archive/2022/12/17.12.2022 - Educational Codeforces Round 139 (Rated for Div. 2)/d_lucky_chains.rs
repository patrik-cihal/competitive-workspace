//{"name":"D. Lucky Chains","group":"Codeforces - Educational Codeforces Round 139 (Rated for Div. 2)","url":"https://codeforces.com/contest/1766/problem/D","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n5 15\n13 37\n8 9\n10009 20000\n","output":"0\n1\n-1\n79\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLuckyChains"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::numbers::primes::{Factorize, primes};
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize, primes: &Vec<i32>) -> i64 {
    let x: i32 = input.read();
    let y: i32 = input.read();

    let mut dif = (y-x) as i32;

    if dif == 1{
        return -1;
    }

    let mut divisors = vec![];
    for prime in primes {
        if dif % prime == 0 {
            divisors.push(*prime);
            while dif % prime == 0 {
                dif /= prime;
            }
        }
    }
    if dif != 1 {
        divisors.push(dif);
    }
    let mut answer = i32::MAX;
    for divis in divisors {
        if y%divis == 0 {
            answer = 0;
        }
        else {
            answer = answer.min(divis-(y%divis));
        }
    }
    answer.into()
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let M = ((10e7+20.) as f64).sqrt() as usize;

    let primes = primes(M);
    for i in 0usize..t {
        out_line!(solve(&mut input, i + 1, &primes));
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
