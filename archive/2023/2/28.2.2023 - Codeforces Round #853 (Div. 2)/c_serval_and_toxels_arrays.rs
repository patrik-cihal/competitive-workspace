//{"name":"C. Serval and Toxel's Arrays","group":"Codeforces - Codeforces Round #853 (Div. 2)","url":"https://codeforces.com/contest/1789/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 2\n1 2 3\n1 4\n2 5\n1 1\n1\n1 1\n10 10\n4 6 9 12 16 20 2 10 19 7\n1 3\n5 4\n2 17\n2 18\n6 11\n7 1\n8 17\n5 5\n5 5\n2 2\n","output":"13\n1\n705\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CServalAndToxelsArrays"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut count = vec![0; n + 1 + m];
    let mut observed = vec![None; n+m+1];

    let mut a: Vec<usize> = input.read_vec(n);

    for i in 0..n {
        observed[a[i]] = Some(0);
    }

    for i in 0..m {
        let x: usize = input.read();
        let y: usize = input.read();

        if let Some(last) = observed[a[x-1]] {
            count[a[x-1]] += 1+i-last;
            observed[a[x-1]] = None;
        }
        observed[y] = Some(i+1);
        a[x-1] = y;
    }

    for i in 0..n+m+1 {
        if let Some(last) = observed[i] {
            count[i] += m+1-last;
        }
    }

    
    let mut answer = n*2*m*(m+1)/2;
    for val in count {
        if val == 0 {
            continue;
        }
        answer -= val*(val-1)/2; 
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
