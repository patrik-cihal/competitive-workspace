//{"name":"segment-tree-test","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"segment-tree-test"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

use algo_lib::collections::segment_tree::SegmentTree;

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let v = input.read_vec::<u64>(n);

    let mut seg_tree = SegmentTree::new_from_iter(v.clone(), 0, |a, b| a + b);

    seg_tree.update(0, 10);

    out_line!(seg_tree.query(0..n/2));
    out_line!(seg_tree.query(n/2..n));
    out_line!(seg_tree.query(0..n));
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
