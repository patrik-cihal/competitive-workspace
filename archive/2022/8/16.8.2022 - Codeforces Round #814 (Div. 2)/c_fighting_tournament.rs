//{"name":"C. Fighting Tournament","group":"Codeforces - Codeforces Round #814 (Div. 2)","url":"https://codeforces.com/contest/1719/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n3 1\n3 1 2\n1 2\n4 2\n1 3 4 2\n4 5\n3 2\n5 2\n1 2 3 5 4\n5 1000000000\n4 6\n","output":"2\n0\n1\n0\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CFightingTournament"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let q: usize = input.read();

    let a: Vec<i32> = input.read_vec(n);

    let mut queries = vec![(0, 0, 0); q];
    let mut answer = vec![0; q];
    let mut result = vec![0; n];


    for i in 0..q {
        let j: usize = input.read();
        let k: usize = input.read();

        queries[i] = (k, j-1, i);
    }

    queries.sort();

    let mut ck = 0;
    let mut cw = 0;
    let mut j = 1;

    // save current winner
    // once we go to next query
    // move in the array by the offset and update win count unless current has highest damage
    // then print win count for the user
    // if j is higher then n
    // update win count for winner by remaining ok
    for i in 0..q {
        while j<n && ck != queries[i].0 {
            ck += 1;
            if a[j]>a[cw] {
                cw = j;
            }
            result[cw] += 1;
            j += 1;
        }
        result[cw] += queries[i].0-ck;
        ck = queries[i].0;
        answer[queries[i].2] = result[queries[i].1];
    }
    for i in 0..q {
        out_line!(answer[i]);
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
