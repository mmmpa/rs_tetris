struct Bar;
struct NotBar;

trait RotType {}

struct Rot0;
struct RotR;
struct RotL;
struct Rot2;

trait RotState {}

struct Srs<T, A, B> {
    t: T,
    a: A,
    b: B,
}

impl<T, A, B> Srs<T, A, B> {
    pub fn new(t: T, a: A, b: B) -> Self {
        Self { t, a, b }
    }
}

impl Srs<NotBar, Rot0, RotR> {
    fn check(&self) -> bool {
        [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]];

        unimplemented!()
    }
}

impl Srs<NotBar, RotR, Rot0> {
    fn check(&self) -> bool {
        [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]];

        unimplemented!()
    }
}

impl Srs<NotBar, RotR, Rot2> {
    fn check(&self) -> bool {
        [[0, 0], [1, 0], [1, -1], [0, 2], [1, 2]];

        unimplemented!()
    }
}

impl Srs<NotBar, Rot2, RotR> {
    fn check(&self) -> bool {
        [[0, 0], [-1, 0], [-1, 1], [0, -2], [-1, -2]];

        unimplemented!()
    }
}

impl Srs<NotBar, Rot2, RotL> {
    fn check(&self) -> bool {
        [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]];

        unimplemented!()
    }
}

impl Srs<NotBar, RotL, Rot2> {
    fn check(&self) -> bool {
        [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]];

        unimplemented!()
    }
}

impl Srs<NotBar, RotL, Rot0> {
    fn check(&self) -> bool {
        [[0, 0], [-1, 0], [-1, -1], [0, 2], [-1, 2]];

        unimplemented!()
    }
}

impl Srs<NotBar, Rot0, RotL> {
    fn check(&self) -> bool {
        [[0, 0], [1, 0], [1, 1], [0, -2], [1, -2]];

        unimplemented!()
    }
}

struct Mino<T> {
    rot: T,
}

type SrsResult<T> = Result<T, ()>;

impl Mino<Rot0> {
    const SRS: Srs<NotBar, Rot0, RotR> = Srs::new(NotBar, Rot0, RotR);
    pub fn right(self) -> Mino<RotR> {
        unimplemented!()
    }
    pub fn left(self) -> Mino<RotL> {
        unimplemented!()
    }
}

impl Mino<RotR> {
    pub fn right(self) -> Mino<Rot2> {
        unimplemented!()
    }
    pub fn left(self) -> Mino<Rot0> {
        unimplemented!()
    }
}

impl Mino<RotL> {
    pub fn right(self) -> Mino<Rot0> {
        unimplemented!()
    }
    pub fn left(self) -> Mino<Rot2> {
        unimplemented!()
    }
}

impl Mino<Rot2> {
    pub fn right(self) -> Mino<RotL> {
        unimplemented!()
    }
    pub fn left(self) -> Mino<RotR> {
        unimplemented!()
    }
}
