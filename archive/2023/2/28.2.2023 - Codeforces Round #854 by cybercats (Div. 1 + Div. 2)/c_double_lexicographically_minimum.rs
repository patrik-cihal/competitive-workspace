//{"name":"C. Double Lexicographically Minimum","group":"Codeforces - Codeforces Round #854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"12\na\naab\nabb\nabc\naabb\naabbb\naaabb\nabbb\nabbbb\nabbcc\neaga\nffcaba\n","output":"a\naba\nbab\nbca\nabba\nabbba\nababa\nbbab\nbbabb\nbbcca\nagea\nacffba\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CDoubleLexicographicallyMinimum"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    let mut s = input.read::<String>().chars().collect::<Vec<char>>();
    let n: usize = s.len();

    if n == 1 {
        out_line!(s[0]);
        return;
    }
    
    let mut count = vec![0; 26];

    for ch in &s {
        count[*ch as usize - 'a' as usize] += 1;
    }

    s.sort();

    let mut front = vec![];
    let mut back = vec![];

    let mut i = 0;

    while i<n-1 && s[i] == s[i+1] {
        front.push(s[i]);
        back.push(s[i]);
        i += 2;
    }

    let mut front2 = front.clone();
    let mut back2 = back.clone();

    {
        for j in i+1..n {
            if (j-(i+1)) % 2 == 0 {
                front2.push(s[j]);
            }
            else {
                back2.push(s[j]);
            }
        }
        if i<n {
            back2.push(s[i]);
        }
    }

    if i < n {
        front.push(s[i]);
        for i in i+1..n {
            back.push(s[i]);
        }
    }

    let mut result1 = vec![];
    for i in 0..front.len() {
        result1.push(front[i]);
    }
    for i in (0..back.len()).rev() {
        result1.push(back[i]);
    }

    let mut result2 = vec![];
    for i in 0..front2.len() {
        result2.push(front2[i]);
    }
    for i in (0..back2.len()).rev() {
        result2.push(back2[i]);
    }

    let result1_reversed = result1.clone().into_iter().rev().collect::<Vec<char>>();
    let result2_reversed = result2.clone().into_iter().rev().collect::<Vec<char>>();
    let result1max = result1.max(result1_reversed);
    let result2max = result2.max(result2_reversed);
    if result1max < result2max {
        for ch in result1max {
            out!(ch);
        }
    }
    else {
        for ch in result2max {
            out!(ch);
        }
    }

    out_line!();
    
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
