//{"name":"D. Serval and Shift-Shift-Shift","group":"Codeforces - Codeforces Round #853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n5\n00111\n11000\n1\n1\n1\n3\n001\n000\n","output":"2\n3 -2\n0\n-1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DServalAndShiftShiftShift"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn apply(a: &mut Vec<u8>, n: i32, answer: &mut Vec<i32>) {
    let mut result = a.clone();
    if n>0 {
        let n = n as usize;
        for i in 0..a.len()-n {
            result[i] ^= a[i+n];
        }
        for i in a.len()-n..a.len() {
            result[i] ^= 0;
        }
    }
    else {
        let n = (-n) as usize;
        for i in n..a.len() {
            result[i] ^= a[i-n];
        }
        for i in 0..n {
            result[i] ^= 0;
        }
    }
    answer.push(n);
    std::mem::swap(a, &mut result)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut a: Vec<u8> = input.read_digit_string(); 
    let b: Vec<u8> = input.read_digit_string();

    if a == b {
        out_line!(0);
        return;
    }
    else if a == vec![0; n] || b == vec![0; n] {
        out_line!(-1);
        return;
    }

    let mut answer = vec![];

    let mut a_left = a.iter().position(|&x| x == 1).unwrap();
    let b_left = b.iter().position(|&x| x == 1).unwrap();

    if a_left > b_left {
        apply(&mut a, (a_left-b_left) as i32, &mut answer);
        a_left = b_left;
    }

    for i in b_left..n {
        if a[i] != b[i] {
            apply(&mut a,  a_left as i32 - i as i32, &mut answer);
        }
    }

    let right = a.iter().rposition(|&x| x == 1).unwrap();

    for i in (0..b_left).rev() {
        if a[i] != b[i] {
            apply(&mut a,  (right - i) as i32, &mut answer);
        }
    }

    out_line!(answer.len());
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
