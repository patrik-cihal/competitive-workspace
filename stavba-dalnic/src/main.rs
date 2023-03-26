//{"name":"stavba-dalnic","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"stavba-dalnic"}}}

use std::collections::BTreeSet;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, minim, maxim};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let q: usize = input.read();

    let mut roads = BTreeSet::new();

    for _ in 0..q {
        let (mut start, mut end): (usize, usize) = input.read();
        let mut to_remove = vec![];
        for &(cur_start, cur_end) in roads.range(..(end+2, 0)).rev() {
            if cur_end+1 < start {
                break;
            } 
            minim!(start, cur_start);
            maxim!(end, cur_end);
            to_remove.push((cur_start, cur_end));
        }
        for road in to_remove {
            roads.remove(&road);
        }
        roads.insert((start, end));
        out_line!(roads.len());
    }

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
