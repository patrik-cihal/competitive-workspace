//{"name":"C. Min Max Sort","group":"Codeforces - Educational Codeforces Round 142 (Rated for Div. 2)","url":"https://codeforces.com/contest/1792/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\n1 5 4 2 3\n3\n1 2 3\n4\n2 1 4 3\n6\n5 2 4 1 6 3\n","output":"2\n0\n1\n3\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CMinMaxSort"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, test_case: usize) {
    let n: usize = input.read();    
    let mut v = vec![0; n];

    // assign indices by numbers
    for i in 0..n {
        v[input.read::<usize>() - 1] = i;
    }

    // go over the array and find the first pair that is not in the right place
    // store left and right

    let mut offset = 0;

    let mut last_left = if n%2 == 1 { Some(v[n/2]) } else {None};
    let mut last_right = if n%2 == 1 { Some(v[n/2]) } else {None};

    let left = |offset| {
        n/2-1-offset
    };
    let right = |offset| {
        (n+1)/2+offset
    };

    while offset<n/2 && v[left(offset)] < v[right(offset)] && v[left(offset)] < last_left.unwrap_or(n) && v[right(offset)] > last_right.unwrap_or(0) {
        last_left = Some(v[left(offset)]);
        last_right = Some(v[right(offset)]);
        offset += 1;
    }


    out_line!(n/2-offset);

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
