//{"name":"hrosi_santa","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"hrosi_santa"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

use std::collections::HashSet;

fn solve(input: &mut Input, _test_case: usize) {
    let mut found = HashSet::<(u32, u32)>::new();
    let a: u32 = input.read();
    let b: u32 = input.read();
    let k: u32 = input.read();

    #[derive(Clone, Copy)]
    struct Vertex {
        state: (u32, u32),
        root: usize,
        syntax_move: usize,
    }

    let mut vertices = vec![Vertex {
        state: (0, 0),
        root: 0,
        syntax_move: 10,
    }];
    let mut counter = 0;
    let syntax_moves = [">", "<", "a", "b", "A", "B"];

    let mut i = 0;

    let answer = loop {
        let vertex = vertices[i];
        if vertex.state.0 == k || vertex.state.1 == k {
            break vertex;
        }

        let mut sc = [(0, 0); 6];
        let state = vertex.state;

        let d = (b - state.1).min(state.0);
        sc[0] = (state.0 - d, state.1 + d);

        let d = (a - state.0).min(state.1);
        sc[1] = (state.0 + d, state.1 - d);

        sc[2] = (0, state.1);
        sc[3] = (state.0, 0);

        sc[4] = (a, state.1);
        sc[5] = (state.0, b);

        for (j, c) in sc.into_iter().enumerate() {
            if found.insert(c) {
                vertices.push(Vertex {
                    state: c,
                    root: i,
                    syntax_move: j,
                });
            }
        }
        i += 1;
    };

    let mut path = vec![];
    let mut vertex = answer;
    while vertex.syntax_move != 10 {
        path.push(vertex.syntax_move);
        vertex = vertices[vertex.root];
    }
    path.reverse();
    out_line!(path.len());
    for syntax_move in path {
        out!(syntax_moves[syntax_move]);
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
