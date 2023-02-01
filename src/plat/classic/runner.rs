use super::config::ExecutionMode;
use super::io::Io;
use super::{checker, solution};

pub fn run() {
    match solution::EXECUTION_MODE {
        ExecutionMode::Solution { multitest } => {
            run_solution(Io::from_env(), multitest);
        }
        ExecutionMode::Checker => {
            run_checker();
        }
    }
}

fn run_solution(mut io: Io, multitest: bool) {
    let test_cnt = if multitest {
        io.reader.read::<usize>()
    } else {
        1
    };
    for _ in 0..test_cnt {
        solution::solve(&mut io);
    }
}

fn run_checker() {
    let (mut driver_io, solution_io) = Io::pipe();
    let solution_handle = std::thread::spawn(move || {
        run_solution(solution_io, true);
    });
    checker::check(&mut Io::from_env(), &mut driver_io);
    solution_handle.join().unwrap();
}
