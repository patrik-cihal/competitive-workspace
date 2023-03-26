//{"name":"ksp-throwing","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ksp-throwing"}}}

use std::f64::consts::PI;

use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::{out, out_line};

const g: f64 = 9.81;

fn calculate_angle(x: f64, y: f64, xf: f64) -> f64 {
    let vx = ((g*xf*x-x*x*g)/(2.*y)).sqrt();
    let vy = g*xf/(2.*vx);

    (vy/vx).atan() / (2.*PI) * 360.
}

fn calculate_velocity_from_point(x: f64, y: f64, xf: f64) -> f64 {
    let vx = ((g*xf*x-x*x*g)/(2.*y)).sqrt();
    let vy = g*xf/(2.*vx);

    (vx*vx+vy*vy).sqrt()
}

fn calculate_velocity(angle: f64, xf: f64) -> f64 {
    let angle = angle / 360. * 2. * PI;
    (g*xf/(2.*angle.sin()*angle.cos())).sqrt()
}

fn solve(input: &mut Input, _test_case: usize) {
    let n: usize = input.read();
    let xf: f64 = input.read();

    let mut rectangles: Vec<(f64, f64, f64, f64)> = vec![0; n].into_iter().map(|_| input.read::<(f64, f64, f64, f64)>()).collect();

    let mut occupied = vec![];

    while let Some(rectangle) = rectangles.pop() {
        let (x1, y1, mut x2, y2) = rectangle;
        if x1<xf/2. && x2 > xf/2. {
            rectangles.push((xf/2., y1, x2, y2));
            x2 = xf/2.;
        }
        if x1 < xf/2. {
            occupied.push((calculate_angle(x1, y2, xf), 1));
            occupied.push((calculate_angle(x2, y1, xf), -1));
        }
        else {
            occupied.push((calculate_angle(x1, y1, xf), -1));
            occupied.push((calculate_angle(x2, y2, xf), 1));
        }
    }

    occupied.push((0., -1));
    occupied.push((0.0001, 1));
    occupied.push((89.9999, -1));
    occupied.push((90., 1));
    occupied.sort_by(|a, b| b.partial_cmp(a).unwrap());

    let mut answer = occupied[1].0; 
    let mut sum = 0;
    for i in 1..occupied.len() {
        sum += occupied[i-1].1;
        if sum == 0 {
            let high_angle = occupied[i-1].0;
            let low_angle = occupied[i].0;
            if high_angle >= 45. && low_angle <= 45. {
                answer = 45.;
            }
            else if high_angle > 45. {
                answer = low_angle;
            }
            else {
                if answer-45. > 45.-high_angle {
                    answer = high_angle;
                }
            }
        }
    }

    out_line!(answer, calculate_velocity(answer, xf));
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
