#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum LaneType {
    Unknown = -1,
    Top = 0,
    Middle = 1,
    Bottom = 2,
    Count = 3,
}
