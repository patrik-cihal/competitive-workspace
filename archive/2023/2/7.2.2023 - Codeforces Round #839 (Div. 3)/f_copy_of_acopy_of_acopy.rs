//{"name":"F. Copy of a Copy of a Copy","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/F","interactive":false,"timeLimit":2000,"tests":[{"input":"3 3 1\n\n010\n111\n010\n\n010\n101\n010\n","output":"2\n2\n1 2 2\n2 1\n"},{"input":"4 5 3\n\n00000\n01000\n11100\n01000\n\n00000\n01000\n10100\n01000\n\n00000\n01010\n10100\n01000\n\n00000\n01000\n10100\n01000\n","output":"3\n5\n1 2 4\n2 2\n2 4\n1 3 2\n2 1\n"},{"input":"5 3 0\n\n110\n010\n001\n011\n001\n","output":"1\n0\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"FCopyOfACopyOfACopy"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use std::collections::HashSet;

fn count(v: Vec<Vec<bool>>) -> HashSet<(usize, usize)> {
    let mut result = HashSet::new();
    for i in 1..v.len()-1 {
        for j in 1..v[0].len()-1 {
            let cat = v[i][j];
            if cat != v[i-1][j] && cat != v[i+1][j] && cat != v[i][j+1] && v[i][j-1] != cat {
                result.insert((i, j));
            }
        }
    }
    result
}

enum Operation {
    Change(usize, usize),
    Picture(usize)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();
    let m = input.read::<usize>();
    let k = input.read::<usize>();


    let mut pictures = vec![];
    for i in 0..k+1 {
        let mut v = vec![vec![]; n];
        for i in 0..n {
            v[i] = input.read::<String>().chars().map(|x| x=='1').collect();
        }
        pictures.push((i, count(v.clone()), v));
    }

    if k == 0 {
        out_line!(1);
        out_line!(0);
        return;
    }

    pictures.sort_by(|a, b| b.1.len().cmp(&a.1.len()));
    out_line!(pictures[0].0+1);

    let mut answer = vec![];

    for i in 1..pictures.len() {
        for val in pictures[i-1].1.iter() {
            if !pictures[i].1.contains(val) && pictures[i].2[val.0][val.1] != pictures[i-1].2[val.0][val.1] {

                answer.push(Operation::Change(val.0+1, val.1+1));
            }
        }
        answer.push(Operation::Picture(pictures[i].0+1));
    }

    out_line!(answer.len());
    for result in answer {
        match result {
            Operation::Change(i, j) => {
                out_line!(1, i, j);
            }
            Operation::Picture(i) => {
                out_line!(2, i);
            }
        }

    }



}

pub(crate) fn run(mut input: Input) -> bool {
    for i in 0usize..1 {
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
