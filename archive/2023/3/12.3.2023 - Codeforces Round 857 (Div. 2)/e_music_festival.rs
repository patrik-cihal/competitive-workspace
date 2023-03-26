//{"name":"E. Music Festival","group":"Codeforces - Codeforces Round 857 (Div. 2)","url":"https://codeforces.com/contest/1802/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"2\n4\n5\n4 9 4 6 8\n1\n7\n2\n8 6\n1\n1\n4\n2\n3 4\n2\n1 8\n2\n2 8\n2\n7 9\n","output":"4\n4\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"EMusicFestival"}}}

use std::cmp::Reverse;
use std::collections::BinaryHeap;

use algo_lib::collections::segment_tree::SegmentTree;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};


fn solve<F: Fn(&usize, &usize) -> usize>(input: &mut Input, _test_case: usize, st: &mut SegmentTree<usize, F>) {
    let n: usize = input.read();

    let mut albums = vec![];
    for i in 0..n {
        let k: usize = input.read();
        let mut album = vec![];
        album.push(input.read::<u32>());
        for _ in 1..k {
            let track: u32 = input.read();
            if album.last().unwrap() < &track {
                album.push(track);
            }
        }
        albums.push(album);
    }


    albums.sort_by(|a1, a2| a2.last().unwrap().cmp(a1.last().unwrap()));

    for album in &albums {
        let mut distance = st.query((*album.last().unwrap() as usize+1)..);
        for track in album.into_iter().rev() {
            distance += 1;
            let new_distance = *st.get(*track as usize).max(&distance);
            st.update(*track as usize, new_distance);
        }
    }


    out_line!(st.query(..));

    for album in &albums {
        for track in album {
            st.update(*track as usize, 0);
        }
    }

}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    let mut st = SegmentTree::new(200_002, 0, |&a, &b| if a>b {a} else {b});
    for i in 0usize..t {
        solve(&mut input, i + 1, &mut st);
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
