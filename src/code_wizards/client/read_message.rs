use std::io;
use byteorder::{ByteOrder, ReadBytesExt};
use code_wizards::model::{
    Bonus,
    BonusType,
    Building,
    BuildingType,
    Faction,
    Game,
    LaneType,
    Message as ModelMessage,
    Minion,
    MinionType,
    Player,
    PlayerContext,
    Projectile,
    ProjectileType,
    SkillType,
    Status,
    StatusType,
    Tree,
    Wizard,
    World,
};
use super::message::Message;

pub enum ReadVecTreeResult {
    Values(Vec<Tree>),
    UseCached,
}

pub trait ReadMessage: ReadBytesExt {
    fn read_message<B: ByteOrder>(&mut self) -> io::Result<Message> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_message_id()) {
            0 => self.read_message_unknown_message(),
            1 => self.read_message_game_over(),
            2 => self.read_message_authentication_token::<B>(),
            3 => self.read_message_team_size::<B>(),
            4 => self.read_message_protocol_version::<B>(),
            5 => self.read_message_game_context::<B>(),
            6 => self.read_message_player_context::<B>(),
            7 => self.read_message_moves_message(),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_message error: invalid message id: {}", v)))
        }
    }

    fn read_message_id(&mut self) -> io::Result<i8> {
        self.read_i8()
    }

    fn read_message_unknown_message(&mut self) -> io::Result<Message> {
        unimplemented!()
    }

    fn read_message_game_over(&mut self) -> io::Result<Message> {
        Ok(Message::GameOver)
    }

    fn read_message_authentication_token<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::AuthenticationToken(try!(self.read_string::<B>())))
    }

    fn read_message_team_size<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::TeamSize(try!(self.read_i32::<B>())))
    }

    fn read_message_protocol_version<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::ProtocolVersion(try!(self.read_i32::<B>())))
    }

    fn read_message_game_context<B: ByteOrder>(&mut self) -> io::Result<Message> {
        Ok(Message::GameContext(try!(self.read_game::<B>())))
    }

    fn read_message_player_context<B: ByteOrder>(&mut self) -> io::Result<Message> {
        match try!(self.read_player_context::<B>()) {
            (v, false) => Ok(Message::PlayerContext(v)),
            (v, true) => Ok(Message::PlayerContextWithoutTrees(v)),
        }
    }

    fn read_message_moves_message(&mut self) -> io::Result<Message> {
        unimplemented!()
    }

    fn read_game<B: ByteOrder>(&mut self) -> io::Result<Game> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_game error: value is false"));
        }

        let result = Game {
            random_seed: try!(self.read_i64::<B>()),
            tick_count: try!(self.read_i32::<B>()),
            map_size: try!(self.read_f64::<B>()),
            skills_enabled: try!(self.read_bool()),
            raw_messages_enabled: try!(self.read_bool()),
            friendly_fire_damage_factor: try!(self.read_f64::<B>()),
            building_damage_score_factor: try!(self.read_f64::<B>()),
            building_elimination_score_factor: try!(self.read_f64::<B>()),
            minion_damage_score_factor: try!(self.read_f64::<B>()),
            minion_elimination_score_factor: try!(self.read_f64::<B>()),
            wizard_damage_score_factor: try!(self.read_f64::<B>()),
            wizard_elimination_score_factor: try!(self.read_f64::<B>()),
            team_working_score_factor: try!(self.read_f64::<B>()),
            victory_score: try!(self.read_i32::<B>()),
            score_gain_range: try!(self.read_f64::<B>()),
            raw_message_max_length: try!(self.read_i32::<B>()),
            raw_message_transmission_speed: try!(self.read_f64::<B>()),
            wizard_radius: try!(self.read_f64::<B>()),
            wizard_cast_range: try!(self.read_f64::<B>()),
            wizard_vision_range: try!(self.read_f64::<B>()),
            wizard_forward_speed: try!(self.read_f64::<B>()),
            wizard_backward_speed: try!(self.read_f64::<B>()),
            wizard_strafe_speed: try!(self.read_f64::<B>()),
            wizard_base_life: try!(self.read_i32::<B>()),
            wizard_life_growth_per_level: try!(self.read_i32::<B>()),
            wizard_base_mana: try!(self.read_i32::<B>()),
            wizard_mana_growth_per_level: try!(self.read_i32::<B>()),
            wizard_base_life_regeneration: try!(self.read_f64::<B>()),
            wizard_life_regeneration_growth_per_level: try!(self.read_f64::<B>()),
            wizard_base_mana_regeneration: try!(self.read_f64::<B>()),
            wizard_mana_regeneration_growth_per_level: try!(self.read_f64::<B>()),
            wizard_max_turn_angle: try!(self.read_f64::<B>()),
            wizard_max_resurrection_delay_ticks: try!(self.read_i32::<B>()),
            wizard_min_resurrection_delay_ticks: try!(self.read_i32::<B>()),
            wizard_action_cooldown_ticks: try!(self.read_i32::<B>()),
            staff_cooldown_ticks: try!(self.read_i32::<B>()),
            magic_missile_cooldown_ticks: try!(self.read_i32::<B>()),
            frost_bolt_cooldown_ticks: try!(self.read_i32::<B>()),
            fireball_cooldown_ticks: try!(self.read_i32::<B>()),
            haste_cooldown_ticks: try!(self.read_i32::<B>()),
            shield_cooldown_ticks: try!(self.read_i32::<B>()),
            magic_missile_manacost: try!(self.read_i32::<B>()),
            frost_bolt_manacost: try!(self.read_i32::<B>()),
            fireball_manacost: try!(self.read_i32::<B>()),
            haste_manacost: try!(self.read_i32::<B>()),
            shield_manacost: try!(self.read_i32::<B>()),
            staff_damage: try!(self.read_i32::<B>()),
            staff_sector: try!(self.read_f64::<B>()),
            staff_range: try!(self.read_f64::<B>()),
            level_up_xp_values: try!(self.read_vec_i32::<B>()),
            minion_radius: try!(self.read_f64::<B>()),
            minion_vision_range: try!(self.read_f64::<B>()),
            minion_speed: try!(self.read_f64::<B>()),
            minion_max_turn_angle: try!(self.read_f64::<B>()),
            minion_life: try!(self.read_i32::<B>()),
            faction_minion_appearance_interval_ticks: try!(self.read_i32::<B>()),
            orc_woodcutter_action_cooldown_ticks: try!(self.read_i32::<B>()),
            orc_woodcutter_damage: try!(self.read_i32::<B>()),
            orc_woodcutter_attack_sector: try!(self.read_f64::<B>()),
            orc_woodcutter_attack_range: try!(self.read_f64::<B>()),
            fetish_blowdart_action_cooldown_ticks: try!(self.read_i32::<B>()),
            fetish_blowdart_attack_range: try!(self.read_f64::<B>()),
            fetish_blowdart_attack_sector: try!(self.read_f64::<B>()),
            bonus_radius: try!(self.read_f64::<B>()),
            bonus_appearance_interval_ticks: try!(self.read_i32::<B>()),
            bonus_score_amount: try!(self.read_i32::<B>()),
            dart_radius: try!(self.read_f64::<B>()),
            dart_speed: try!(self.read_f64::<B>()),
            dart_direct_damage: try!(self.read_i32::<B>()),
            magic_missile_radius: try!(self.read_f64::<B>()),
            magic_missile_speed: try!(self.read_f64::<B>()),
            magic_missile_direct_damage: try!(self.read_i32::<B>()),
            frost_bolt_radius: try!(self.read_f64::<B>()),
            frost_bolt_speed: try!(self.read_f64::<B>()),
            frost_bolt_direct_damage: try!(self.read_i32::<B>()),
            fireball_radius: try!(self.read_f64::<B>()),
            fireball_speed: try!(self.read_f64::<B>()),
            fireball_explosion_max_damage_range: try!(self.read_f64::<B>()),
            fireball_explosion_min_damage_range: try!(self.read_f64::<B>()),
            fireball_explosion_max_damage: try!(self.read_i32::<B>()),
            fireball_explosion_min_damage: try!(self.read_i32::<B>()),
            guardian_tower_radius: try!(self.read_f64::<B>()),
            guardian_tower_vision_range: try!(self.read_f64::<B>()),
            guardian_tower_life: try!(self.read_f64::<B>()),
            guardian_tower_attack_range: try!(self.read_f64::<B>()),
            guardian_tower_damage: try!(self.read_i32::<B>()),
            guardian_tower_cooldown_ticks: try!(self.read_i32::<B>()),
            faction_base_radius: try!(self.read_f64::<B>()),
            faction_base_vision_range: try!(self.read_f64::<B>()),
            faction_base_life: try!(self.read_f64::<B>()),
            faction_base_attack_range: try!(self.read_f64::<B>()),
            faction_base_damage: try!(self.read_i32::<B>()),
            faction_base_cooldown_ticks: try!(self.read_i32::<B>()),
            burning_duration_ticks: try!(self.read_i32::<B>()),
            burning_summary_damage: try!(self.read_i32::<B>()),
            empowered_duration_ticks: try!(self.read_i32::<B>()),
            empowered_damage_factor: try!(self.read_f64::<B>()),
            frozen_duration_ticks: try!(self.read_i32::<B>()),
            hastened_duration_ticks: try!(self.read_i32::<B>()),
            hastened_bonus_duration_factor: try!(self.read_f64::<B>()),
            hastened_movement_bonus_factor: try!(self.read_f64::<B>()),
            hastened_rotation_bonus_factor: try!(self.read_f64::<B>()),
            shielded_duration_ticks: try!(self.read_i32::<B>()),
            shielded_bonus_duration_factor: try!(self.read_f64::<B>()),
            shielded_direct_damage_absorption_factor: try!(self.read_f64::<B>()),
            aura_skill_range: try!(self.read_f64::<B>()),
            range_bonus_per_skill_level: try!(self.read_f64::<B>()),
            magical_damage_bonus_per_skill_level: try!(self.read_i32::<B>()),
            staff_damage_bonus_per_skill_level: try!(self.read_i32::<B>()),
            movement_bonus_factor_per_skill_level: try!(self.read_f64::<B>()),
            magical_damage_absorption_per_skill_level: try!(self.read_i32::<B>()),
        };

        Ok(result)
    }

    fn read_player_context<B: ByteOrder>(&mut self) -> io::Result<(Option<PlayerContext>, bool)> {
        if !try!(self.read_bool()) {
            return Ok((None, false));
        }

        let mut use_cached_trees = false;
        let result = PlayerContext {
            wizards: try!(self.read_vec_wizard::<B>()),
            world: match try!(self.read_world::<B>()) {
                (v, false) => v,
                (v, true) => {
                    use_cached_trees = true;
                    v
                }
            },
        };

        Ok((Some(result), use_cached_trees))
    }

    fn read_world<B: ByteOrder>(&mut self) -> io::Result<(World, bool)> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_world error: value is false"));
        }

        let mut use_cached_trees = false;
        let world = World {
            tick_index: try!(self.read_i32::<B>()),
            tick_count: try!(self.read_i32::<B>()),
            width: try!(self.read_f64::<B>()),
            height: try!(self.read_f64::<B>()),
            players: try!(self.read_vec_player::<B>()),
            wizards: try!(self.read_vec_wizard::<B>()),
            minions: try!(self.read_vec_minion::<B>()),
            projectiles: try!(self.read_vec_projectile::<B>()),
            bonuses: try!(self.read_vec_bonus::<B>()),
            buildings: try!(self.read_vec_building::<B>()),
            trees: match try!(self.read_vec_tree::<B>()) {
                ReadVecTreeResult::Values(v) => v,
                ReadVecTreeResult::UseCached => {
                    use_cached_trees = true;
                    vec![]
                }
            },
        };

        Ok((world, use_cached_trees))
    }

    fn read_player<B: ByteOrder>(&mut self) -> io::Result<Player> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_player error: value is false"));
        }

        let result = Player {
            id: try!(self.read_i64::<B>()),
            me: try!(self.read_bool()),
            name: try!(self.read_string::<B>()),
            strategy_crashed: try!(self.read_bool()),
            score: try!(self.read_i32::<B>()),
            faction: try!(self.read_faction()),
        };

        Ok(result)
    }

    fn read_wizard<B: ByteOrder>(&mut self) -> io::Result<Wizard> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_wizard error: value is false"));
        }

        let mut result = Wizard::new();

        result.set_id(try!(self.read_i64::<B>()))
            .set_x(try!(self.read_f64::<B>()))
            .set_y(try!(self.read_f64::<B>()))
            .set_speed_x(try!(self.read_f64::<B>()))
            .set_speed_y(try!(self.read_f64::<B>()))
            .set_angle(try!(self.read_f64::<B>()))
            .set_faction(try!(self.read_faction()))
            .set_radius(try!(self.read_f64::<B>()))
            .set_life(try!(self.read_i32::<B>()))
            .set_max_life(try!(self.read_i32::<B>()))
            .set_statuses(try!(self.read_vec_status::<B>()))
            .set_owner_player_id(try!(self.read_i64::<B>()))
            .set_me(try!(self.read_bool()))
            .set_mana(try!(self.read_i32::<B>()))
            .set_max_mana(try!(self.read_i32::<B>()))
            .set_vision_range(try!(self.read_f64::<B>()))
            .set_cast_range(try!(self.read_f64::<B>()))
            .set_xp(try!(self.read_i32::<B>()))
            .set_level(try!(self.read_i32::<B>()))
            .set_skills(try!(self.read_vec_skill_type::<B>()))
            .set_remaining_action_cooldown_ticks(try!(self.read_i32::<B>()))
            .set_remaining_cooldown_ticks_by_action(try!(self.read_vec_i32::<B>()))
            .set_master(try!(self.read_bool()))
            .set_messages(try!(self.read_vec_model_message::<B>()));

        Ok(result)
    }

    fn read_minion<B: ByteOrder>(&mut self) -> io::Result<Minion> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_minion error: value is false"));
        }

        let mut result = Minion::new();

        result.set_id(try!(self.read_i64::<B>()))
            .set_x(try!(self.read_f64::<B>()))
            .set_y(try!(self.read_f64::<B>()))
            .set_speed_x(try!(self.read_f64::<B>()))
            .set_speed_y(try!(self.read_f64::<B>()))
            .set_angle(try!(self.read_f64::<B>()))
            .set_faction(try!(self.read_faction()))
            .set_radius(try!(self.read_f64::<B>()))
            .set_life(try!(self.read_i32::<B>()))
            .set_max_life(try!(self.read_i32::<B>()))
            .set_statuses(try!(self.read_vec_status::<B>()))
            .set_type(try!(self.read_minion_type()))
            .set_vision_range(try!(self.read_f64::<B>()))
            .set_damage(try!(self.read_i32::<B>()))
            .set_cooldown_ticks(try!(self.read_i32::<B>()))
            .set_remaining_action_cooldown_ticks(try!(self.read_i32::<B>()));

        Ok(result)
    }

    fn read_projectile<B: ByteOrder>(&mut self) -> io::Result<Projectile> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_projectile error: value is false"));
        }

        let mut result = Projectile::new();

        result.set_id(try!(self.read_i64::<B>()))
            .set_x(try!(self.read_f64::<B>()))
            .set_y(try!(self.read_f64::<B>()))
            .set_speed_x(try!(self.read_f64::<B>()))
            .set_speed_y(try!(self.read_f64::<B>()))
            .set_angle(try!(self.read_f64::<B>()))
            .set_faction(try!(self.read_faction()))
            .set_radius(try!(self.read_f64::<B>()))
            .set_type(try!(self.read_projectile_type()))
            .set_owner_unit_id(try!(self.read_i64::<B>()))
            .set_owner_player_id(try!(self.read_i64::<B>()));

        Ok(result)
    }

    fn read_bonus<B: ByteOrder>(&mut self) -> io::Result<Bonus> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_bonus error: value is false"));
        }

        let mut result = Bonus::new();

        result.set_id(try!(self.read_i64::<B>()))
            .set_x(try!(self.read_f64::<B>()))
            .set_y(try!(self.read_f64::<B>()))
            .set_speed_x(try!(self.read_f64::<B>()))
            .set_speed_y(try!(self.read_f64::<B>()))
            .set_angle(try!(self.read_f64::<B>()))
            .set_faction(try!(self.read_faction()))
            .set_radius(try!(self.read_f64::<B>()))
            .set_type(try!(self.read_bonus_type()));

        Ok(result)
    }

    fn read_building<B: ByteOrder>(&mut self) -> io::Result<Building> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_building error: value is false"));
        }

        let mut result = Building::new();

        result.set_id(try!(self.read_i64::<B>()))
            .set_x(try!(self.read_f64::<B>()))
            .set_y(try!(self.read_f64::<B>()))
            .set_speed_x(try!(self.read_f64::<B>()))
            .set_speed_y(try!(self.read_f64::<B>()))
            .set_angle(try!(self.read_f64::<B>()))
            .set_faction(try!(self.read_faction()))
            .set_radius(try!(self.read_f64::<B>()))
            .set_life(try!(self.read_i32::<B>()))
            .set_max_life(try!(self.read_i32::<B>()))
            .set_statuses(try!(self.read_vec_status::<B>()))
            .set_type(try!(self.read_building_type()))
            .set_vision_range(try!(self.read_f64::<B>()))
            .set_attack_range(try!(self.read_f64::<B>()))
            .set_damage(try!(self.read_i32::<B>()))
            .set_cooldown_ticks(try!(self.read_i32::<B>()))
            .set_remaining_action_cooldown_ticks(try!(self.read_i32::<B>()));

        Ok(result)
    }

    fn read_tree<B: ByteOrder>(&mut self) -> io::Result<Tree> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_tree error: value is false"));
        }

        let mut result = Tree::new();

        result.set_id(try!(self.read_i64::<B>()))
            .set_x(try!(self.read_f64::<B>()))
            .set_y(try!(self.read_f64::<B>()))
            .set_speed_x(try!(self.read_f64::<B>()))
            .set_speed_y(try!(self.read_f64::<B>()))
            .set_angle(try!(self.read_f64::<B>()))
            .set_faction(try!(self.read_faction()))
            .set_radius(try!(self.read_f64::<B>()))
            .set_life(try!(self.read_i32::<B>()))
            .set_max_life(try!(self.read_i32::<B>()))
            .set_statuses(try!(self.read_vec_status::<B>()));

        Ok(result)
    }

    fn read_status<B: ByteOrder>(&mut self) -> io::Result<Status> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_status error: value is false"));
        }

        let result = Status {
            id: try!(self.read_i64::<B>()),
            type_: try!(self.read_status_type()),
            wizard_id: try!(self.read_i64::<B>()),
            player_id: try!(self.read_i64::<B>()),
            remaining_duration_ticks: try!(self.read_i32::<B>()),
        };

        Ok(result)
    }

    fn read_model_message<B: ByteOrder>(&mut self) -> io::Result<ModelMessage> {
        use std::io::{Error, ErrorKind};

        if !try!(self.read_bool()) {
            return Err(Error::new(ErrorKind::Other, "ReadMessage::read_model_message error: value is false"));
        }

        let result = ModelMessage {
            lane: try!(self.read_lane_type()),
            skill_to_learn: try!(self.read_skill_type()),
            raw_message: try!(self.read_vec_i8::<B>()),
        };

        Ok(result)
    }

    fn read_bonus_type(&mut self) -> io::Result<BonusType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(BonusType::Unknown),
            0 => Ok(BonusType::Empower),
            1 => Ok(BonusType::Haste),
            2 => Ok(BonusType::Shield),
            3 => Ok(BonusType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_bonus_type error: invalid BonusType value: {}", v))),
        }
    }

    fn read_building_type(&mut self) -> io::Result<BuildingType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(BuildingType::Unknown),
            0 => Ok(BuildingType::GuardianTower),
            1 => Ok(BuildingType::FactionBase),
            2 => Ok(BuildingType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_building_type error: invalid BuildingType value: {}", v))),
        }
    }

    fn read_faction(&mut self) -> io::Result<Faction> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(Faction::Unknown),
            0 => Ok(Faction::Academy),
            1 => Ok(Faction::Renegades),
            2 => Ok(Faction::Neutral),
            3 => Ok(Faction::Other),
            4 => Ok(Faction::Count),
            v => Err(Error::new(ErrorKind::Other, format!("Invalid Faction value: {}", v))),
        }
    }

    fn read_lane_type(&mut self) -> io::Result<LaneType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(LaneType::Unknown),
            0 => Ok(LaneType::Top),
            1 => Ok(LaneType::Middle),
            2 => Ok(LaneType::Bottom),
            3 => Ok(LaneType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_lane_type error: invalid LaneType value: {}", v))),
        }
    }

    fn read_minion_type(&mut self) -> io::Result<MinionType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(MinionType::Unknown),
            0 => Ok(MinionType::OrdWoodcutter),
            1 => Ok(MinionType::FetishBlowdart),
            2 => Ok(MinionType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_minion_type error: invalid MinionType value: {}", v))),
        }
    }

    fn read_projectile_type(&mut self) -> io::Result<ProjectileType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(ProjectileType::Unknown),
            0 => Ok(ProjectileType::MagicMissile),
            1 => Ok(ProjectileType::FrostBolt),
            2 => Ok(ProjectileType::Fireball),
            3 => Ok(ProjectileType::Dart),
            4 => Ok(ProjectileType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_projectile_type error: invalid ProjectileType value: {}", v))),
        }
    }

    fn read_skill_type(&mut self) -> io::Result<SkillType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(SkillType::Unknown),
            0 => Ok(SkillType::RangeBonusPassive1),
            1 => Ok(SkillType::RangeBonusAura1),
            2 => Ok(SkillType::RangeBonusPassive2),
            3 => Ok(SkillType::RangeBonusAura2),
            4 => Ok(SkillType::AdvancedMagicMissile),
            5 => Ok(SkillType::MagicalDamageBonusPassive1),
            6 => Ok(SkillType::MagicalDamageBonusAura1),
            7 => Ok(SkillType::MagicalDamageBonusPassive2),
            8 => Ok(SkillType::MagicalDamageBonusAura2),
            9 => Ok(SkillType::FrostBolt),
            10 => Ok(SkillType::StaffDamageBonusPassive1),
            11 => Ok(SkillType::StaffDamageBonusAura1),
            12 => Ok(SkillType::StaffDamageBonusPassive2),
            13 => Ok(SkillType::StaffDamageBonusAura2),
            14 => Ok(SkillType::Fireball),
            15 => Ok(SkillType::MovementBonusFactorPassive1),
            16 => Ok(SkillType::MovementBonusFactorAura1),
            17 => Ok(SkillType::MovementBonusFactorPassive2),
            18 => Ok(SkillType::MovementBonusFactorAura2),
            19 => Ok(SkillType::Haste),
            20 => Ok(SkillType::MagicalDamageAbsorptionPassive1),
            21 => Ok(SkillType::MagicalDamageAbsorptionAura1),
            22 => Ok(SkillType::MagicalDamageAbsorptionPassive2),
            23 => Ok(SkillType::MagicalDamageAbsorptionAura2),
            24 => Ok(SkillType::Shield),
            25 => Ok(SkillType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_skill_type error: invalid SkillType value: {}", v))),
        }
    }

    fn read_status_type(&mut self) -> io::Result<StatusType> {
        use std::io::{Error, ErrorKind};
        match try!(self.read_i8()) {
            -1 => Ok(StatusType::Unknown),
            0 => Ok(StatusType::Burning),
            1 => Ok(StatusType::Empowered),
            2 => Ok(StatusType::Frozen),
            3 => Ok(StatusType::Hastened),
            4 => Ok(StatusType::Shielded),
            5 => Ok(StatusType::Count),
            v => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_status_type error: invalid StatusType value: {}", v))),
        }
    }

    fn read_string<B: ByteOrder>(&mut self) -> io::Result<String> {
        use std::io::{Error, ErrorKind};
        let buf = try!(self.read_vec_u8::<B>());
        match String::from_utf8(buf) {
            Ok(v) => Ok(v),
            Err(v) => Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_string error: {:?}", v))),
        }
    }

    fn read_vec_player<B: ByteOrder>(&mut self) -> io::Result<Vec<Player>> {
        self.read_vec::<B, _, _>(|s| s.read_player::<B>())
    }

    fn read_vec_wizard<B: ByteOrder>(&mut self) -> io::Result<Vec<Wizard>> {
        self.read_vec::<B, _, _>(|s| s.read_wizard::<B>())
    }

    fn read_vec_minion<B: ByteOrder>(&mut self) -> io::Result<Vec<Minion>> {
        self.read_vec::<B, _, _>(|s| s.read_minion::<B>())
    }

    fn read_vec_projectile<B: ByteOrder>(&mut self) -> io::Result<Vec<Projectile>> {
        self.read_vec::<B, _, _>(|s| s.read_projectile::<B>())
    }

    fn read_vec_bonus<B: ByteOrder>(&mut self) -> io::Result<Vec<Bonus>> {
        self.read_vec::<B, _, _>(|s| s.read_bonus::<B>())
    }

    fn read_vec_building<B: ByteOrder>(&mut self) -> io::Result<Vec<Building>> {
        self.read_vec::<B, _, _>(|s| s.read_building::<B>())
    }

    fn read_vec_tree<B: ByteOrder>(&mut self) -> io::Result<ReadVecTreeResult> {
        let len = try!(self.read_i32::<B>());
        if len < 0 {
            return Ok(ReadVecTreeResult::UseCached);
        }
        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            result.push(try!(self.read_tree::<B>()));
        }
        Ok(ReadVecTreeResult::Values(result))
    }

    fn read_vec_model_message<B: ByteOrder>(&mut self) -> io::Result<Vec<ModelMessage>> {
        self.read_vec::<B, _, _>(|s| s.read_model_message::<B>())
    }

    fn read_vec_skill_type<B: ByteOrder>(&mut self) -> io::Result<Vec<SkillType>> {
        self.read_vec::<B, _, _>(|s| s.read_skill_type())
    }

    fn read_vec_status<B: ByteOrder>(&mut self) -> io::Result<Vec<Status>> {
        self.read_vec::<B, _, _>(|s| s.read_status::<B>())
    }

    fn read_vec_i32<B: ByteOrder>(&mut self) -> io::Result<Vec<i32>> {
        self.read_vec::<B, _, _>(|s| s.read_i32::<B>())
    }

    fn read_vec_i8<B: ByteOrder>(&mut self) -> io::Result<Vec<i8>> {
        self.read_vec::<B, _, _>(|s| s.read_i8())
    }

    fn read_vec_u8<B: ByteOrder>(&mut self) -> io::Result<Vec<u8>> {
        use std::io::{Error, ErrorKind};
        let len = try!(self.read_i32::<B>());
        if len < 0 {
            return Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_vec_u8 error: len < 0, where len={}",  len)));
        }
        let mut result = vec![0; len as usize];
        try!(self.read_exact(&mut result));
        Ok(result)
    }

    fn read_vec<B: ByteOrder, T, F>(&mut self, read: F) -> io::Result<Vec<T>>
            where F: Fn(&mut Self) -> io::Result<T> {
        use std::io::{Error, ErrorKind};
        let len = try!(self.read_i32::<B>());
        if len < 0 {
            return Err(Error::new(ErrorKind::Other, format!("ReadMessage::read_vec error: len < 0, where len={}", len)));
        }
        let mut result = Vec::with_capacity(len as usize);
        for _ in 0..len {
            result.push(try!(read(self)));
        }
        Ok(result)
    }

    fn read_bool(&mut self) -> io::Result<bool> {
        Ok(try!(self.read_u8()) != 0)
    }
}

