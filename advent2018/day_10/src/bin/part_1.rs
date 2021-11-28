use day_10::*;
use std::io::*;
use termion::{
    event::{Event, Key, MouseEvent},
    input::{MouseTerminal, TermRead},
    raw::IntoRawMode,
    screen::AlternateScreen,
    terminal_size,
};
use utils::*;

fn render_galaxy() {
    let (width, height) = terminal_size().unwrap();
    for pt1 in 0..height {
        for pt2 in 0..width {
            print!("{:<1}", ".");
        }
        print!("{:<1}", ".");
        // print!("\n");
        // write!(screen, "").unwrap();
    }
}

fn main() {
    let lasers: Vec<Laser> = read_lines_as();
    // lasers.iter().mao(|laser| {

    // })

    let mut screen = AlternateScreen::from(stdout());
    print!("{}{}", termion::clear::All, termion::cursor::Goto(1, 1));

    let stdin = stdin();
    let mut stdout = stdout().into_raw_mode().unwrap();

    write!(
        stdout,
        "{}{}q to exit. Click, click, click!",
        termion::clear::All,
        termion::cursor::Goto(1, 1)
    )
    .unwrap();
    stdout.flush().unwrap();

    for c in stdin.events() {
        let event = c.unwrap();
        match event {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => match me {
                MouseEvent::Press(_, x, y) => {
                    write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                }
                _ => (),
            },
            _ => {}
        }
        stdout.flush().unwrap();
    }

    // println!("Hello {:<5}!", ".");
    screen.flush().unwrap();
    loop {}
}
