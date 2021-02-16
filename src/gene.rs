enum Gene {
    MoveForward,
    Eat,
    Photosynthesize,
    ChangeLineOfSight(u8),
    AttachToCell,
    DetachFromCells,
    ShareEnergy,
    SkipMove,
    GoTo(usize),
}
