//{"name":"domino","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"domino"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};
use algo_lib::misc::recursive_function::{Callable2, Callable3, RecursiveFunction2, RecursiveFunction3};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut k: Vec<Vec<char>> = vec![0; n].iter().map(|_| input.read::<String>().chars().collect()).collect();
    let mut z: Vec<Vec<char>> = vec![0; n].iter().map(|_| input.read::<String>().chars().collect()).collect();


    let mut fix = RecursiveFunction3::new(move |f, x: usize, y: usize, init: bool| -> Vec<(usize, usize)> {
        if init && k[y][x] == z[y][x] {
            return vec![];
        }
        if k[y][x] == 'U' {
            let mut answer = vec![];
            if k[y][x+1] != 'U' {
                answer = f.call(x+1, y, false);
            }
            k[y][x] = 'L';
            k[y][x+1] = 'R';
            k[y+1][x] = 'L';
            k[y + 1][x + 1] = 'R';

            answer.push((x, y));
            answer
        }
        else if k[y][x] == 'L' {
            let mut answer = vec![];
            if k[y+1][x] != 'L' {
                answer = f.call(x, y+1, false);
            }
            k[y][x] = 'U';
            k[y][x+1] = 'U';
            k[y+1][x] = 'D';
            k[y + 1][x + 1] = 'D';

            answer.push((x, y));
            answer
        }
        else if k[y][x] == 'D' {
            let mut answer = f.call(x, y-1, false);
            answer.append(&mut f.call(x, y, false));
            answer
        }
        else {
            println!("wth shouldn't happen");
            vec![]
        }

    });

    let mut answer = vec![];

    for x in 0..m {
        for y in 0..n {
            answer.append(&mut fix.call(x, y, true));
        }
    }

    out_line!(answer.len());
    for result in answer {
        out_line!(result.1+1, result.0+1);
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
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
