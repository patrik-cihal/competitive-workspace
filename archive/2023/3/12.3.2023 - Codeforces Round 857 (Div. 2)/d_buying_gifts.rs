//{"name":"D. Buying gifts","group":"Codeforces - Codeforces Round 857 (Div. 2)","url":"https://codeforces.com/contest/1802/problem/D","interactive":false,"timeLimit":3000,"tests":[{"input":"2\n2\n1 2\n2 1\n5\n1 5\n2 7\n3 3\n4 10\n2 5\n","output":"0\n1\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DBuyingGifts"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line, minim};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();

    let mut store1 = vec![];
    let mut store2 = vec![];

    for i in 0..n {
        let (p1, p2): (u64, u64) = input.read();
        store1.push((p1, i));
        store2.push((p2, i));
    }

    store1.sort();
    store2.sort();
    store1.reverse();
    store2.reverse();

    let mut seen = vec![false; n];

    let mut i = 0;
    let mut j = 0;

    let mut answer = u64::MAX;

    while i<n && j<n {
        
        if store1[i].0 > store2[j].0 {
            let j = j + (store1[i].1 ==store2[j].1) as usize;
            if j == n {
                if seen[store1[i].1] {
                    break;
                }
                i += 1;
                continue;
            }
            answer = answer.min(store1[i].0-store2[j].0);
            if seen[store1[i].1] {
                break;
            }
            seen[store1[i].1] = true;
            i += 1;
        }
        else {
            let i = i + (store1[i].1 ==store2[j].1) as usize;
            if i == n {
                if seen[store2[j].1] {
                    break;
                }
                j += 1;
                continue;
            }
            answer = answer.min(store2[j].0-store1[i].0);
            if seen[store2[j].1] {
                break;
            }
            seen[store2[j].1] = true;
            j += 1;
        }
        
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
