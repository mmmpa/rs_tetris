use crate::*;
use core::iter::repeat;

use rand::prelude::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

pub struct Game<F: FnMut(GameEvent)> {
    callback: F,
    rng: SmallRng,

    field: Field,
    is_landing: bool,
    landing_time: u8,
    lock_time: u8,
    mino: Option<MinoAggregation>,
    pub minos_index: [usize; 14],
    pub minos_position: usize,
    spun: bool,

    score: Score,

    alive: bool,
}

#[derive(Default, Debug)]
pub struct Score {
    deleted_line: usize,
    t_spin1: usize,
    t_spin2: usize,
    t_spin3: usize,
    tetris: usize,
}

impl<F: FnMut(GameEvent)> Game<F> {
    pub fn new(seed: [u8; 16], callback: F) -> Self {
        let mut rng = SmallRng::from_seed(seed);

        let mut minos_index = [0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6];

        Game {
            callback,
            field: Field::new(),
            is_landing: false,
            landing_time: 0,
            lock_time: 2,
            mino: Some(MINOS_SRC[0]),
            minos_index,
            minos_position: 0,
            spun: false,
            score: Default::default(),
            rng,
            alive: false,
        }
    }

    pub fn mino(&self) -> &MinoAggregation {
        &self.mino.as_ref().unwrap()
    }

    pub fn field_mut(&mut self) -> &mut Field {
        &mut self.field
    }

    pub fn rows(&self) -> &[[bool; FIELD_W]; FIELD_H] {
        self.field.rows()
    }

    pub fn step(&mut self, event: impl Into<Event>) {
        if !self.alive {
            return;
        }

        let event = event.into();

        let mut mino = self.mino.take().unwrap();
        let next = match &mut mino {
            MinoAggregation::Is0(m) => self.action(m, event),
            MinoAggregation::Os0(m) => self.action(m, event),
            MinoAggregation::Ss0(m) => self.action(m, event),
            MinoAggregation::Zs0(m) => self.action(m, event),
            MinoAggregation::Js0(m) => self.action(m, event),
            MinoAggregation::Ls0(m) => self.action(m, event),
            MinoAggregation::Ts0(m) => self.action(m, event),
            MinoAggregation::IsR(m) => self.action(m, event),
            MinoAggregation::OsR(m) => self.action(m, event),
            MinoAggregation::SsR(m) => self.action(m, event),
            MinoAggregation::ZsR(m) => self.action(m, event),
            MinoAggregation::JsR(m) => self.action(m, event),
            MinoAggregation::LsR(m) => self.action(m, event),
            MinoAggregation::TsR(m) => self.action(m, event),
            MinoAggregation::Is2(m) => self.action(m, event),
            MinoAggregation::Os2(m) => self.action(m, event),
            MinoAggregation::Ss2(m) => self.action(m, event),
            MinoAggregation::Zs2(m) => self.action(m, event),
            MinoAggregation::Js2(m) => self.action(m, event),
            MinoAggregation::Ls2(m) => self.action(m, event),
            MinoAggregation::Ts2(m) => self.action(m, event),
            MinoAggregation::IsL(m) => self.action(m, event),
            MinoAggregation::OsL(m) => self.action(m, event),
            MinoAggregation::SsL(m) => self.action(m, event),
            MinoAggregation::ZsL(m) => self.action(m, event),
            MinoAggregation::JsL(m) => self.action(m, event),
            MinoAggregation::LsL(m) => self.action(m, event),
            MinoAggregation::TsL(m) => self.action(m, event),
        };

        match next {
            None => self.mino = Some(mino),
            Some(new) => self.mino = Some(new),
        };
    }

