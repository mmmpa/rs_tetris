#![allow(warnings)]

#[macro_use]
extern crate log;

#[macro_use]
extern crate tetris;

use tetris::*;

use std::io::{stdin, stdout, Write};
use std::sync::{Arc, RwLock};
use std::thread::Thread;
use std::{thread, time};
use termion::cursor::Goto;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::*;

const MESSAGE: &str = "Merry Christmas !!";

#[derive(Default)]
pub struct Pressed {
    pub fall: bool,
    pub left: bool,
    pub right: bool,
    pub down: bool,
    pub rotate_l: bool,
    pub rotate_r: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let locker = Arc::new(RwLock::new(true));
    let pressed = Arc::new(RwLock::new(Pressed::default()));

    {
        let locker = locker.clone();
        let pressed = pressed.clone();
        thread::spawn(move || {
            let stdin = stdin();
            let mut stdout = stdout().into_raw_mode().unwrap();
            write!(stdout, "{}{}", clear::All, cursor::Hide).unwrap();
            write!(stdout, "{}Hello world !!", cursor::Goto(1, 1)).unwrap();
            stdout.flush().unwrap();

            for c in stdin.keys() {
                match c {
                    Ok(event::Key::Right) => pressed.write().unwrap().right = true,
                    Ok(event::Key::Left) => pressed.write().unwrap().left = true,
                    Ok(event::Key::Down) => pressed.write().unwrap().down = true,
                    Ok(event::Key::Up) => pressed.write().unwrap().fall = true,
                    Ok(event::Key::Char('a')) => pressed.write().unwrap().rotate_l = true,
                    Ok(event::Key::Char('s')) => pressed.write().unwrap().rotate_r = true,
                    Ok(event::Key::Ctrl('c')) => break,
                    _ => {}
                };
            }
            *locker.write().unwrap() = false
        });
    }

    {
        let locker = locker.clone();
        let pressed = pressed.clone();
        thread::spawn(move || {
            let mut game = Game::new(|event| {});

            let mut stdout = stdout();

            let interval = 10;
            let mut now = 0;
            let ten_millis = time::Duration::from_millis(1000 / 60);

            loop {
                let Pressed {
                    fall,
                    left,
                    right,
                    down,
                    rotate_l,
                    rotate_r,
                } = *pressed.read().unwrap();
                *pressed.write().unwrap() = Default::default();

                if fall {
                    game.step(Event::Land);
                }
                if left {
                    game.step(Event::MoveL);
                }
                if right {
                    game.step(Event::MoveR);
                }
                if down {
                    game.step(Event::MoveDown);
                }
                if rotate_l {
                    game.step(Event::RotateL);
                }
                if rotate_r {
                    game.step(Event::RotateR);
                }

                now += 1;

                if now > interval {
                    game.step(Event::TimeGo);
                    now = 0;
                }

                write!(stdout, "{}{}", Goto(1, 1), print_field(&game)).unwrap();
                thread::sleep(ten_millis);
            }
        });
    }

    while *locker.read().unwrap() {}

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", termion::cursor::Show).unwrap();

    Ok(())
}

// ⬜: mino
// ⬛: locked
// 　: blank
fn print_field<F: FnMut(GameEvent)>(game: &Game<F>) -> String {
    let mut minos = [["　"; FIELD_W]; FIELD_H + 1];

    mut_with_absolute_cells(game.mino(), |x, y| minos[y as usize][x as usize] = "⬜");

    game.rows().iter().enumerate().rev().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, cell)| {
            if *cell {
                minos[y][x] = "⬛";
            }
        });
    });
    minos.last_mut().unwrap().iter_mut().for_each(|c| *c = "──");

    minos.iter_mut().fold(String::new(), |mut a, row| {
        row.iter().fold(a, |a, s| a + s) + "|\n\r"
    })
}

fn print_next(mino: &MinoAggregation) -> String {
    let mut minos = [["　"; 7]; 7];
    mut_with_absolute_cells(mino, |x, y| minos[y as usize][x as usize] = "⬜");

    minos.iter_mut().fold(String::new(), |mut a, row| {
        row.iter().fold(a, |a, s| a + s) + "\n\r"
    })
}

pub fn mut_with_absolute_cells<F>(mino: &MinoAggregation, f: F)
where
    F: FnMut(i8, i8),
{
    define_macro_state_method!(mino, mut_with_absolute_cells(f));
}
