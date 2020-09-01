use crate::*;
use core::iter::repeat;

use rand::prelude::SmallRng;
use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};

pub struct Game<F: FnMut(GameEvent)> {
    callback: F,
    rng: SmallRng,

    // Indexes of MINOS_SRC that has 2 laps of shuffled 7 minos.
    // 7..14 is a next lap.
    minos_index: [usize; 14],

    // Point a next mino.
    // Always 0..6.
    minos_position: usize,

    // A mino user is controlling.
    // Option is just for handling multiple mutable ownership in a struct.
    // Always mino is a Some.
    mino: Option<MinoAggregation>,

    field: Field,

    alive: bool,
    is_landing: bool,
    is_locked: bool,
    landing_wait_count: u8,
    locking_wait_count: u8,
    spun: bool,

    delete_row: [i8; 4],

    score: Score,
}

#[derive(Default, Debug, Clone)]
pub struct Score {
    pub deleted_line: usize,
    pub t_spin1: usize,
    pub t_spin2: usize,
    pub t_spin3: usize,
    pub tetris: usize,
}

impl Score {
    pub fn new() -> Self {
        Self::default()
    }
}

impl<F: FnMut(GameEvent)> Game<F> {
    pub fn new(seed: [u8; 16], callback: F) -> Self {
        let mut rng = SmallRng::from_seed(seed);

        let mut minos_index = [0, 1, 2, 3, 4, 5, 6, 0, 1, 2, 3, 4, 5, 6];

        Game {
            callback,
            rng,

            minos_index,
            minos_position: 0,

            mino: Some(MINOS_SRC[0]),

            field: Field::new(),

            alive: false,
            is_landing: false,
            is_locked: false,
            landing_wait_count: 0,
            locking_wait_count: 0,
            spun: false,

            delete_row: [-1; 4],

            score: Default::default(),
        }
    }

    pub fn start(&mut self) {
        self.minos_index[0..6].shuffle(&mut self.rng);
        self.minos_index[7..14].shuffle(&mut self.rng);
        self.minos_position = 0;

        self.mino = self.new_mino();

        self.field = Field::new();

        self.alive = true;
        self.is_landing = false;
        self.landing_wait_count = 0;
        self.spun = false;

        self.score = Score::new();

        self.inform_next();
        self.inform_score_change();
        self.inform_game_start();
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

    pub fn new_mino(&mut self) -> Option<MinoAggregation> {
        let mino = self.next_mino();
        self.forward_minos_position();
        self.inform_next();
        Some(mino)
    }

    fn next_mino(&self) -> MinoAggregation {
        let index = self.minos_index[self.minos_position as usize] as usize;
        MINOS_SRC[index]
    }

    fn game_over(&mut self) -> Option<MinoAggregation> {
        self.alive = false;
        self.inform_game_over();
        None
    }

    fn inform(&mut self, event: GameEvent) {
        (self.callback)(event)
    }

    fn inform_game_start(&mut self) {
        self.inform(GameEvent::Start);
    }

    fn inform_game_over(&mut self) {
        self.inform(GameEvent::Overflow);
    }

    fn inform_score_change(&mut self) {
        self.inform(GameEvent::ScoreChange(self.score.clone()));
    }

    fn inform_next(&mut self) {
        // to avoid borrow checker
        (self.callback)(GameEvent::Next(
            &self.minos_index[self.minos_position..self.minos_position + 3],
        ));
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
        self.landing_wait_count += 1;

        if self.landing_wait_count > LOCKING_TIME {
            self.lock(mino)
        } else {
            None
        }
    }

    fn reset_previous_state(&mut self) -> Option<MinoAggregation> {
        self.spun = false;
        self.is_landing = false;
        self.landing_wait_count = 0;
        None
    }

    /// return false if all cells are out of display
    fn test_mino_in_display(&mut self, mino: &mut impl MinoFn) -> bool {
        if mino.pos().1 >= FIELD_TOP {
            true
        } else {
            mino.test_with_absolute_cells(|_, y| y >= FIELD_TOP)
        }
    }

    // TODO: detect Tetris or T-spin, etc.
    fn lock(&mut self, mino: &mut impl MinoFn) -> Option<MinoAggregation> {
        if !self.test_mino_in_display(mino) {
            return self.game_over();
        }

        self.is_locked = true;

        let mut filled_count = 0;
        self.delete_row = [-1; 4];

        mino.mut_with_absolute_cells(|x, y| {
            self.field.set(x, y);
            if self.field.is_filled(y) {
                self.delete_row[filled_count] = y;
                filled_count += 1;
            }
        });

        if filled_count != 0 {
            self.score.deleted_line += filled_count;

            if filled_count == 4 {
                self.score.tetris += 1;
            } else if self.spun {
                match filled_count {
                    1 => self.score.t_spin1 += 1,
                    2 => self.score.t_spin2 += 1,
                    3 => self.score.t_spin3 += 1,
                    _ => unreachable!(),
                }
            }

            self.inform_score_change();
        }

        self.reset_previous_state();

        None
    }

    fn erase(&mut self) -> Option<MinoAggregation> {
        if self.locking_wait_count < LOCKING_WAIT_TIME {
            self.locking_wait_count += 1;
            return None;
        }

        self.is_locked = false;
        self.locking_wait_count = 0;

        for row in self.delete_row.iter().rev().copied() {
            if row != -1 {
                self.field.delete(row);
                self.field.float(row);
            }
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
        self.lock(mino)
    }

    fn try_move(&mut self, moving: &mut impl MinoFn, offset: Offset) -> Result<(), ()> {
        moving.offset(offset.plus);
        if moving.test_with_absolute_cells(|x, y| self.field.test(x, y)) {
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
            if !rotated.test_with_absolute_cells(|x, y| self.field.test(x, y)) {
                self.reset_previous_state();
                self.spun = true;
                return Ok(rotated.into());
            }
        }
        Err(())
    }

    pub fn step(&mut self, event: impl Into<Event>) {
        if self.is_locked {
            match self.erase() {
                None => {}
                Some(mino) => self.mino = Some(mino),
            }
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

            Event::Nop => None,

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

    Nop,

    #[cfg(test)]
    Test(TestEvent),
}

pub enum GameEvent<'a> {
    Start,
    ScoreChange(Score),
    Next(&'a [usize]),
    ChangeNextMinoAggregation,
    Overflow,
    Nop,
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

/// This seed generate shuffled index that has MinoI as first.
#[cfg(test)]
pub const TEST_SEED: [u8; 16] = [5; 16];

#[cfg(test)]
mod tests {
    use crate::game::test_uti::{get_mino_pos, print_field};
    use crate::TestEvent::AbsoluteMovement;
    use crate::*;
    use std::prelude::v1::*;

    #[test]
    fn test_step_i() {
        let mut game = Game::new(TEST_SEED, |_| {});
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
        let mut game = Game::new(TEST_SEED, |_| {});
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
