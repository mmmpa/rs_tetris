use crate::*;

pub trait withCell: Position + Cell {
    /// for rendering
    fn mut_with_absolute_cells<F>(&self, mut f: F)
    where
        F: FnMut(i8, i8),
    {
        let (base_x, base_y) = self.pos();
        Self::cells()
            .iter()
            .for_each(|(x, y)| f(base_x + x, base_y + y));
    }

    /// For hit testing.
    /// Returning false means that all cells don't hit.
    fn test_with_absolute_cells<F>(&self, f: F) -> bool
    where
        F: Fn(i8, i8) -> bool,
    {
        let (base_x, base_y) = self.pos();
        for (x, y) in Self::cells().iter() {
            if f(base_x + x, base_y + y) {
                return true;
            }
        }

        false
    }
}

impl<T: Position + Cell> withCell for T {}
