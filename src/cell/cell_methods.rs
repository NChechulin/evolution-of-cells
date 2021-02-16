pub trait CellMethods {
    /// "Kills" the cell and transforms it into food
    fn die(&mut self);

    /// Reduces the cell's energy by desired value
    fn decrease_energy_by(&mut self, by: u8);

    /// Increases the cell's energy by desired value
    fn increase_energy_by(&mut self, by: u8);

    /// Moves the cell towards line_of_sight
    fn move_forward(&mut self);

    /// Try to eat object "in front" of the cell
    fn eat(&mut self);

    /// Generate energy from Sun
    fn photosynthesize(&mut self);

    /// Change the direction cell is pointed at
    fn change_line_of_sight(&mut self, new_direction: u8);

    /// Not sure yet...
    fn attach_to_cell(&mut self);

    /// Not sure yet...
    fn detach_from_all_cells(&mut self);

    /// Share energy among all attached cells.
    /// The energy levels of them become equal
    fn share_energy(&mut self);

    /// Do nothing
    fn skip_move(&mut self);

    /// Execute `genome[new_index]`
    fn go_to(&mut self, new_index: usize);

    /// Execute current gene
    fn execute_gene(&mut self);
}