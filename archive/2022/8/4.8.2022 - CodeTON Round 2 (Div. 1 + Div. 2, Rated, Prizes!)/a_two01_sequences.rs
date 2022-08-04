//{"name":"A. Two 0-1 sequences","group":"Codeforces - CodeTON Round 2 (Div. 1 + Div. 2, Rated, Prizes!)","url":"https://codeforces.com/contest/1704/problem/A","interactive":false,"timeLimit":1000,"tests":[{"input":"10\n6 2\n001001\n11\n6 2\n110111\n01\n6 2\n000001\n11\n6 2\n111111\n01\n8 5\n10000101\n11010\n7 4\n1010001\n1001\n8 6\n01010010\n010010\n8 4\n01010101\n1001\n8 4\n10101010\n0110\n7 5\n1011100\n11100\n","output":"YES\nYES\nNO\nNO\nNO\nYES\nYES\nNO\nNO\nYES\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATwo01Sequences"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let m: usize = input.read();

    let mut a: Vec<char> = input.read::<String>().chars().collect();
    let b: Vec<char> = input.read::<String>().chars().collect();


    for i in 0..b.len()-1 {
        if a[n-i-1] != b[m-i-1] {
            out_line!("NO");
            return;
        }
    }
    let need = b[0];
    a.truncate(n+1-m);

    if !a.contains(&need) {
        out_line!("NO");
    }
    else {
        out_line!("YES");
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
