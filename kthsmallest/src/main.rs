//{"name":"kth smallest","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"kthsmallest"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn kth_smallest(v: Vec<i32>, k: usize) -> i32 {
    let mid = v[0];
    let mut left: Vec<i32> = vec![];
    let mut right: Vec<i32> = vec![];

    for i in 1..v.len() {
        if v[i] <= mid {
            left.push(v[i]);
        }
        else {
            right.push(v[i]);
        }
    }
    
    if k==left.len() {
        return mid;
    }

    if k<left.len() {
        kth_smallest(left, k)
    }
    else {
        kth_smallest(right, k-left.len()-1)
    }

}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let k = input.read::<usize>()-1;

    let v = input.read_vec::<i32>(n);

    out_line!(kth_smallest(v, k));
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
