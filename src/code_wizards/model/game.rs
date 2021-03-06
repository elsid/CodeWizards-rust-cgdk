#[derive(Clone, Debug, PartialEq)]
pub struct Game {
    pub random_seed: i64,
    pub tick_count: i32,
    pub map_size: f64,
    pub skills_enabled: bool,
    pub raw_messages_enabled: bool,
    pub friendly_fire_damage_factor: f64,
    pub building_damage_score_factor: f64,
    pub building_elimination_score_factor: f64,
    pub minion_damage_score_factor: f64,
    pub minion_elimination_score_factor: f64,
    pub wizard_damage_score_factor: f64,
    pub wizard_elimination_score_factor: f64,
    pub team_working_score_factor: f64,
    pub victory_score: i32,
    pub score_gain_range: f64,
    pub raw_message_max_length: i32,
    pub raw_message_transmission_speed: f64,
    pub wizard_radius: f64,
    pub wizard_cast_range: f64,
    pub wizard_vision_range: f64,
    pub wizard_forward_speed: f64,
    pub wizard_backward_speed: f64,
    pub wizard_strafe_speed: f64,
    pub wizard_base_life: i32,
    pub wizard_life_growth_per_level: i32,
    pub wizard_base_mana: i32,
    pub wizard_mana_growth_per_level: i32,
    pub wizard_base_life_regeneration: f64,
    pub wizard_life_regeneration_growth_per_level: f64,
    pub wizard_base_mana_regeneration: f64,
    pub wizard_mana_regeneration_growth_per_level: f64,
    pub wizard_max_turn_angle: f64,
    pub wizard_max_resurrection_delay_ticks: i32,
    pub wizard_min_resurrection_delay_ticks: i32,
    pub wizard_action_cooldown_ticks: i32,
    pub staff_cooldown_ticks: i32,
    pub magic_missile_cooldown_ticks: i32,
    pub frost_bolt_cooldown_ticks: i32,
    pub fireball_cooldown_ticks: i32,
    pub haste_cooldown_ticks: i32,
    pub shield_cooldown_ticks: i32,
    pub magic_missile_manacost: i32,
    pub frost_bolt_manacost: i32,
    pub fireball_manacost: i32,
    pub haste_manacost: i32,
    pub shield_manacost: i32,
    pub staff_damage: i32,
    pub staff_sector: f64,
    pub staff_range: f64,
    pub level_up_xp_values: Vec<i32>,
    pub minion_radius: f64,
    pub minion_vision_range: f64,
    pub minion_speed: f64,
    pub minion_max_turn_angle: f64,
    pub minion_life: i32,
    pub faction_minion_appearance_interval_ticks: i32,
    pub orc_woodcutter_action_cooldown_ticks: i32,
    pub orc_woodcutter_damage: i32,
    pub orc_woodcutter_attack_sector: f64,
    pub orc_woodcutter_attack_range: f64,
    pub fetish_blowdart_action_cooldown_ticks: i32,
    pub fetish_blowdart_attack_range: f64,
    pub fetish_blowdart_attack_sector: f64,
    pub bonus_radius: f64,
    pub bonus_appearance_interval_ticks: i32,
    pub bonus_score_amount: i32,
    pub dart_radius: f64,
    pub dart_speed: f64,
    pub dart_direct_damage: i32,
    pub magic_missile_radius: f64,
    pub magic_missile_speed: f64,
    pub magic_missile_direct_damage: i32,
    pub frost_bolt_radius: f64,
    pub frost_bolt_speed: f64,
    pub frost_bolt_direct_damage: i32,
    pub fireball_radius: f64,
    pub fireball_speed: f64,
    pub fireball_explosion_max_damage_range: f64,
    pub fireball_explosion_min_damage_range: f64,
    pub fireball_explosion_max_damage: i32,
    pub fireball_explosion_min_damage: i32,
    pub guardian_tower_radius: f64,
    pub guardian_tower_vision_range: f64,
    pub guardian_tower_life: f64,
    pub guardian_tower_attack_range: f64,
    pub guardian_tower_damage: i32,
    pub guardian_tower_cooldown_ticks: i32,
    pub faction_base_radius: f64,
    pub faction_base_vision_range: f64,
    pub faction_base_life: f64,
    pub faction_base_attack_range: f64,
    pub faction_base_damage: i32,
    pub faction_base_cooldown_ticks: i32,
    pub burning_duration_ticks: i32,
    pub burning_summary_damage: i32,
    pub empowered_duration_ticks: i32,
    pub empowered_damage_factor: f64,
    pub frozen_duration_ticks: i32,
    pub hastened_duration_ticks: i32,
    pub hastened_bonus_duration_factor: f64,
    pub hastened_movement_bonus_factor: f64,
    pub hastened_rotation_bonus_factor: f64,
    pub shielded_duration_ticks: i32,
    pub shielded_bonus_duration_factor: f64,
    pub shielded_direct_damage_absorption_factor: f64,
    pub aura_skill_range: f64,
    pub range_bonus_per_skill_level: f64,
    pub magical_damage_bonus_per_skill_level: i32,
    pub staff_damage_bonus_per_skill_level: i32,
    pub movement_bonus_factor_per_skill_level: f64,
    pub magical_damage_absorption_per_skill_level: i32,
}
