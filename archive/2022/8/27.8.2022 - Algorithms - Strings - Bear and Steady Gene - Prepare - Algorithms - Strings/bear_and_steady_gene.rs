//{"name":"Bear and Steady Gene","group":"HackerRank - Algorithms - Strings - Bear and Steady Gene - Prepare - Algorithms - Strings","url":"https://www.hackerrank.com/challenges/bear-and-steady-gene/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"8\nGAAATAAA\n","output":"5\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"BearAndSteadyGene"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let s: Vec<char> = input.read::<String>().chars().collect();
    let v: Vec<usize> = s.into_iter().map(|x| if x=='A' {0} else if x=='C' {1} else if x=='G' {2} else {3}).collect();
    let mut count = vec![-(n as i32)/4; 4];
    let mut last_index = vec![0; 4];
    for i in 0..n {
        count[v[i]] += 1;
    }
    let mut c = 0;
    for i in 0..4 {
        if count[i] <= 0 {
            last_index[i] = n;
            c += 1;
        }
        else {
            let mut j = 0;
            while v[j] != i {
                j += 1;
            }
            last_index[i] = j;
        }
    }
    if c==4 {
        out_line!(0);
        return;
    }
    let mut answer = usize::MAX;
    // we will loop through the array
    // we check whether remaining count is less than 0
    // if not then we reduce count
    // if yes then we update the last index and answer
    for i in 0..n {
        if count[v[i]] <= 0 {
            last_index[v[i]] += 1;
            while last_index[v[i]] < n && v[last_index[v[i]]] != v[i] {
                last_index[v[i]] += 1;
            }
        }
        else {
            count[v[i]] -= 1;
        }
        let mut c = true;
        for i in 0..4 {
            if count[i] > 0 {
                c = false;
            }
        }
        if c {
            answer = answer.min(i-last_index.iter().min().unwrap()+1);
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
