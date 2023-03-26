//{"name":"D. Accommodation","group":"Codeforces - Nebius Welcome Round (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1804/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"5 4\n0100\n1100\n0110\n1010\n1011\n","output":"7 10\n"},{"input":"1 8\n01011100\n","output":"3 4\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAccommodation"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn minimize(floor: &Vec<u8>) -> usize {
    let mut ap2 = floor.len()/4;

    let mut visited = vec![false; floor.len()];

    let mut count = 0;

    for i in 0..floor.len()-1 {
        if floor[i] == 1 && floor[i+1] == 1 && !visited[i] && ap2 > 0{
            visited[i] = true;
            visited[i+1] = true;
            ap2 -= 1;
            count += 1;
        }
    }

    for i in 0..floor.len()-1 {
        if (floor[i] == 1 || floor[i+1] == 1) && !visited[i] && !visited[i+1] && ap2 > 0 {
            ap2 -= 1;
            count += 1;
            visited[i] = true;
            visited[i+1] = true;
        }
    }

    for i in 0..floor.len() {
        if !visited[i] && floor[i] == 1 {
            count += 1;
        }
    }

    count
}
fn maximize(floor: &Vec<u8>) -> usize {
    let mut ap2 = floor.len()/4;

    let mut visited = vec![false; floor.len()];

    let mut count = 0;

    for i in 0..floor.len()-1 {
        if floor[i] == 0 && floor[i+1] == 0 && !visited[i] && ap2 > 0{
            visited[i] = true;
            visited[i+1] = true;
            ap2 -= 1;
        }
        else if (floor[i] == 0 || floor[i+1] == 0) && !visited[i] && !visited[i+1] && ap2 > 0 {
            visited[i] = true;
            visited[i+1] = true;
            ap2 -= 1;
            count += 1;
        }
    }

    for i in 0..floor.len() {
        if !visited[i] && floor[i] == 1 {
            count += 1;
        }
    }
    count -= ap2;

    count
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();

    let mut v: Vec<Vec<u8>> = vec![0; n].into_iter().map(|_| input.read_digit_string()).collect();

    let mut min_answer = 0;
    for i in 0..n {
        min_answer += minimize(&v[i]);
    }
    let mut max_answer = 0;
    for i in 0..n {
        max_answer += maximize(&v[i]);
    }

    out_line!(min_answer, max_answer);

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
