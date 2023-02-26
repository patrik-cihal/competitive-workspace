//{"name":"circuits","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"circuits"}}}

use std::collections::{HashSet, HashMap};

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    
    let n: usize = input.read(); 
    let mut points = vec![];
    for i in 0..n {
        let m: usize = input.read();

        for j in 0..m {
            let l: u32 = input.read();
            let r: u32 = input.read();

            points.push((l, i));
            points.push((r, i));
        }
    }

    points.sort();

    let mut column_ranges: Vec<(u32, Vec<usize>, u32)> = vec![];
    let mut rows = vec![false; n];
    for i in 1..points.len() {
        rows[points[i-1].1] = !rows[points[i-1].1];
        if points[i].0 == points[i-1].0 {
            continue;
        }
        let active_rows = rows.iter().enumerate().filter(|(_, &x)| x).map(|(i, _)| i).collect::<Vec<usize>>();
        if active_rows.len() > 0 {
            column_ranges.push((points[i].0-points[i-1].0, active_rows, points[i-1].0));
        }
    }

    let mut flow_network: HashMap<String, HashMap<String, (u32, u32)>> = HashMap::new();

    flow_network.create_node("source".to_string());

    for i in 0..column_ranges.len() {
        flow_network.create_node(format!("column_range{}", i));
        flow_network.add_edge("source", &format!("column_range{}", i), column_ranges[i].0, 0);
    }

    flow_network.create_node("sink".to_string());

    for i in 0..n {
        flow_network.create_node(format!("row{}", i));
        flow_network.add_edge(&format!("row{}", i), "sink", 1, 0);
    }

    for i in 0..column_ranges.len() {
        for j in 0..column_ranges[i].1.len() {
            flow_network.add_edge(&format!("column_range{}", i), &format!("row{}", column_ranges[i].1[j]), 1, 0);
        }
    }

    flow_network.maximize_flow("source", "sink");

    let mut answer = vec![];

    for i in 0..column_ranges.len() {
        for j in 0..column_ranges[i].1.len() {
            if flow_network.get_edge(&format!("column_range{}", i), &format!("row{}", column_ranges[i].1[j])).flow > 0 {
                answer.push((column_ranges[i].2, column_ranges[i].1[j]));
            }
        }
    }

    if answer.len() != n {
        out_line!("IMPOSSIBLE");
        return;
    }

    answer.sort(); 

    
    for i in 1..answer.len() {
        if answer[i].0 <= answer[i-1].0 {
            answer[i].0 += 1;
        }
    }

    answer.sort_by(|a, b| a.1.cmp(&b.1));

    for i in 0..answer.len() {
        print!("{} ", answer[i].0);
    }
    println!();

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
