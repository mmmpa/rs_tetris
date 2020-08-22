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

    pub fn step(&mut self, mut mino: Minos, event: impl Into<Event>) -> Minos {
        let event = event.into();

        let next = match &mut mino {
            Minos::Is0(m) => self.action(m, event),
            Minos::Os0(m) => self.action(m, event),
            Minos::Ss0(m) => self.action(m, event),
            Minos::Zs0(m) => self.action(m, event),
            Minos::Js0(m) => self.action(m, event),
            Minos::Ls0(m) => self.action(m, event),
            Minos::Ts0(m) => self.action(m, event),
            Minos::IsR(m) => self.action(m, event),
            Minos::OsR(m) => self.action(m, event),
            Minos::SsR(m) => self.action(m, event),
            Minos::ZsR(m) => self.action(m, event),
            Minos::JsR(m) => self.action(m, event),
            Minos::LsR(m) => self.action(m, event),
            Minos::TsR(m) => self.action(m, event),
            Minos::Is2(m) => self.action(m, event),
            Minos::Os2(m) => self.action(m, event),
            Minos::Ss2(m) => self.action(m, event),
            Minos::Zs2(m) => self.action(m, event),
            Minos::Js2(m) => self.action(m, event),
            Minos::Ls2(m) => self.action(m, event),
            Minos::Ts2(m) => self.action(m, event),
            Minos::IsL(m) => self.action(m, event),
            Minos::OsL(m) => self.action(m, event),
            Minos::SsL(m) => self.action(m, event),
            Minos::ZsL(m) => self.action(m, event),
            Minos::JsL(m) => self.action(m, event),
            Minos::LsL(m) => self.action(m, event),
            Minos::TsL(m) => self.action(m, event),
        };

        match next {
            None => mino,
            Some(mino) => mino,
        }
    }

    pub fn action(&mut self, mut mino: &mut impl MinoFn, event: Event) -> Option<Minos> {
        // my_print!("{:?}", event);

        match event {
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

            // only for test
            #[cfg(test)]
            Event::Test(test_event) => match test_event {
                TestEvent::AbsoluteMovement(pos) => {
                    mino.absolute(pos);
                    None
                }
                TestEvent::AbsoluteRotation(r) => match r {
                    AbsoluteRotation::State0 => {
                        match (mino.is_0(), mino.is_r(), mino.is_l(), mino.is_2()) {
                            (true, _, _, _) => None,
                            (_, true, _, _) => Some(mino.left().0.into()),
                            (_, _, true, _) => Some(mino.right().0.into()),
                            (_, _, _, true) => Some(mino.right().0.right().0.into()),
                            _ => None,
                        }
                    }
                    AbsoluteRotation::StateR => {
                        match (mino.is_0(), mino.is_r(), mino.is_l(), mino.is_2()) {
                            (true, _, _, _) => Some(mino.right().0.into()),
                            (_, true, _, _) => None,
                            (_, _, true, _) => Some(mino.right().0.right().0.into()),
                            (_, _, _, true) => Some(mino.left().0.into()),
                            _ => None,
                        }
                    }
                    AbsoluteRotation::StateL => {
                        match (mino.is_0(), mino.is_r(), mino.is_l(), mino.is_2()) {
                            (true, _, _, _) => Some(mino.left().0.into()),
                            (_, true, _, _) => Some(mino.left().0.left().0.into()),
                            (_, _, true, _) => None,
                            (_, _, _, true) => Some(mino.right().0.into()),
                            _ => None,
                        }
                    }
                    AbsoluteRotation::State2 => {
                        match (mino.is_0(), mino.is_r(), mino.is_l(), mino.is_2()) {
                            (true, _, _, _) => Some(mino.right().0.right().0.into()),
                            (_, true, _, _) => Some(mino.right().0.into()),
                            (_, _, true, _) => Some(mino.left().0.into()),
                            (_, _, _, true) => None,
                            _ => None,
                        }
                    }
                },
            },
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
            if !rotated.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize)) {
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

    #[cfg(test)]
    Test(TestEvent),
}

#[derive(Debug)]
pub enum Movement {
    Right,
    Left,
    Down,
    Fall,
    None,
}

impl Into<Event> for Movement {
    fn into(self) -> Event {
        Event::Movement(self)
    }
}

#[derive(Debug)]
pub enum Rotation {
    Right,
    Left,
    None,
}

impl Into<Event> for Rotation {
    fn into(self) -> Event {
        Event::Rotation(self)
    }
}

#[derive(Debug)]
pub enum TestEvent {
    AbsoluteMovement((i8, i8)),
    AbsoluteRotation(AbsoluteRotation),
}

#[cfg(test)]
impl Into<Event> for TestEvent {
    fn into(self) -> Event {
        Event::Test(self)
    }
}

#[derive(Debug)]
pub enum AbsoluteRotation {
    State0,
    StateR,
    StateL,
    State2,
}

#[cfg(test)]
impl Into<Event> for AbsoluteRotation {
    fn into(self) -> Event {
        Event::Test(TestEvent::AbsoluteRotation(self))
    }
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
        mut_with_absolute_cells(mino, |x, y| minos[y as usize][x as usize] = "　");

