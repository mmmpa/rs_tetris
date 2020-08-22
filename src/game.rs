use crate::*;
use core::iter::repeat;

pub struct Game {
    field: Field,
    minos_position: u8,
    minos_index: [u8; 252],
    minos_src: [Minos; 7],
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
            minos_src: [
                Minos::Is0(MinoState::<MinoI, BarTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
                Minos::Os0(MinoState::<MinoO, NormalTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
                Minos::Ss0(MinoState::<MinoS, NormalTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
                Minos::Zs0(MinoState::<MinoZ, NormalTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
                Minos::Js0(MinoState::<MinoJ, NormalTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
                Minos::Ls0(MinoState::<MinoL, NormalTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
                Minos::Ts0(MinoState::<MinoT, NormalTypeMino, State0>::new_with(
                    3,
                    DISPLAY_FIELD_H as i8,
                )),
            ],
            ground: false,
            ground_time: 0,
            lock_time: 2,
        }
    }

    pub fn field(&self) -> &[[bool; FIELD_W]; FIELD_H] {
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
        let mino = self.minos_src[self.minos_index[self.minos_position as usize] as usize];
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

macro_rules! define_minos {
    ( $( $name:tt => $state:path ),* $(,)? ) => {
        #[derive(Debug,  Copy, Clone)]
        pub enum Minos {
            $( $name($state), )*
        }
        $(
            impl Into<Minos> for $state {
                fn into(self) -> Minos {
                    Minos::$name(self)
                }
            }
        )*
    }
}

define_minos!(
    Is0 => MinoState<MinoI, BarTypeMino, State0>,
    Os0 => MinoState<MinoO, NormalTypeMino, State0>,
    Ss0 => MinoState<MinoS, NormalTypeMino, State0>,
    Zs0 => MinoState<MinoZ, NormalTypeMino, State0>,
    Js0 => MinoState<MinoJ, NormalTypeMino, State0>,
    Ls0 => MinoState<MinoL, NormalTypeMino, State0>,
    Ts0 => MinoState<MinoT, NormalTypeMino, State0>,
    IsR => MinoState<MinoI, BarTypeMino, StateR>,
    OsR => MinoState<MinoO, NormalTypeMino, StateR>,
    SsR => MinoState<MinoS, NormalTypeMino, StateR>,
    ZsR => MinoState<MinoZ, NormalTypeMino, StateR>,
    JsR => MinoState<MinoJ, NormalTypeMino, StateR>,
    LsR => MinoState<MinoL, NormalTypeMino, StateR>,
    TsR => MinoState<MinoT, NormalTypeMino, StateR>,
    Is2 => MinoState<MinoI, BarTypeMino, State2>,
    Os2 => MinoState<MinoO, NormalTypeMino, State2>,
    Ss2 => MinoState<MinoS, NormalTypeMino, State2>,
    Zs2 => MinoState<MinoZ, NormalTypeMino, State2>,
    Js2 => MinoState<MinoJ, NormalTypeMino, State2>,
    Ls2 => MinoState<MinoL, NormalTypeMino, State2>,
    Ts2 => MinoState<MinoT, NormalTypeMino, State2>,
    IsL => MinoState<MinoI, BarTypeMino, StateL>,
    OsL => MinoState<MinoO, NormalTypeMino, StateL>,
    SsL => MinoState<MinoS, NormalTypeMino, StateL>,
    ZsL => MinoState<MinoZ, NormalTypeMino, StateL>,
    JsL => MinoState<MinoJ, NormalTypeMino, StateL>,
    LsL => MinoState<MinoL, NormalTypeMino, StateL>,
    TsL => MinoState<MinoT, NormalTypeMino, StateL>,
);

#[cfg(test)]
mod tests {
    use crate::*;
    use std::prelude::v1::*;

    // ⬜: mino
    // ⬛: locked
    // 　: blank
    fn print_field(game: &Game) {
        game.field().iter().rev().for_each(|row| {
            row.iter()
                .for_each(|cell| if *cell { print!("⬛") } else { print!("⬜") });
            print!("\n");
        });
        println!("printed");
    }

    #[test]
    fn step() {
        let mut game = Game::new();

        let mut mino = game.new_mino().unwrap();

        for _ in 0..22 {
            mino = game.step(mino, Event::TimeGo);
        }

        println!("{:?}", mino)
    }
}
