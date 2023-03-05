//{"name":"E. City Union","group":"Codeforces - Codeforces Round 854 by cybercats (Div. 1 + Div. 2)","url":"https://codeforces.com/contest/1799/problem/E","interactive":false,"timeLimit":1000,"tests":[{"input":"11\n1 3\n#.#\n2 2\n.#\n#.\n4 4\n..##\n...#\n#...\n##..\n6 6\n.##...\n##....\n......\n....##\n.....#\n...###\n6 5\n.#..#\n.#..#\n.#..#\n.#.##\n.#...\n##...\n5 5\n#####\n#...#\n#.#.#\n#...#\n#####\n4 4\n.##.\n##.#\n#.##\n.##.\n5 5\n..###\n....#\n.....\n#....\n#....\n5 6\n.##...\n##....\n#....#\n....##\n...##.\n6 5\n..##.\n...##\n....#\n#....\n##...\n.##..\n5 4\n..##\n..#.\n..#.\n#...\n#...\n","output":"###\n\n.#\n##\n\n..##\n..##\n###.\n##..\n\n.##...\n###...\n..#...\n..####\n...###\n...###\n\n.####\n.####\n.####\n.####\n.#...\n##...\n\n#####\n#####\n#####\n#####\n#####\n\n.##.\n####\n####\n.##.\n\n..###\n..###\n..#..\n###..\n#....\n\n.##...\n###...\n######\n...###\n...##.\n\n..##.\n..###\n..###\n###..\n###..\n.##..\n\n..##\n..#.\n..#.\n###.\n#...\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ECityUnion"}}}

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

fn fill(v: &mut Vec<Vec<bool>>) {
    for row in v.iter_mut() {
        let mut active: Vec<(usize, &bool)> = row.iter().enumerate().filter(|(i, x)| **x).collect();
        if active.len() > 1 {
            for i in active[0].0..active.last().unwrap().0+1 {
                row[i] = true;
            }
        }
    }
    for column in 0..v[0].len() {
        let mut active: Vec<(usize, bool)> = v.iter().enumerate().map(|(i, x)| (i, x[column])).filter(|(i, x)| *x).collect();
        if active.len() > 1 {
            for j in active[0].0..active.last().unwrap().0+1 {
                v[j][column] = true;
            }
        }
    }
}

fn borders(v: &Vec<Vec<bool>>) -> ([usize; 2], [usize; 2]) {
    let mut mn = [v.len(), v[0].len()];
    let mut mx = [0, 0];
    for i in 0..v.len() {
        for j in 0..v[0].len() {
            if v[i][j] {
                mn = [mn[0].min(i), mn[1].min(j)];
                mx = [mx[0].max(i), mx[1].max(j)];
            }
        }
    }
    (mn, mx)
}

fn path(v: &mut Vec<Vec<bool>>, from: [usize; 2], to: [usize; 2]) {
    let [from_x, from_y] = from;
    let [to_x, to_y] = to;
    let mut x = from_x;
    let mut y = from_y;
    while x != to_x {
        if x > to_x {
            x-=1;
        }
        else {
            x += 1;
        }
        v[x][y] = true;
    }
    while y != to_y {
        if y>to_y {
            y-=1;
        }
        else {
            y+=1;
        }
        v[x][y] = true;
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let (n, m): (usize, usize) = input.read();    
    let mut g: Vec<Vec<bool>> = vec![0; n].into_iter().map(|_| input.read_vec::<char>(m).into_iter().map(|c| c=='#').collect()).collect();

    let mut pos = [0, 0];
    for i in 0..n {
        for j in 0..m {
            if g[i][j] {
                pos = [i, j];
            }
        }
    }

    g[pos[0]][pos[1]] = false;
    let mut stack = vec![pos];
    let mut city1 = vec![vec![false; m]; n];

    while let Some(new_pos) = stack.pop() {
        city1[new_pos[0]][new_pos[1]] = true;
        let mut offsets = vec![];
        if new_pos[0] > 0 {
            offsets.push([new_pos[0]-1, new_pos[1]]);
        }
        if new_pos[0]<n-1 {
            offsets.push([new_pos[0]+1, new_pos[1]]);
        }
        if new_pos[1] > 0 {
            offsets.push([new_pos[0], new_pos[1]-1]);
        }
        if new_pos[1] < m-1 {
            offsets.push([new_pos[0], new_pos[1]+1]);
        }

        for offset in offsets {
            if g[offset[0]][offset[1]] {
                g[offset[0]][offset[1]] = false;
                stack.push(offset);
            }
        }
    }

    fill(&mut city1);
    fill(&mut g);

    let mx2 = [n, m];
    let mn2 = [0, 0];


    let (mn1, mx1) = borders(&g);
    let (mn2, mx2) = borders(&city1);


    for i in 0..n {
        for j in 0..m {
            g[i][j] |= city1[i][j];
        }
    }

    if mn1[0] > mx2[0] && mn1[1] > mx2[1] {
        path(&mut g, mx2, mn1);
    }
    else if mn1[0] > mx2[0] && mn2[1] > mx1[1] {
        path(&mut g, [mn1[0], mx1[1]], [mx2[0], mn2[1]]);
    }
    else if mn2[0] > mx1[0] && mn2[1] > mx1[1] {
        path(&mut g, mx1, mn2);
    }
    else if mn2[0] > mx1[0] && mn1[1] > mx2[1] {
        path(&mut g, [mn2[0], mx2[1]], [mx1[0], mn1[1]]);
    }

    fill(&mut g);
    fill(&mut g);

    for row in g {
        out_line!(row.into_iter().map(|x| if x { '#' } else {'.'}).collect::<String>());
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