        game.rows().iter().enumerate().rev().for_each(|(y, row)| {
            row.iter().enumerate().for_each(|(x, cell)| {
                if *cell {
                    minos[y as usize][x as usize] = "⬛";
                }
            });
        });
        minos
            .into_iter()
            .skip(r.start)
            .take(r.count())
            .flat_map(|mut s| {
                s.push("\n");
                s
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn mut_with_absolute_cells<F>(mino: &Minos, f: F)
    where
        F: FnMut(i8, i8),
    {
        define_macro_state_method!(mino, mut_with_absolute_cells(f));
    }

    pub fn get_mino_pos(mino: &Minos) -> (i8, i8) {
        define_macro_state_method!(mino, pos())
    }

    pub fn mino_is_0(mino: &Minos) -> bool {
        define_macro_state_method!(mino, is_0())
    }

    pub fn mino_is_r(mino: &Minos) -> bool {
        define_macro_state_method!(mino, is_r())
    }

    pub fn mino_is_l(mino: &Minos) -> bool {
        define_macro_state_method!(mino, is_l())
    }

    pub fn mino_is_2(mino: &Minos) -> bool {
        define_macro_state_method!(mino, is_2())
    }
}

#[cfg(test)]
mod tests {
    use crate::game::test_uti::{get_mino_pos, print_field};
    use crate::TestEvent::AbsoluteMovement;
    use crate::*;
    use std::prelude::v1::*;

    #[test]
    fn test_step_i() {
        let mut game = Game::new();

        let mut mino = MINOS_SRC[0];
        {
            mino = game.step(mino, AbsoluteMovement((0, 20)));
            mino = game.step(mino, Movement::Left);
            assert_eq!(0, get_mino_pos(&mino).0);

            mino = game.step(mino, AbsoluteMovement((6, 20)));
            mino = game.step(mino, Movement::Right);
            assert_eq!(6, get_mino_pos(&mino).0);
        }

        println!("from L");
        {
            mino = game.step(mino, AbsoluteRotation::StateL);
            mino = game.step(mino, AbsoluteMovement((0, 21)));
            mino = game.step(mino, Rotation::Left);
            println!("to 2");
            println!("{}", print_field(&game, &mino, 0..6));

            mino = game.step(mino, AbsoluteRotation::StateL);
            mino = game.step(mino, AbsoluteMovement((0, 21)));
            mino = game.step(mino, Rotation::Right);
            println!("to 0");
            println!("{}", print_field(&game, &mino, 0..6));
        }

        println!("from R");
        {
            mino = game.step(mino, AbsoluteRotation::StateR);
            mino = game.step(mino, AbsoluteMovement((0, 21)));
            println!("{}", print_field(&game, &mino, 0..6));
            mino = game.step(mino, Rotation::Left);
            println!("to 0");
            println!("{}", print_field(&game, &mino, 0..6));

            mino = game.step(mino, AbsoluteRotation::StateR);
            mino = game.step(mino, AbsoluteMovement((0, 21)));
            println!("{}", print_field(&game, &mino, 0..6));
            mino = game.step(mino, Rotation::Right);
            println!("to 2");
            println!("{}", print_field(&game, &mino, 0..6));
        }
    }
}
#[cfg(test)]
mod only_test_method_tests {
    use crate::game::test_uti::*;
    use crate::TestEvent::AbsoluteMovement;
    use crate::*;
    use std::prelude::v1::*;

    #[test]
    fn test_absolute_rotation() {
        let mut game = Game::new();
        let mut mino = MINOS_SRC[0];

        let assert_0 = |game: &Game, mino: &Minos| {
            let s = print_field(&game, mino, 0..6);
            assert_eq!(
                "\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜　　　　⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ",
                s,
                "\n{}",
                s
            );
        };
        let assert_r = |game: &Game, mino: &Minos| {
            let s = print_field(&game, mino, 0..6);
            assert_eq!(
                "\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜　⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜　⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜　⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜　⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ",
                s,
                "\n{}",
                s
            );
        };
        let assert_l = |game: &Game, mino: &Minos| {
            let s = print_field(&game, mino, 0..6);
            assert_eq!(
                "\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜　⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜　⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜　⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜　⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ",
                s,
                "\n{}",
                s
            );
        };
        let assert_2 = |game: &Game, mino: &Minos| {
            let s = print_field(&game, mino, 0..6);
            assert_eq!(
                "\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜　　　　⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                    ⬜⬜⬜⬜⬜⬜⬜⬜⬜⬜\n\
                ",
                s,
                "{}",
                s
            );
        };

        mino = game.step(mino, AbsoluteMovement((3, 2)));

        // println!("from 0");

        mino = game.step(mino, AbsoluteRotation::State0);
        assert!(mino_is_0(&mino));
        assert_0(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::StateR);
        assert!(mino_is_r(&mino));
        assert_r(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::State0);
        assert_0(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::StateL);
        assert_l(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::State0);
        assert_0(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::State2);
        assert_2(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::State0);
        assert_0(&game, &mino);

        // println!("from R");

        mino = game.step(mino, AbsoluteRotation::StateR);
        assert_r(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::StateL);
        assert_l(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::StateR);
        assert_r(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::State2);
        assert_2(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::StateR);
        assert_r(&game, &mino);

        // println!("from L");

        mino = game.step(mino, AbsoluteRotation::StateL);
        assert_l(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::State2);
        assert_2(&game, &mino);
        mino = game.step(mino, AbsoluteRotation::StateL);
        assert_l(&game, &mino);
    }
}
