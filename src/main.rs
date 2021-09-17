use std::io::{Write, stdout};
use std::{thread, time};

mod conway;

fn main() {
    let mut stdout = stdout();

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

        board.transition();
        thread::sleep(time::Duration::from_millis(1000 / 12))
    }
}

