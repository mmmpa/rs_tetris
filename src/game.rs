use crate::*;
use core::iter::repeat;

pub struct Game {
    field: Field,
    minos_position: u8,
    minos_index: [u8; 252],
    ground: bool,
    ground_time: u8,
    lock_time: u8,
}

impl Game {
    pub fn new() -> Self {
        let mut minos_index = [0; 252];

        (0..7).cycle().take(252).enumerate().for_each(|(i, n)| {
            minos_index[i] = n;
        });

        Game {
            field: Field::new(),
            minos_position: 0,
            minos_index,
            ground: false,
            ground_time: 0,
            lock_time: 2,
        }
    }

    pub fn field_mut(&mut self) -> &mut Field {
        &mut self.field
    }

    pub fn rows(&self) -> &[[bool; FIELD_W]; FIELD_H] {
        self.field.rows()
    }

    pub fn step(&mut self, mut mino: Minos, action: Event) -> Minos {
        let next = match &mut mino {
            Minos::Is0(m) => self.action(m, action),
            Minos::Os0(m) => self.action(m, action),
            Minos::Ss0(m) => self.action(m, action),
            Minos::Zs0(m) => self.action(m, action),
            Minos::Js0(m) => self.action(m, action),
            Minos::Ls0(m) => self.action(m, action),
            Minos::Ts0(m) => self.action(m, action),
            Minos::IsR(m) => self.action(m, action),
            Minos::OsR(m) => self.action(m, action),
            Minos::SsR(m) => self.action(m, action),
            Minos::ZsR(m) => self.action(m, action),
            Minos::JsR(m) => self.action(m, action),
            Minos::LsR(m) => self.action(m, action),
            Minos::TsR(m) => self.action(m, action),
            Minos::Is2(m) => self.action(m, action),
            Minos::Os2(m) => self.action(m, action),
            Minos::Ss2(m) => self.action(m, action),
            Minos::Zs2(m) => self.action(m, action),
            Minos::Js2(m) => self.action(m, action),
            Minos::Ls2(m) => self.action(m, action),
            Minos::Ts2(m) => self.action(m, action),
            Minos::IsL(m) => self.action(m, action),
            Minos::OsL(m) => self.action(m, action),
            Minos::SsL(m) => self.action(m, action),
            Minos::ZsL(m) => self.action(m, action),
            Minos::JsL(m) => self.action(m, action),
            Minos::LsL(m) => self.action(m, action),
            Minos::TsL(m) => self.action(m, action),
        };

        match next {
            None => mino,
            Some(mino) => mino,
        }
    }

    pub fn action(&mut self, mut mino: &mut impl MinoFn, action: Event) -> Option<Minos> {
        my_print!("{:?}", action);

        match action {
            Event::Movement(movement) => match movement {
                Movement::Right => {
                    mino.offset((1, 0));
                    if mino.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize))
                    {
                        mino.offset((-1, 0));
                    };
                    None
                }
                Movement::Left => {
                    mino.offset((-1, 0));
                    if mino.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize))
                    {
                        mino.offset((1, 0));
                    }
                    None
                }
                Movement::Down => {
                    mino.offset((0, 1));
                    if mino.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize))
                    {
                        my_print!("hit");
                        mino.offset((0, -1));
                        self.ground = true;
                    }
                    None
                }
                Movement::Fall => {
                    if self.ground {
                        self.lock(mino)
                    } else {
                        self.fall(mino)
                    }
                }
                Movement::None => None,
            },
            Event::Rotation(rotation) => match rotation {
                Rotation::Right => {
                    let (mut right, offsets) = mino.right();
                    self.try_rotate(right, offsets).ok()
                }
                Rotation::Left => {
                    let (mut left, offsets) = mino.left();
                    self.try_rotate(left, offsets).ok()
                }
                Rotation::None => None,
            },
            Event::TimeGo => {
                if self.ground {
                    self.wait_locking(mino)
                } else {
                    self.action(mino, Event::Movement(Movement::Down))
                }
            }
            Event::Nop => None,
        }
    }

    pub fn new_mino(&mut self) -> Option<Minos> {
        let mino = MINOS_SRC[self.minos_index[self.minos_position as usize] as usize];
        self.minos_position += 1;
        Some(mino)
    }

    fn wait_locking(&mut self, mino: &mut impl MinoCore) -> Option<Minos> {
        self.ground_time += 1;
        if self.lock_time > self.ground_time {
            self.lock(mino)
        } else {
            None
        }
    }

    fn lock(&mut self, mino: &mut impl MinoCore) -> Option<Minos> {
        self.ground = false;
        self.ground_time = 0;
        mino.mut_with_absolute_cells(|x, y| self.field.set(x as usize, y as usize));
        self.new_mino()
    }

    fn fall(&mut self, mino: &mut impl MinoCore) -> Option<Minos> {
        loop {
            mino.offset((0, 1));
            if mino.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize)) {
                mino.offset((1, 0));
                break;
            }
        }
        self.ground = true;
        None
    }

    fn try_rotate(&self, mut rotated: impl MinoCore, offsets: &[(i8, i8)]) -> Result<Minos, ()> {
        for offset in offsets {
            rotated.offset(*offset);
            if rotated.test_with_absolute_cells(|x, y| !self.field.test(x as usize, y as usize)) {
                return Ok(rotated.into());
            }
        }
        Err(())
    }
}

#[derive(Debug)]
pub enum Event {
    Movement(Movement),
    Rotation(Rotation),
    TimeGo,
    Nop,
}

#[derive(Debug)]
pub enum Movement {
    Right,
    Left,
    Down,
    Fall,
    None,
}

#[derive(Debug)]
pub enum Rotation {
    Right,
    Left,
    None,
}

#[cfg(test)]
pub mod test_uti {
    use crate::*;
    use core::ops::Range;
    use std::prelude::v1::*;

    // ⬜: mino
    // ⬛: locked
    // 　: blank
    pub fn print_field(game: &Game, mino: &Minos, r: Range<usize>) -> String {
        let mut minos = vec![vec!["⬜"; FIELD_W]; FIELD_H];
        detect_mino(mino, |x, y| {
            if y >= FIELD_H as i8 || x >= FIELD_W as i8 {
                return;
            }
            minos[y as usize][x as usize] = "　"
        });

        game.rows().iter().enumerate().rev().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if *cell {
                    minos[y as usize][x as usize] = "⬛";
                }
            });
        });
        minos
            .into_iter()
            .rev()
            .skip(r.start)
            .take(r.count())
            .flat_map(|mut s| {
                s.push("\n");
                s
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn detect_mino<F>(mino: &Minos, f: F)
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
}

#[cfg(test)]
mod tests {
    use crate::game::test_uti::print_field;
    use crate::*;
    use std::prelude::v1::*;

    #[test]
    fn step() {
        let mut game = Game::new();

        let mut mino = MINOS_SRC[0];

        mino = game.step(mino, Event::Movement(Movement::Down));
        mino = game.step(mino, Event::Movement(Movement::Down));
        mino = game.step(mino, Event::Movement(Movement::Left));
        mino = game.step(mino, Event::Movement(Movement::Left));
        mino = game.step(mino, Event::Movement(Movement::Left));
        mino = game.step(mino, Event::Movement(Movement::Left));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Rotation(Rotation::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Movement(Movement::Right));
        mino = game.step(mino, Event::Rotation(Rotation::Right));

        println!("{}", print_field(&game, &mino, 0..24));
    }
}
