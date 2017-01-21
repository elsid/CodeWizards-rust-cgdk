#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum ActionType {
    Unknown = -1,
    None = 0,
    Staff = 1,
    MagicMissile = 2,
    FrostBolt = 3,
    Fireball = 4,
    Haste = 5,
    Shield = 6,
    Count = 7,
}
