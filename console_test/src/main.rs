#![allow(warnings)]

#[macro_use]
extern crate log;

use device_query::{DeviceQuery, DeviceState, Keycode, MouseState};
use futures::future::join_all;
use tetris::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    pretty_env_logger::init();

    info!("start game");

    let key = tokio::spawn(async { serve_key().await });
    let game = tokio::spawn(async { serve_game().await });

    join_all(vec![key, game]).await;

    Ok(())
}

async fn serve_game() {
    print!("\x1b[2J");

    let mut game = Game::new();

    let mut mino = game.new_mino().unwrap();

    loop {
        mino = game.step(mino, Event::TimeGo);
        info!("{:?}", mino);
        print_field(&game, &mino);
        tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;
    }
}

async fn serve_key() {
    loop {
        tokio::time::delay_for(tokio::time::Duration::from_millis(50)).await;
    }
}

// ⬜: mino
// ⬛: locked
// 　: blank
fn print_field(game: &Game, mino: &Minos) {
    let mut minos = [[false; FIELD_W]; FIELD_H];
    detect_mino(mino, |x, y| minos[y as usize][x as usize] = true);

    game.field().iter().enumerate().rev().for_each(|(y, row)| {
        row.iter().enumerate().for_each(|(x, cell)| {
            if minos[y][x] {
                print!("⬜")
            } else if *cell {
                print!("⬛")
            } else {
                print!("　")
            }
        });
        print!("|\n");
    });
    info!("printed");
    print!("\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A\x1b[A");
}

fn detect_mino<F>(mino: &Minos, f: F)
where
    F: FnMut(i8, i8),
{
    match mino {
        Minos::Is0(m) => m.mut_with_absolute_cells(f),
        Minos::Os0(m) => m.mut_with_absolute_cells(f),
        Minos::Ss0(m) => m.mut_with_absolute_cells(f),
        Minos::Zs0(m) => m.mut_with_absolute_cells(f),
        Minos::Js0(m) => m.mut_with_absolute_cells(f),
        Minos::Ls0(m) => m.mut_with_absolute_cells(f),
        Minos::Ts0(m) => m.mut_with_absolute_cells(f),
        Minos::IsR(m) => m.mut_with_absolute_cells(f),
        Minos::OsR(m) => m.mut_with_absolute_cells(f),
        Minos::SsR(m) => m.mut_with_absolute_cells(f),
        Minos::ZsR(m) => m.mut_with_absolute_cells(f),
        Minos::JsR(m) => m.mut_with_absolute_cells(f),
        Minos::LsR(m) => m.mut_with_absolute_cells(f),
        Minos::TsR(m) => m.mut_with_absolute_cells(f),
        Minos::Is2(m) => m.mut_with_absolute_cells(f),
        Minos::Os2(m) => m.mut_with_absolute_cells(f),
        Minos::Ss2(m) => m.mut_with_absolute_cells(f),
        Minos::Zs2(m) => m.mut_with_absolute_cells(f),
        Minos::Js2(m) => m.mut_with_absolute_cells(f),
        Minos::Ls2(m) => m.mut_with_absolute_cells(f),
        Minos::Ts2(m) => m.mut_with_absolute_cells(f),
        Minos::IsL(m) => m.mut_with_absolute_cells(f),
        Minos::OsL(m) => m.mut_with_absolute_cells(f),
        Minos::SsL(m) => m.mut_with_absolute_cells(f),
        Minos::ZsL(m) => m.mut_with_absolute_cells(f),
        Minos::JsL(m) => m.mut_with_absolute_cells(f),
        Minos::LsL(m) => m.mut_with_absolute_cells(f),
        Minos::TsL(m) => m.mut_with_absolute_cells(f),
    }
}
