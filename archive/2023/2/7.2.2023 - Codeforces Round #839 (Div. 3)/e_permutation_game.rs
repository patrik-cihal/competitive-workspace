//{"name":"E. Permutation Game","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/E","interactive":false,"timeLimit":4000,"tests":[{"input":"4\n4\n1 2 4 3\n3\n2 3 1\n5\n3 4 5 2 1\n6\n1 5 6 3 2 4\n","output":"First\nTie\nSecond\nTie\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EPermutationGame"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let v = input.read_vec::<i32>(n);
    let mut first_v = v.clone();
    let mut second_v = v.clone();

    first_v.sort();
    second_v.sort();
    second_v.reverse();

    let mut a = 0;
    let mut b = 0;
    let mut c = 0;
    for i in 0..n {
        if v[i] != first_v[i] && v[i] != second_v[i] {
            c += 1;
        }
        else if v[i] != first_v[i] {
            a+=1;
        }
        else if v[i] != second_v[i] {
            b += 1;
        }
    }

    if a+c <= b {
        out_line!("First");
    }
    else if b+c < a {
        out_line!("Second");
    }
    else {
        out_line!("Tie");
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
