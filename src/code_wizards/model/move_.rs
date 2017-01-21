use super::action_type::ActionType;
use super::message::Message;
use super::skill_type::SkillType;

#[derive(Clone, Debug, PartialEq)]
pub struct Move {
    speed: f64,
    strafe_speed: f64,
    turn: f64,
    action: ActionType,
    cast_angle: f64,
    min_cast_distance: f64,
    max_cast_distance: f64,
    status_target_id: i64,
    skill_to_learn: SkillType,
    messages: Vec<Message>,
}

impl Move {
    pub fn new() -> Self {
        Move {
            speed: 0.0,
            strafe_speed: 0.0,
            turn: 0.0,
            action: ActionType::Unknown,
            cast_angle: 0.0,
            min_cast_distance: 0.0,
            max_cast_distance: 10000.0,
            status_target_id: -1,
            skill_to_learn: SkillType::Unknown,
            messages: vec![],
        }
    }

    pub fn speed(&self) -> f64 {
        self.speed
    }

    pub fn set_speed(&mut self, value: f64) -> &mut Self {
        self.speed = value;
        self
    }

    pub fn strafe_speed(&self) -> f64 {
        self.strafe_speed
    }

    pub fn set_strafe_speed(&mut self, value: f64) -> &mut Self {
        self.strafe_speed = value;
        self
    }

    pub fn turn(&self) -> f64 {
        self.turn
    }

    pub fn set_turn(&mut self, value: f64) -> &mut Self {
        self.turn = value;
        self
    }

    pub fn action(&self) -> ActionType {
        self.action
    }

    pub fn set_action(&mut self, value: ActionType) -> &mut Self {
        self.action = value;
        self
    }

    pub fn cast_angle(&self) -> f64 {
        self.cast_angle
    }

    pub fn set_cast_angle(&mut self, value: f64) -> &mut Self {
        self.cast_angle = value;
        self
    }

    pub fn min_cast_distance(&self) -> f64 {
        self.min_cast_distance
    }

    pub fn set_min_cast_distance(&mut self, value: f64) -> &mut Self {
        self.min_cast_distance = value;
        self
    }

    pub fn max_cast_distance(&self) -> f64 {
        self.max_cast_distance
    }

    pub fn set_max_cast_distance(&mut self, value: f64) -> &mut Self {
        self.max_cast_distance = value;
        self
    }

    pub fn status_target_id(&self) -> i64 {
        self.status_target_id
    }

    pub fn set_status_target_id(&mut self, value: i64) -> &mut Self {
        self.status_target_id = value;
        self
    }

    pub fn skill_to_learn(&self) -> SkillType {
        self.skill_to_learn
    }

    pub fn set_skill_to_learn(&mut self, value: SkillType) -> &mut Self {
        self.skill_to_learn = value;
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
