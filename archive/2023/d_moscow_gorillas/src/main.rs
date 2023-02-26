//{"name":"D. Moscow Gorillas","group":"Codeforces - Codeforces Round #852 (Div. 2)","url":"https://codeforces.com/contest/1793/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"3\n1 3 2\n2 1 3\n","output":"2\n"},{"input":"7\n7 3 6 2 1 5 4\n6 7 2 5 3 1 4\n","output":"16\n"},{"input":"6\n1 2 3 4 5 6\n6 5 4 3 2 1\n","output":"11\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DMoscowGorillas"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();    
    let p: Vec<usize> = input.read_vec::<usize>(n).into_iter().map(|x| x-1).collect();
    let q: Vec<usize> = input.read_vec::<usize>(n).into_iter().map(|x| x-1).collect();

    let mut pi = vec![0; n];
    let mut qi = vec![0; n];

    for i in 0..n {
        pi[p[i]] = i;
        qi[q[i]] = i;
    }


    let mut a = pi[0];
    let mut b = qi[0]; 

    if a>b {
        std::mem::swap(&mut a, &mut b);
    }

    let mut answer = a*(a+1)/2 + (n-b-1)*(n-b)/2 + if b==a {0} else {(b-a-1)*(b-a)/2};

    let mut l = a;
    let mut r = b;

    for i in 1..n {
        let mut a = pi[i];
        let mut b = qi[i]; 

        if a>b {
            std::mem::swap(&mut a, &mut b);
        }

        let new_l = l.min(a);
        let new_r = r.max(b);

        if (a >= l && a <= r) || (b>=l && b<=r) {
            l = new_l;
            r = new_r;
            continue;
        }
        else {
            if a<l && b>r {
                answer += (b-r)*(l-a);
            }
            else if a<l && b<l {
                answer += (n-r)*(l-b);
            }
            else {
                answer += (l+1)*(a-r);
            }
            l = new_l;
            r = new_r;
        }
    }
    answer += 1;
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
