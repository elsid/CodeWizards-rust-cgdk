#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Unknown = -1,
    Burning = 0,
    Empowered = 1,
    Frozen = 2,
    Hastened = 3,
    Shielded = 4,
    Count = 5,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Status {
    pub id: i64,
    pub type_: Type,
    pub wizard_id: i64,
    pub player_id: i64,
    pub remaining_duration_ticks: i32,
}
