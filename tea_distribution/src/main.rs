//{"name":"tea_distribution","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"tea_distribution"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

#[derive(Clone)]
struct Edge {
    target: usize,
    opened: bool
}

#[derive(Clone)]
enum Leaf {
    Manager,
    Programmer,
    Empty
}

impl Leaf {
    fn from_char(c: char) -> Self {
        match c {
            'M' => Leaf::Manager,
            'P' => Leaf::Programmer,
            'E' => Leaf::Empty,
            _ => panic!("Wrong input")
        }
    }
}

#[derive(Default, Clone)]
struct Node {
    parent_edge: Option<Edge>,
    leaf: Option<Leaf>
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let m: usize = input.read();

    let mut nodes: Vec<Node> = vec![Default::default(); n];

    for i in 0..n-1 {
        let from: usize = input.read();
        let to: usize = input.read();
        let opened = input.read::<char>() == 'O';

        nodes[to-1].parent_edge = Some(Edge {
            target: from-1,
            opened
        });
    }


    let mut leafs = vec![0; m];
    for i in 0..m {
        let node_index = input.read::<usize>()-1;
        nodes[node_index].leaf = Some(Leaf::from_char(input.read::<char>()));
        leafs[i] = node_index;
    }
    
    if n == 1 {
        out_line!(0);
        return;
    }


    let mut programmer_used = vec![false; n];
    programmer_used[0] = true;

    let mut answer = 0;

    for &leaf in &leafs {
        let Some(Leaf::Programmer) = nodes[leaf].leaf else {
            continue;
        };
        let mut cur_node = leaf;
        loop {
            if programmer_used[cur_node] {
                break;
            }
            programmer_used[cur_node] = true;
            let parent_edge = nodes[cur_node].parent_edge.clone().unwrap();
            if !parent_edge.opened {
                answer += 1;
            }
            cur_node = parent_edge.target;
        }
    }        
    let mut manager_used = vec![false; n];
    for &leaf in &leafs {
        let Some(Leaf::Manager) = nodes[leaf].leaf else {
            continue;
        };

        let mut cur_node = leaf;
        loop {
            manager_used[cur_node] = true;
            let parent_edge = nodes[cur_node].parent_edge.clone().unwrap();
            let parent_node = parent_edge.target;

            if manager_used[parent_node] || !parent_edge.opened {
                break;
            }
            if programmer_used[parent_node] {
                answer += 1;
                break;
            }
            cur_node = parent_node;
        }
    }

    out_line!(answer);
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
