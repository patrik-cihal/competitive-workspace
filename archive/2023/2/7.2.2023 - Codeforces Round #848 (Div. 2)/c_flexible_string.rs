//{"name":"C. Flexible String","group":"Codeforces - Codeforces Round #848 (Div. 2)","url":"https://codeforces.com/contest/1778/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"6\n3 1\nabc\nabd\n3 0\nabc\nabd\n3 1\nxbb\nxcd\n4 1\nabcd\naxcb\n3 10\nabc\nabd\n10 3\nlkwhbahuqa\nqoiujoncjb\n","output":"6\n3\n6\n6\n6\n11\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFlexibleString"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn generate_combinations(pos: usize, left: usize, list_size: usize) -> Vec<Vec<usize>> {
    if left == 0 {
        return vec![vec![]];
    }
    let mut answer = vec![];
    for i in pos..list_size+1-left {
        let mut result = generate_combinations(i+1, left-1, list_size);
        for v in &mut result {
            v.push(i);
        }
        answer.extend(result);
    } 
    answer
}

fn solve(input: &mut Input, _test_case: usize) {


    let n = input.read::<usize>();
    let k = input.read::<usize>();

    let a = input.read::<String>().chars().collect::<Vec<char>>();
    let b = input.read::<String>().chars().collect::<Vec<char>>();

    let mut letters = vec![];
    for c in &a {
        if !letters.contains(c) {
            letters.push(*c);
        }
    }

    let mut answer = 0;

    for comb in generate_combinations(0, k.min(letters.len()), letters.len()) {
        let mut enabled_letters = vec![false; 26];

        for index in comb {
            enabled_letters[letters[index] as usize - 'a' as usize] = true;
        }

        let mut last = 0;
        let mut result = 0;
        for i in 0..n {
            if a[i] != b[i] && !enabled_letters[a[i] as usize - 'a' as usize] {
                last = i+1;
            }
            result += i+1-last;
        }
        answer = answer.max(result);
    }

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
