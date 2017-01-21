use super::lane_type::LaneType;
use super::skill_type::SkillType;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Message {
    pub lane: LaneType,
    pub skill_to_learn: SkillType,
    pub raw_message: Vec<i8>,
}