impl<R: ReadBytesExt> ReadMessage for R {}

#[test]
fn test_read_bool() {
    use std::io::Cursor;
    assert_eq!(Cursor::new(vec![0u8]).read_bool().unwrap(), false);
    assert_eq!(Cursor::new(vec![1u8]).read_bool().unwrap(), true);
    assert_eq!(Cursor::new(vec![255u8]).read_bool().unwrap(), true);
}

#[test]
fn test_read_vec_u8() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, 13u8])
        .read_vec_u8::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42u8, 13u8]);
}

#[test]
fn test_read_vec_i8() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, -42i8 as u8])
        .read_vec_i8::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42i8, -42i8]);
}

#[test]
fn test_read_vec_i32() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 42u8, 0u8, 0u8, 0u8, 13u8, 0u8, 0u8, 0u8])
        .read_vec_i32::<LittleEndian>()
        .unwrap();
    assert_eq!(result, vec![42i32, 13i32]);
}

#[test]
fn test_read_string() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    let result = Cursor::new(vec![2u8, 0u8, 0u8, 0u8, 'a' as u8, 'b' as u8])
        .read_string::<LittleEndian>()
        .unwrap();
    assert_eq!(result, "ab".to_string());
}

#[test]
fn test_read_status_type() {
    use std::io::Cursor;
    assert_eq!(Cursor::new(vec![-1i8 as u8]).read_status_type().unwrap(), StatusType::Unknown);
    assert_eq!(Cursor::new(vec![0u8]).read_status_type().unwrap(), StatusType::Burning);
    assert_eq!(Cursor::new(vec![1u8]).read_status_type().unwrap(), StatusType::Empowered);
    assert_eq!(Cursor::new(vec![2u8]).read_status_type().unwrap(), StatusType::Frozen);
    assert_eq!(Cursor::new(vec![3u8]).read_status_type().unwrap(), StatusType::Hastened);
    assert_eq!(Cursor::new(vec![4u8]).read_status_type().unwrap(), StatusType::Shielded);
    assert_eq!(Cursor::new(vec![5u8]).read_status_type().unwrap(), StatusType::Count);
    assert_eq!(Cursor::new(vec![6u8]).read_status_type().is_ok(), false);
}

#[test]
fn test_read_player() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    use code_wizards::model::Faction;
    let result = Cursor::new(vec![
        1u8,
        42u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8, 0u8,
        1u8,
        1u8, 0u8, 0u8, 0u8, 'x' as u8,
        0u8,
        13u8, 0u8, 0u8, 0u8,
        1u8,
    ]).read_player::<LittleEndian>().unwrap();
    assert_eq!(result,
        Player {
            id: 42,
            me: true,
            name: "x".to_string(),
            strategy_crashed: false,
            score: 13,
            faction: Faction::Renegades,
        });
}

#[test]
fn test_read_message_game_over() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    assert_eq!(Cursor::new(vec![1u8]).read_message::<LittleEndian>().unwrap(), Message::GameOver);
}

#[test]
fn test_read_message_team_size() {
    use std::io::Cursor;
    use byteorder::LittleEndian;
    assert_eq!(Cursor::new(vec![3u8, 42u8, 0u8, 0u8, 0u8]).read_message::<LittleEndian>().unwrap(), Message::TeamSize(42));
}