    pub fn action(&mut self, mut mino: &mut impl MinoFn, event: Event) -> Option<MinoAggregation> {
        // my_print!("{:?}", event);

        match event {
            Event::MoveR => {
                self.try_move(mino, OFFSET_RIGHT);
                None
            }
            Event::MoveL => {
                self.try_move(mino, OFFSET_LEFT);
                None
            }
            Event::MoveDown => {
                if self.is_landing {
                    self.lock(mino)
                } else {
                    self.action(mino, Event::FreeFall)
                }
            }
            Event::Land => {
                if self.is_landing {
                    self.lock(mino)
                } else {
                    self.land(mino)
                }
            }
            Event::RotateR | Event::RotateL => {
                // only for O type mino
                // to detect T-spin MinoO must always fail to rotate
                if !mino.is_rotatable() {
                    return None;
                }

                if event == Event::RotateR {
                    let (mut right, offsets) = mino.right();
                    self.try_rotate(right, offsets).ok()
                } else {
                    let (mut left, offsets) = mino.left();
                    self.try_rotate(left, offsets).ok()
                }
            }
            Event::FreeFall => match self.try_move(mino, OFFSET_DOWN) {
                Ok(_) => self.reset_previous_state(),
                Err(_) => self.wait_locking(mino),
            },
            Event::TimeGo => self.action(mino, Event::FreeFall),

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

    pub fn new_mino(&mut self) -> Option<MinoAggregation> {
        let mino = MINOS_SRC[self.minos_index[self.minos_position as usize] as usize];
        self.forward_minos_position();
        self.inform_next();
        Some(mino)
    }

    fn inform_next(&mut self) {
        (self.callback)(GameEvent::Next(
            &self.minos_index[self.minos_position..self.minos_position + 3],
        ));
    }

    fn inform_game_over(&mut self) {
        (self.callback)(GameEvent::Overflow);
    }

    pub fn start(&mut self) {
        self.minos_index[0..6].shuffle(&mut self.rng);
        self.minos_index[7..14].shuffle(&mut self.rng);
        self.alive = true;

        self.inform_next();
    }

    fn forward_minos_position(&mut self) {
        self.minos_position += 1;

        if self.minos_position != 7 {
            return;
        }

        for i in 0..7 {
            self.minos_index.swap(i, i + 7)
        }

        self.minos_index[7..14].shuffle(&mut self.rng);
        self.minos_position = 0;
    }

    fn wait_locking(&mut self, mino: &mut impl MinoFn) -> Option<MinoAggregation> {
        self.landing_time += 1;

        if self.landing_time > self.lock_time {
            self.lock(mino)
        } else {
            None
        }
    }

    fn reset_previous_state(&mut self) -> Option<MinoAggregation> {
        self.spun = false;
        self.is_landing = false;
        self.landing_time = 0;
        None
    }

    // TODO: detect Tetris or T-spin, etc.
    fn lock(&mut self, mino: &mut impl MinoFn) -> Option<MinoAggregation> {
        if mino.pos().1 < FIELD_TOP {
            let is_mino_in_display = mino.test_with_absolute_cells(|_, y| y >= FIELD_TOP);

            if !is_mino_in_display {
                self.alive = false;
                return None;
            }
        }

        self.reset_previous_state();
        let mut filled_count = 0;
        let mut filled = [0; 4];
        mino.mut_with_absolute_cells(|x, y| {
            if self.field.set(x as usize, y as usize) {
                filled[filled_count] = y as usize;
                filled_count += 1;
            }
        });

        while filled_count > 0 {
            filled_count -= 1;
            let row = filled[filled_count];
            self.field.delete(row);
            self.field.float(row);
        }

        self.new_mino()
    }

    fn land(&mut self, mino: &mut impl MinoFn) -> Option<MinoAggregation> {
        // lock when cannot move down at all
        if let Err(_) = self.try_move(mino, OFFSET_DOWN) {
            return self.lock(mino);
        }

        loop {
            if let Err(_) = self.try_move(mino, OFFSET_DOWN) {
                break;
            }
        }

        self.reset_previous_state();
        self.is_landing = true;
        None
    }

    fn try_move(&mut self, moving: &mut impl MinoFn, offset: Offset) -> Result<(), ()> {
        moving.offset(offset.plus);
        if moving.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize)) {
            moving.offset(offset.minus);
            return Err(());
        }
        Ok(())
    }

    fn try_rotate(
        &mut self,
        mut rotated: impl WithCell + Into<MinoAggregation>,
        offsets: &[(i8, i8)],
    ) -> Result<MinoAggregation, ()> {
        let (x, y) = rotated.pos();
        for (offset_x, offset_y) in offsets {
            rotated.absolute((x + offset_x, y + offset_y));
            if !rotated.test_with_absolute_cells(|x, y| self.field.test(x as usize, y as usize)) {
                self.reset_previous_state();
                self.spun = true;
                return Ok(rotated.into());
            }
        }
        Err(())
    }
}

