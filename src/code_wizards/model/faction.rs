#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Faction {
    Unknown = -1,
    Academy = 0,
    Renegades = 1,
    Neutral = 2,
    Other = 3,
    Count = 4,
}
