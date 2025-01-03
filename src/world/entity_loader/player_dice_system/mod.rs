use super::dice_system::*;
use super::player::PlayerStr;

trait PlayerInterface {
    fn player_atk(&mut self) -> isize;
    fn player_tech(&mut self) -> isize;
    fn player_agi(&mut self) -> isize;
    fn player_charm(&mut self) -> isize;
}

struct PlayerDiceSystem {
    player_atk_dice: DiceAttackSystem,
    player_tech_dice: DiceTechSystem,
    player_agi_dice: DiceAgilitySystem,
    player_charm_dice: DiceCharmSystem,
    player_str: PlayerStr,
}

impl PlayerInterface for PlayerDiceSystem {
    fn player_atk(&mut self) -> isize {
        let atk_roll = self.player_atk_dice.get_attack_roll().total + *self.player_str.get_str() as isize;
        return atk_roll;
    }

    fn player_tech(&mut self) -> isize {

    }

    fn player_agi(&mut self) -> isize {

    }

    fn player_charm(&mut self) -> isize {

    }
}
