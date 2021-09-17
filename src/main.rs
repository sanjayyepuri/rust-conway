use std::io::{Write, stdout};
use std::{thread};
use std::time::{Duration, Instant};

mod conway;

fn main() {
    let mut stdout = stdout();

    let mut total_micros: u128 = 0;
    let mut total_iters: u128 = 0;

    let mut board = conway::ConwayBoard::new(conway::_GLIDER_GUN);

    write!(stdout, "{}{}press ctrl-c to exit{}",
        termion::clear::All,
        termion::cursor::Goto(1, 1),
        termion::cursor::Hide)  
            .unwrap();
    stdout.flush().unwrap();

    loop {
        write!(stdout,
            "{}{}",
            termion::cursor::Goto(1, 3),
            termion::clear::CurrentLine)
             .unwrap();

        writeln!(stdout, "{}", board).unwrap();
        let now = Instant::now();
        board.next();

        total_micros += now.elapsed().as_micros();
        total_iters += 1;
        writeln!(stdout,"{} avg. microseconds per iter", 
            total_micros / total_iters).unwrap();

        thread::sleep(Duration::from_millis(1000 / 12));

        if total_iters > 60 * 12 { return; }
    }
}
