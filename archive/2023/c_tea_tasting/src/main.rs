//{"name":"C. Tea Tasting","group":"Codeforces - Educational Codeforces Round 143 (Rated for Div. 2)","url":"https://codeforces.com/contest/1795/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n3\n10 20 15\n9 8 6\n1\n5\n7\n4\n13 8 5 4\n3 4 2 1\n3\n1000000000 1000000000 1000000000\n1 1 1000000000\n","output":"9 9 12\n5\n3 8 6 4\n1 2 2999999997\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CTeaTasting"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read(); 

    let mut bh = BinaryHeap::new();


    let mut a: Vec<u64> = input.read_vec(n);
    let mut b = input.read_vec::<u64>(n);

    // how much are we going drinking
    let mut drink = 0;
    let mut answer = vec![0; n];

    for i in 0..n {
        // when is it going to expire
        bh.push(Reverse(a[i]+drink));

        // check whether can is some tea empty

        drink += b[i];


        while let Some(Reverse(left)) = bh.pop() {
            if left > drink {
                bh.push(Reverse(left));
                break;
            }
            else {
                answer[i] += left-(drink-b[i]);
            }
        }

        answer[i] += b[i]*bh.len() as u64;
        
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
