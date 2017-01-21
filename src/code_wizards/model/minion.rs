use super::circular_unit::CircularUnit;
use super::faction::Faction;
use super::living_unit::LivingUnit;
use super::status::Status;
use super::unit::Unit;

#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum Type {
    Unknown = -1,
    OrdWoodcutter = 0,
    FetishBlowdart = 1,
    Count = 2,
}

#[derive(Clone, Debug, PartialEq)]
pub struct Minion {
    id: i64,
    x: f64,
    y: f64,
    speed_x: f64,
    speed_y: f64,
    angle: f64,
    faction: Faction,
    radius: f64,
    life: i32,
    max_life: i32,
    statuses: Vec<Status>,
    type_: Type,
    vision_range: f64,
    damage: i32,
    cooldown_ticks: i32,
    remaining_action_cooldown_ticks: i32,
}

impl Minion {
    pub fn new() -> Self {
        Minion {
            id: 0,
            x: 0.0,
            y: 0.0,
            speed_x: 0.0,
            speed_y: 0.0,
            angle: 0.0,
            faction: Faction::Unknown,
            radius: 0.0,
            life: 0,
            max_life: 0,
            statuses: vec![],
            type_: Type::Unknown,
            vision_range: 0.0,
            damage: 0,
            cooldown_ticks: 0,
            remaining_action_cooldown_ticks: 0,
        }
    }

    pub fn id(&self) -> i64 {
        self.id
    }

    pub fn set_id(&mut self, value: i64) -> &mut Self {
        self.id = value;
        self
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn set_x(&mut self, value: f64) -> &mut Self {
        self.x = value;
        self
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn set_y(&mut self, value: f64) -> &mut Self {
        self.y = value;
        self
    }

    pub fn speed_x(&self) -> f64 {
        self.speed_x
    }

    pub fn set_speed_x(&mut self, value: f64) -> &mut Self {
        self.speed_x = value;
        self
    }

    pub fn speed_y(&self) -> f64 {
        self.speed_y
    }

    pub fn set_speed_y(&mut self, value: f64) -> &mut Self {
        self.speed_y = value;
        self
    }

    pub fn angle(&self) -> f64 {
        self.angle
    }

    pub fn set_angle(&mut self, value: f64) -> &mut Self {
        self.angle = value;
        self
    }

    pub fn faction(&self) -> Faction {
        self.faction
    }

    pub fn set_faction(&mut self, value: Faction) -> &mut Self {
        self.faction = value;
        self
    }

    pub fn radius(&self) -> f64 {
        self.radius
    }

    pub fn set_radius(&mut self, value: f64) -> &mut Self {
        self.radius = value;
        self
    }

    pub fn life(&self) -> i32 {
        self.life
    }

    pub fn set_life(&mut self, value: i32) -> &mut Self {
        self.life = value;
        self
    }

    pub fn max_life(&self) -> i32 {
        self.max_life
    }

    pub fn set_max_life(&mut self, value: i32) -> &mut Self {
        self.max_life = value;
        self
    }

    pub fn statuses(&self) -> &Vec<Status> {
        &self.statuses
    }

    pub fn set_statuses(&mut self, value: Vec<Status>) -> &mut Self {
        self.statuses = value;
        self
    }

    pub fn type_(&self) -> Type {
        self.type_
    }

    pub fn set_type(&mut self, value: Type) -> &mut Self {
        self.type_ = value;
        self
    }

    pub fn vision_range(&self) -> f64 {
        self.vision_range
    }

    pub fn set_vision_range(&mut self, value: f64) -> &mut Self {
        self.vision_range = value;
        self
    }

    pub fn damage(&self) -> i32 {
        self.damage
    }

    pub fn set_damage(&mut self, value: i32) -> &mut Self {
        self.damage = value;
        self
    }
    pub fn cooldown_ticks(&self) -> i32 {
        self.cooldown_ticks
    }

    pub fn set_cooldown_ticks(&mut self, value: i32) -> &mut Self {
        self.cooldown_ticks = value;
        self
    }

    pub fn remaining_action_cooldown_ticks(&self) -> i32 {
        self.remaining_action_cooldown_ticks
    }

    pub fn set_remaining_action_cooldown_ticks(&mut self, value: i32) -> &mut Self {
        self.remaining_action_cooldown_ticks = value;
        self
    }
}

unit_impl!(Minion);
circular_unit_impl!(Minion);
living_unit_impl!(Minion);
