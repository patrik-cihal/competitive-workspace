//{"name":"extension_cords","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"extension_cords"}}}

use algo_lib::collections::graph::Graph;
use algo_lib::collections::segment_tree::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read::<usize>();    
    let mut g = vec![vec![]; n];
    let mut nodes_data = vec![0; n];
    for i in 0..n {
        nodes_data[i] = input.read();
    }
    for _ in 0..n-1 {
        let (u, v): (usize, usize) = input.read();
        g[u-1].push(v-1);
    }

    let mut leafs = vec![];
    for (u, _) in g.dfs(0) {
        if g[u].is_empty() {
            leafs.push(u);
        }
    }

    let mut st = SegmentTree::<(u32, usize), _>::new_from_iter(leafs.iter().enumerate().map(|(i, u)| (nodes_data[*u], i)), (0, 0), |a, b| {
        *a.max(b)
    });

    let mut segments = vec![None; n];

    fn find_segments(g: &Vec<Vec<usize>>, v: usize, left: usize, segments: &mut Vec<Option<(usize, usize)>>) -> usize {
        if g[v].is_empty() {
            return 1;
        }
        let mut count = 0;
        for u in &g[v] {
            count += find_segments(g, *u, left+count, segments);
        }
        segments[v] = Some((left, left+count));
        count
    }

    find_segments(&g, 0, 0, &mut segments);

    let mut answer = vec![];

    fn go<F: Fn(&(u32, usize), &(u32, usize)) -> (u32, usize)>
        (
            g: &Vec<Vec<usize>>, v: usize, nodes_data: &Vec<u32>, 
            st: &mut SegmentTree<(u32, usize), F>, segments: &Vec<Option<(usize, usize)>>,
            answer: &mut Vec<usize>
        ) -> u32 
    {
        if g[v].is_empty() {
            return nodes_data[v];
        }
        let mut sum = g[v].iter().map(|u| go(g, *u, nodes_data, st, segments, answer)).sum();
        while sum > nodes_data[v] {
            let segment = segments[v].unwrap();
            let (mx, i) = st.query(segment.0..segment.1);
            st.update(i, (0, 0));
            sum -= mx;
            answer.push(i);
        }
        sum
    }
    go(&g, 0, &nodes_data, &mut st, &segments, &mut answer);


    out_line!(answer.into_iter().map(|l| leafs[l]+1).collect::<Vec<usize>>());

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
