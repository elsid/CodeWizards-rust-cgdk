use super::circular_unit::CircularUnit;
use super::faction::Faction;
use super::living_unit::LivingUnit;
use super::message::Message;
use super::skill_type::SkillType;
use super::status::Status;
use super::unit::Unit;

#[derive(Clone, Debug, PartialEq)]
pub struct Wizard {
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
    owner_player_id: i64,
    me: bool,
    mana: i32,
    max_mana: i32,
    vision_range: f64,
    cast_range: f64,
    xp: i32,
    level: i32,
    skills: Vec<SkillType>,
    remaining_action_cooldown_ticks: i32,
    remaining_cooldown_ticks_by_action: Vec<i32>,
    master: bool,
    messages: Vec<Message>,
}

impl Wizard {
    pub fn new() -> Self {
        Wizard {
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
            owner_player_id: 0,
            me: false,
            mana: 0,
            max_mana: 0,
            vision_range: 0.0,
            cast_range: 0.0,
            xp: 0,
            level: 0,
            skills: vec![],
            remaining_action_cooldown_ticks: 0,
            remaining_cooldown_ticks_by_action: vec![],
            master: false,
            messages: vec![],
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

    pub fn owner_player_id(&self) -> i64 {
        self.owner_player_id
    }

    pub fn set_owner_player_id(&mut self, value: i64) -> &mut Self {
        self.owner_player_id = value;
        self
    }

    pub fn me(&self) -> bool {
        self.me
    }

    pub fn set_me(&mut self, value: bool) -> &mut Self {
        self.me = value;
        self
    }

    pub fn mana(&self) -> i32 {
        self.mana
    }

    pub fn set_mana(&mut self, value: i32) -> &mut Self {
        self.mana = value;
        self
    }

    pub fn max_mana(&self) -> i32 {
        self.max_mana
    }

    pub fn set_max_mana(&mut self, value: i32) -> &mut Self {
        self.max_mana = value;
        self
    }

    pub fn vision_range(&self) -> f64 {
        self.vision_range
    }

    pub fn set_vision_range(&mut self, value: f64) -> &mut Self {
        self.vision_range = value;
        self
    }

    pub fn cast_range(&self) -> f64 {
        self.cast_range
    }

    pub fn set_cast_range(&mut self, value: f64) -> &mut Self {
        self.cast_range = value;
        self
    }

    pub fn xp(&self) -> i32 {
        self.xp
    }

    pub fn set_xp(&mut self, value: i32) -> &mut Self {
        self.xp = value;
        self
    }

    pub fn level(&self) -> i32 {
        self.level
    }

    pub fn set_level(&mut self, value: i32) -> &mut Self {
        self.level = value;
        self
    }

    pub fn skills(&self) -> &Vec<SkillType> {
        &self.skills
    }

    pub fn set_skills(&mut self, value: Vec<SkillType>) -> &mut Self {
        self.skills = value;
        self
    }

    pub fn remaining_action_cooldown_ticks(&self) -> i32 {
        self.remaining_action_cooldown_ticks
    }

    pub fn set_remaining_action_cooldown_ticks(&mut self, value: i32) -> &mut Self {
        self.remaining_action_cooldown_ticks = value;
        self
    }

    pub fn remaining_cooldown_ticks_by_action(&self) -> &Vec<i32> {
        &self.remaining_cooldown_ticks_by_action
    }

    pub fn set_remaining_cooldown_ticks_by_action(&mut self, value: Vec<i32>) -> &mut Self {
        self.remaining_cooldown_ticks_by_action = value;
        self
    }

    pub fn master(&self) -> bool {
        self.master
    }

    pub fn set_master(&mut self, value: bool) -> &mut Self {
        self.master = value;
        self
    }

    pub fn messages(&self) -> &Vec<Message> {
        &self.messages
    }

    pub fn set_messages(&mut self, value: Vec<Message>) -> &mut Self {
        self.messages = value;
        self
    }
}

unit_impl!(Wizard);
circular_unit_impl!(Wizard);
living_unit_impl!(Wizard);
