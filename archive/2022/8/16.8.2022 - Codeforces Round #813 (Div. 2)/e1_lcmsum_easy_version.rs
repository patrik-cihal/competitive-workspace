//{"name":"E1. LCM Sum (easy version)","group":"Codeforces - Codeforces Round #813 (Div. 2)","url":"https://codeforces.com/contest/1712/problem/E1","interactive":false,"timeLimit":3500,"tests":[{"input":"5\n1 4\n3 5\n8 86\n68 86\n6 86868\n","output":"3\n1\n78975\n969\n109229059713337\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"E1LCMSumEasyVersion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    // read l, r
    let l: usize = input.read();
    let r: usize = input.read();

    let mut result = 0;

    // go from l..r+1
    for i in l..r+1 {
        // for given i calculate m divisors of i and o odd divisors of i*2
        let mut divisors = vec![];
        let mut cd = 2;
        while i/cd>=l {
            if i%cd == 0 {
                divisors.push(i/cd);
            }
            cd += 1;
        }

        if divisors.len() == 0 {
            continue;
        }
        // to the result add m*(m-1)/2
        result += divisors.len()*(divisors.len()-1)/2;

        // get all divisors only of i*2 into an array
        let mut odd_divisors = vec![];

        let mut cd = 3;
        while (i*2)/cd >= l {
            if i*2%cd == 0 {
                odd_divisors.push(i*2/cd);
            }
            cd += 2;
        }

        if odd_divisors.len() == 0 {
            continue;
        }

        divisors.reverse();
        odd_divisors.reverse();

        let mut j = 0;
        let mut k = 0;
        let mut l = 0;

        let mut dpos = vec![0; divisors.len()];
        let mut odpos = vec![0; odd_divisors.len()];

        while j != divisors.len() || k != odd_divisors.len() {
            if j == divisors.len() {
                odpos[k] = l;
                k += 1;
                l += 1;
            }
            else if k==odd_divisors.len() || divisors[j]<odd_divisors[k] {
                dpos[j] = l;
                j += 1;
                l+=1;
            }
            else {
                odpos[k] = l;
                k += 1;
                l += 1;
            }
        }

        // for normal divisors find which satisfy
        // one pointer will be 0 the other will go from the top, the bottom pointer increases
        // until the sum is sufficient once the sum is sufficient to the result we add up pointer - down pointer

        let mut j = 0;
        let mut k = divisors.len()-1;

        // for odd divisors find which satisfy
        while j != odd_divisors.len() {
            if odd_divisors[j]+divisors[k]<=i {
                j += 1;
                continue;
            }
            result += (odpos[j] as i32 - dpos[k] as i32).abs() as usize;
            if k == 0 {
                break;
            }
            k-=1;
        }


        let mut j = 0;
        let mut k = odd_divisors.len()-1;

        while j!=k {
            if odd_divisors[j]+odd_divisors[k]<=i {
                j += 1;
                continue;
            }
            result += odpos[k]-odpos[j];
            k-=1;
        }
    }

    let n = r-l+1;
    let total = n*(n-1)*(n-2)/6;

    // the answer is the number of all possible triplets minus our result
    out_line!(total-result);
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
