//{"name":"C. Equal Frequencies","group":"Codeforces - Codeforces Round #844 (Div. 1 + Div. 2, based on VK Cup 2022 - Elimination Round)","url":"https://codeforces.com/contest/1782/problem/C","interactive":false,"timeLimit":2000,"tests":[{"input":"4\n5\nhello\n10\ncodeforces\n5\neevee\n6\nappall\n","output":"1\nhelno\n2\ncodefofced\n1\neeeee\n0\nappall\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"CEqualFrequencies"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn find_primes(n: usize) -> Vec<u32> {
    if n == 1 {
        return vec![2];
    }
    let mut v = vec![true; n + 1];
    v[0] = false;
    v[1] = false;
    let mx = (n as f64).sqrt().ceil() as usize;
    for i in 2..mx {
        if v[i] {
            for j in (i * i..n + 1).step_by(i) {
                v[j] = false;
            }
        }
    }
    let mut result = vec![];
    for i in 0..v.len() {
        if v[i] {
            result.push(i as u32);
        }
    }
    result
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();;

    
    let mut divisors = vec![];

    for i in 1..n+1 {
        if n%i == 0 && n/i <= 26 {
            divisors.push(i);
        }
    }

    let mut v = input.read::<String>().chars().collect::<Vec<char>>();
    // let mut v: Vec<char> = vec!['a'; n].into_iter().map(|_| (rand::random::<u8>()%26 + 'a' as u8) as char).collect();


    let mut count = vec![('a', vec![]); 26];

    for i in 0..26 {
        count[i].0 = (i as u8 + 'a' as u8) as char;
    }
    

    for (i, c) in v.iter().enumerate() {
        let index = *c as usize - 'a' as usize;
        count[index].1.push(i);
    }

    count.sort_by(|a, b| b.1.len().cmp(&a.1.len()));

    let mut candidates = vec![];

    for divisor in divisors {
        let mut below = 0;
        for i in 0..n/divisor {
            if count[i].1.len() < divisor {
                below += divisor-count[i].1.len();
            }
        }
        candidates.push((below, divisor));
    }

    let mut best_candidate = *candidates.iter().min().unwrap();

    let mut available = vec![];

    let divisor = best_candidate.1;

    for i in 0..n/divisor {
        if count[i].1.len() > divisor {
            for j in divisor..count[i].1.len() {
                available.push(count[i].1[j]);
            }
        }
    }
    for i in n/divisor..26 {
        for pos in &count[i].1 {
            available.push(*pos);
        }
    }

    let mut index = 0;
    for i in 0..n/divisor {
        if count[i].1.len() < divisor {
            for j in 0..divisor-count[i].1.len() {
                v[available[index]] = count[i].0;
                index += 1;
            }
        }
    }

    
    let answer = v.iter().collect::<String>();
    out_line!(best_candidate.0);
    out_line!(answer);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();;
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
