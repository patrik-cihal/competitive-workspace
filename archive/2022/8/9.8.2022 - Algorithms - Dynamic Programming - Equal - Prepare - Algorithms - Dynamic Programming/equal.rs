//{"name":"Equal","group":"HackerRank - Algorithms - Dynamic Programming - Equal - Prepare - Algorithms - Dynamic Programming","url":"https://www.hackerrank.com/challenges/equal/problem?isFullScreen=true","interactive":false,"timeLimit":4000,"tests":[{"input":"STDIN       Function\n-----       --------\n1           t = 1\n4           arr[] size n = 4\n2 2 3 7     arr =[2, 2, 3, 7]\n","output":"2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"Equal"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let mut v: Vec<i32> = input.read_vec(n);
    
    let mn = *v.iter().min().unwrap();

    let mut answer = i32::MAX;
    for i in 0..5 {
        let mut result = 0;

        for j in 0..n {
            let d = v[j]-(mn-i);
            result += (d/5)+(d%5+1)/2;
        }
        
        answer = answer.min(result);
    }
    
    out_line!(answer);

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
