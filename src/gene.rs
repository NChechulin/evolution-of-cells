enum Gene {
    MoveForward,
    Eat,
    Photosynthesize,
    ChangeLineOfSight(u8),
    AttachToCell,
    DetachFromAllCells,
    ShareEnergy,
    SkipMove,
    GoTo(usize),
}

impl Gene {
    pub fn from(val: u8) -> Gene {
        match val {
            1 => Gene::MoveForward,
            2 => Gene::Eat,
            3 => Gene::Photosynthesize,
            4..=11 => Gene::ChangeLineOfSight(val % 8),
            12 => Gene::AttachToCell,
            13 => Gene::DetachFromAllCells,
            14 => Gene::ShareEnergy,
            15 => Gene::SkipMove,
            _ => Gene::GoTo(val as usize),
        }
    }
}
