use super::player::Player;
use super::wizard::Wizard;
use super::minion::Minion;
use super::projectile::Projectile;
use super::bonus::Bonus;
use super::building::Building;
use super::tree::Tree;

#[derive(Clone, Debug, PartialEq)]
pub struct World {
    pub tick_index: i32,
    pub tick_count: i32,
    pub width: f64,
    pub height: f64,
    pub players: Vec<Player>,
    pub wizards: Vec<Wizard>,
    pub minions: Vec<Minion>,
    pub projectiles: Vec<Projectile>,
    pub bonuses: Vec<Bonus>,
    pub buildings: Vec<Building>,
    pub trees: Vec<Tree>,
}
