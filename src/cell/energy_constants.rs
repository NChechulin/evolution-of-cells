use rand::Rng;

#[derive(Copy, Clone)]
pub struct EnergyConstants {
    pub move_forward: f32,
    pub eat: f32,
    pub photosynthesize: f32,
    pub change_line_of_sight: f32,
    pub attach_to_cell: f32,
    pub detach_from_all_cells: f32,
    pub share_energy: f32,
    pub skip_move: f32,
    pub split: f32,
    pub go_to: f32,
}

impl EnergyConstants {
    pub fn new() -> EnergyConstants {
        EnergyConstants {
            move_forward: 1.0,
            eat: 2.0,
            photosynthesize: -2.0,
            change_line_of_sight: 0.5,
            attach_to_cell: 3.0,
            detach_from_all_cells: 2.0,
            share_energy: 0.5,
            skip_move: 0.5,
            split: 1.0,
            go_to: 0.0,
        }
    }

    pub fn mutate_value(val: f32) -> f32 {
        let mut rng = rand::thread_rng();
        let coefficient = rng.gen_range(0.5..2.0);
        let threshold = rng.gen_range(-0.2..0.2);

        val * coefficient + threshold
    }

    pub fn create_mutated_copy(&self) -> EnergyConstants {
        let mut new = *self;
        let mut rng = rand::thread_rng();

        match rng.gen_range(1..11) {
            1 => new.move_forward = EnergyConstants::mutate_value(new.move_forward),
            2 => new.eat = EnergyConstants::mutate_value(new.eat),
            3 => new.photosynthesize = EnergyConstants::mutate_value(new.photosynthesize),
            4 => new.change_line_of_sight = EnergyConstants::mutate_value(new.change_line_of_sight),
            5 => new.attach_to_cell = EnergyConstants::mutate_value(new.attach_to_cell),
            6 => new.detach_from_all_cells = EnergyConstants::mutate_value(new.detach_from_all_cells),
            7 => new.share_energy = EnergyConstants::mutate_value(new.share_energy),
            8 => new.skip_move = EnergyConstants::mutate_value(new.skip_move),
            9 => new.split = EnergyConstants::mutate_value(new.split),
            10 => new.go_to = EnergyConstants::mutate_value(new.go_to),
            _ => {}
        }

        new
    }
}