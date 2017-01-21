use super::model::{Game, Move, Wizard, World};

#[derive(Clone, Debug)]
pub struct MyStrategy {}

impl MyStrategy {
    pub fn new() -> Self {
        MyStrategy {}
    }

    pub fn move_(&mut self, me: &Wizard, world: &World, game: &Game, move_: &mut Move) {
        use super::model::ActionType;
        move_.set_speed(game.wizard_forward_speed);
        move_.set_strafe_speed(game.wizard_strafe_speed);
        move_.set_turn(game.wizard_max_turn_angle);
        move_.set_action(ActionType::MagicMissile);
    }
}
