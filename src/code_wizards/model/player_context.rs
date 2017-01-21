use super::wizard::Wizard;
use super::world::World;

#[derive(Clone, Debug, PartialEq)]
pub struct PlayerContext {
    pub wizards: Vec<Wizard>,
    pub world: World,
}
