//{"name":"D. Absolute Sorting","group":"Codeforces - Codeforces Round #839 (Div. 3)","url":"https://codeforces.com/contest/1772/problem/D","interactive":false,"timeLimit":2000,"tests":[{"input":"8\n5\n5 3 3 3 5\n4\n5 3 4 5\n8\n1 2 3 4 5 6 7 8\n6\n10 5 4 3 2 1\n3\n3 3 1\n3\n42 43 42\n2\n100000000 99999999\n6\n29613295 52036613 75100585 78027446 81409090 73215\n","output":"4\n-1\n0\n42\n2\n-1\n100000000\n40741153\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DAbsoluteSorting"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.read();    
    let v = input.read_vec::<i32>(n);

    let mut mx = i32::MAX/4;
    let mut mn = 0;

    let mut up ;

    let mut start = None;
    for i in 1..n {
        if v[i] != v[i-1] {
            start = Some(i);
            break;
        }
    }

    let Some(start) = start else {
        out_line!(v[0]);
        return;
    };

    if v[start] > v[start-1] {
        mx = v[start-1]+(v[start]-v[start-1])/2;
        up = true;
    }
    else {
        up = false;
        mn = v[start]+(v[start-1]-v[start]+1)/2;
    }

    for i in 1..n {
        if v[i]>v[i-1] {
            if !up {
                mx = mx.min(v[i-1]+(v[i]-v[i-1])/2);
            }
            up = true;
        }
        else if v[i]<v[i-1] {
            if up {
                mn = mn.max(v[i]+(v[i-1]-v[i]+1)/2);
            }
            up = false;
        }
    }

    if mn>mx {
        out_line!(-1);
    }
    else {
        out_line!(mn);
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
