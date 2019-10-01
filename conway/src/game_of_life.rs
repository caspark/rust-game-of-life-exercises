// This is the "interface" that you should implement; the UI uses this to drive the behavior of the
// game of life.
pub trait GameOfLife {
    /// Return `Some(true)` if the cell is alive, `Some(false)` if it is dead, or `None` if `x`
    /// and/or `y` are out of bounds.
    fn is_cell_alive(&self, x: usize, y: usize) -> Option<bool>;

    /// Swap the given cell from alive to dead or dead to alive.
    ///
    /// If `x` or `y` is out of bounds, this method should do nothing.
    ///
    /// The origin is assumed to be at the top left, i.e. when `(x, y) == (0, 0)` then the
    /// top-left-most cell should be toggled.
    fn toggle_cell(&mut self, x: usize, y: usize);

    /// Execute one timestep; i.e. cause cells to live, be born, or die based on the amount of
    /// neighbors they have.
    fn tick(&mut self);

    /// Return the current width in cells of the game.
    fn width(&self) -> usize;

    /// Return the current height in cells of the game.
    fn height(&self) -> usize;
}
