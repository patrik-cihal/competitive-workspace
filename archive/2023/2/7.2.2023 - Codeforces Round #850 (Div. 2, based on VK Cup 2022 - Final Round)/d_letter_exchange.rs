//{"name":"D. Letter Exchange","group":"Codeforces - Codeforces Round #850 (Div. 2, based on VK Cup 2022 - Final Round)","url":"https://codeforces.com/contest/1786/problem/D#","interactive":false,"timeLimit":4000,"tests":[{"input":"3\n2\nnwi\ninw\n3\ninn\nnww\nwii\n4\nwin\nwww\niii\nnnn\n","output":"0\n2\n2 w 3 i\n3 w 1 n\n3\n2 w 3 i\n2 w 4 n\n3 i 4 n\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"DLetterExchange"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn solve(input: &mut Input, _test_case: usize) {
    // parse letters to indices from 1 to 3a
    let m: usize = input.read();

    // make a matrix 3x3 where cell i, j represents indices of seq. that need letter i and provide letter j
    let mut mt = vec![vec![vec![]; 3]; 3];
    
    // for each sentence fill the matrix
    for i in 0..m {
        let mut s = input.read::<String>().chars().map(|c| match c { 'w' => 0, 'i' => 1, 'n' => 2, _ => panic!("invalid") }).collect::<Vec<_>>();
        s.sort();
        match s[..] {
            [0, 0, 1] => mt[2][0].push(i+1),
            [0, 0, 2] => mt[1][0].push(i+1),
            [0, 1, 1] => mt[2][1].push(i+1),
            [0, 2, 2] => mt[1][2].push(i+1),
            [0, 0, 0] => {
                mt[1][0].push(i+1);
                mt[2][0].push(i+1);
            }
            [1, 1, 2] => mt[0][1].push(i+1),
            [1, 2, 2] => mt[0][2].push(i+1),
            [1, 1, 1] => {
                mt[0][1].push(i+1);
                mt[2][1].push(i+1);
            }
            [2, 2, 2] => {
                mt[0][2].push(i+1);
                mt[1][2].push(i+1);
            },
            _ => ()
        }
    }

    // go through the matrix if i == j skip, otherwise look whether matrix [j][i] is not empty: add these two to the answer, otherwise make a triplet of swaps as an answer

    let mut answer = vec![];
    for i in 0..3 {
        for j in i+1..3 {
            let n = mt[i][j].len().min(mt[j][i].len());
            for k in 0..n {
                answer.push((mt[i][j][k], mt[j][i][k], (j, i)));
            }
            mt[i][j].drain(0..n);
            mt[j][i].drain(0..n);
        }
    }

    let s = mt[0][1].len().min(mt[1][2].len()).min(mt[2][0].len());
    for i in 0..mt[0][1].len() {
        answer.push((mt[0][1][i], mt[1][2][i], (1, 2)));
        answer.push((mt[0][1][i], mt[2][0][i], (2, 0)));
    }
    mt[0][1].drain(0..s);
    mt[1][2].drain(0..s);
    mt[2][0].drain(0..s);

    for i in 0..mt[0][2].len() {
        answer.push((mt[0][2][i], mt[2][1][i], (2, 1)));
        answer.push((mt[0][2][i], mt[1][0][i], (1, 0)));
    }


    out_line!(answer.len());
    for result in answer {
        let letter_1 = match result.2.0 {
            0 => 'w',
            1 => 'i',
            2 => 'n',
            _ => panic!("invalid input")
        };
        let letter_2 = match result.2.1 {
            0 => 'w',
            1 => 'i',
            2 => 'n',
            _ => panic!("invalid input")
        };
        out_line!(result.0, letter_1, result.1, letter_2);
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
