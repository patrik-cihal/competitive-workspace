//{"name":"merge_soort","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"merge_soort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn merge_sort(v: Vec<i32>) -> (Vec<i32>, u32) {
    if v.len() == 1 {
        return (v, 0);
    }
    let left = v[0..v.len() / 2].to_owned();
    let right = v[v.len() / 2..v.len()].to_owned();

    let mut inversions = 0;
    let (left, il) = merge_sort(left);
    let (right, ir) = merge_sort(right);

    inversions += il + ir;

    let mut f = vec![];
    let mut j = 0;
    for i in 0..left.len() {
        while j < right.len() && right[j] < left[i] {
            f.push(right[j]);
            j += 1;
            inversions += (left.len()-i) as u32;
        }
        f.push(left[i]);
    }
    while j < right.len() {
        f.push(right[j]);
        j += 1;
    }
    (f, inversions)
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<i32>(n);
    let answer = merge_sort(v);
    out_line!(answer.0);
    out_line!(answer.1);
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
