use super::dice_system::*;
use super::player::PlayerAgility;
use super::player::PlayerCharm;
use super::player::PlayerStr;
use super::player::PlayerTech;

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
    player_tech: PlayerTech,
    player_agi: PlayerAgility,
    player_charm: PlayerCharm,
}

impl PlayerInterface for PlayerDiceSystem {
    fn player_atk(&mut self) -> isize {
        let atk_roll = self.player_atk_dice.get_attack_roll().total + *self.player_str.get_str() as isize;
        return atk_roll;
    }

    fn player_tech(&mut self) -> isize {
        let tech_roll = self.player_tech_dice.get_tech_roll().total + *self.player_tech.get_tp() as isize;
        return tech_roll;
    }

    fn player_agi(&mut self) -> isize {
        let agi_roll = self.player_agi_dice.get_agility_roll().total + *self.player_agi.get_agi() as isize;
        return agi_roll
    }

    fn player_charm(&mut self) -> isize {
        let charm_roll = self.player_charm_dice.get_charm_roll().total + *self.player_charm.get_charm() as isize;
        return charm_roll;
    }
}
