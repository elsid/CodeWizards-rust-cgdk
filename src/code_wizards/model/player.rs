use super::faction::Faction;

#[derive(Clone, Debug, PartialEq)]
pub struct Player {
    pub id: i64,
    pub me: bool,
    pub name: String,
    pub strategy_crashed: bool,
    pub score: i32,
    pub faction: Faction,
}