struct Offset {
    plus: (i8, i8),
    minus: (i8, i8),
}

const OFFSET_RIGHT: Offset = Offset {
    plus: (1, 0),
    minus: (-1, 0),
};

const OFFSET_LEFT: Offset = Offset {
    plus: (-1, 0),
    minus: (1, 0),
};

const OFFSET_DOWN: Offset = Offset {
    plus: (0, 1),
    minus: (0, -1),
};

#[derive(Debug, Eq, PartialEq)]
pub enum Event {
    MoveR,
    MoveL,
    MoveDown,
    Land,

    RotateR,
    RotateL,

    TimeGo,
    FreeFall,

    #[cfg(test)]
    Test(TestEvent),
}

pub enum GameEvent<'a> {
    Locked,
    Next(&'a [usize]),
    ChangeNextMinoAggregation,
    Overflow,
}

#[derive(Debug, Eq, PartialEq)]
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

#[derive(Debug, Eq, PartialEq)]
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
    pub fn print_field<F: FnMut(GameEvent)>(game: &Game<F>, r: Range<usize>) -> String {
        let mut minos = vec![vec!["⬜"; FIELD_W]; FIELD_H];
        mut_with_absolute_cells(&game.mino.unwrap(), |x, y| {
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
            .skip(r.start)
            .take(r.count())
            .flat_map(|mut s| {
                s.push("\n");
                s
            })
            .collect::<Vec<_>>()
            .join("")
    }

    pub fn mut_with_absolute_cells<F>(mino: &MinoAggregation, f: F)
    where
        F: FnMut(i8, i8),
    {
        define_macro_state_method!(mino, mut_with_absolute_cells(f));
    }

    pub fn get_mino_pos(mino: &MinoAggregation) -> (i8, i8) {
        define_macro_state_method!(mino, pos())
    }

    pub fn mino_is_0(mino: &MinoAggregation) -> bool {
        define_macro_state_method!(mino, is_0())
    }

    pub fn mino_is_r(mino: &MinoAggregation) -> bool {
        define_macro_state_method!(mino, is_r())
    }

    pub fn mino_is_l(mino: &MinoAggregation) -> bool {
        define_macro_state_method!(mino, is_l())
    }

    pub fn mino_is_2(mino: &MinoAggregation) -> bool {
        define_macro_state_method!(mino, is_2())
    }
}
#[cfg(test)]
mod tests_no_std {
    use rand::rngs::{SmallRng, StdRng};
    use rand::{CryptoRng, Rng};
    use rand::{RngCore, SeedableRng};

    #[test]
    fn test_shuffle_small() {
        let mut rng = SmallRng::from_seed([0; 16]);
        let mut src = [0, 1, 2, 3, 4, 5, 6];

        shuffle(&mut rng, &mut src);
        assert_eq!([2, 3, 6, 4, 5, 0, 1], src);
        shuffle(&mut rng, &mut src);
        assert_eq!([6, 5, 1, 2, 0, 4, 3], src);
        shuffle(&mut rng, &mut src);
        assert_eq!([5, 1, 4, 6, 2, 3, 0], src);
    }

    #[test]
    fn test_shuffle_std_rng() {
        let mut rng = StdRng::from_seed([0; 32]);
        let mut src = [0, 1, 2, 3, 4, 5, 6];

        shuffle_crypt(&mut rng, &mut src);
        assert_eq!([6, 5, 4, 1, 3, 0, 2], src);
        shuffle_crypt(&mut rng, &mut src);
        assert_eq!([4, 2, 5, 6, 0, 1, 3], src);
        shuffle_crypt(&mut rng, &mut src);
        assert_eq!([3, 4, 1, 0, 5, 2, 6], src);
    }

    fn shuffle<T>(rng: &mut impl Rng, data: &mut [T]) {
        for i in 1..data.len() {
            let j = rng.gen_range(0, i);
            data.swap(i, j);
        }
    }

    fn shuffle_crypt<T>(rng: &mut (impl Rng + CryptoRng), data: &mut [T]) {
        shuffle(rng, data)
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
        let mut game = Game::new([0; 16], |_| {});
        game.start();
        {
            game.step(AbsoluteMovement((0, 2)));
            game.step(Event::MoveL);
            assert_eq!(0, get_mino_pos(game.mino()).0);

            game.step(AbsoluteMovement((9, 3)));
            game.step(Event::MoveR);
            assert_eq!(9, get_mino_pos(game.mino()).0);
        }

        println!("from L");
        {
            game.step(AbsoluteRotation::StateL);
            game.step(AbsoluteMovement((0, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateL);
            println!("to 2");
            println!("{}", print_field(&game, 0..6));

            game.step(AbsoluteRotation::StateL);
            game.step(AbsoluteMovement((0, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateR);
            println!("to 0");
            println!("{}", print_field(&game, 0..6));
        }

        println!("from R");
        {
            game.step(AbsoluteRotation::StateR);
            game.step(AbsoluteMovement((0, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateL);
            println!("to 0");
            println!("{}", print_field(&game, 0..6));

            game.step(AbsoluteRotation::StateR);
            game.step(AbsoluteMovement((0, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateR);
            println!("to 2");
            println!("{}", print_field(&game, 0..6));
        }

        println!("from L");
        {
            game.step(AbsoluteRotation::StateL);
            game.step(AbsoluteMovement((9, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateL);
            println!("to 2");
            println!("{}", print_field(&game, 0..6));

            game.step(AbsoluteRotation::StateL);
            game.step(AbsoluteMovement((9, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateR);
            println!("to 0");
            println!("{}", print_field(&game, 0..6));
        }

        println!("from R");
        {
            game.step(AbsoluteRotation::StateR);
            game.step(AbsoluteMovement((8, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateL);
            println!("to 0");
            println!("{}", print_field(&game, 0..6));

            game.step(AbsoluteRotation::StateR);
            game.step(AbsoluteMovement((8, 1)));
            println!("{}", print_field(&game, 0..6));
            game.step(Event::RotateR);
            println!("to 2");
            println!("{}", print_field(&game, 0..6));
        }
    }
}

#[cfg(test)]
mod only_test_method_tests {
    use crate::game::test_uti::*;
    use crate::TestEvent::AbsoluteMovement;
    use crate::*;
    use std::prelude::v1::*;

    fn assert_0<F: FnMut(GameEvent)>(game: &Game<F>) {
        let s = print_field(&game, 0..6);
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
    }
    fn assert_r<F: FnMut(GameEvent)>(game: &Game<F>) {
        let s = print_field(&game, 0..6);
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
    }
    fn assert_l<F: FnMut(GameEvent)>(game: &Game<F>) {
        let s = print_field(&game, 0..6);
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
    }
    fn assert_2<F: FnMut(GameEvent)>(game: &Game<F>) {
        let s = print_field(&game, 0..6);
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
    }

    #[test]
    fn test_absolute_rotation() {
        let mut game = Game::new([0; 16], |_| {});
        game.start();
        let mut mino = MINOS_SRC[0];

        game.step(AbsoluteMovement((4, 2)));

        // println!("from 0");

        game.step(AbsoluteRotation::State0);
        assert_0(&game);
        game.step(AbsoluteRotation::StateR);
        assert_r(&game);
        game.step(AbsoluteRotation::State0);
        assert_0(&game);
        game.step(AbsoluteRotation::StateL);
        assert_l(&game);
        game.step(AbsoluteRotation::State0);
        assert_0(&game);
        game.step(AbsoluteRotation::State2);
        assert_2(&game);
        game.step(AbsoluteRotation::State0);
        assert_0(&game);

        // println!("from R");

        game.step(AbsoluteRotation::StateR);
        assert_r(&game);
        game.step(AbsoluteRotation::StateL);
        assert_l(&game);
        game.step(AbsoluteRotation::StateR);
        assert_r(&game);
        game.step(AbsoluteRotation::State2);
        assert_2(&game);
        game.step(AbsoluteRotation::StateR);
        assert_r(&game);

        // println!("from L");

        game.step(AbsoluteRotation::StateL);
        assert_l(&game);
        game.step(AbsoluteRotation::State2);
        assert_2(&game);
        game.step(AbsoluteRotation::StateL);
        assert_l(&game);
    }
}
