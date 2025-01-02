use dice_system::*;
use dice::dice::roll;
use dice::results::AllRollResults;
use enemy::*;
use player::*;

pub trait Battle {
    fn attack_against(&mut self, player_attack_roll, 
        enemy_attack_roll) -> (&AllRollResults, &AllRollResults);
}

pub struct BattleSystem {
    roll_result: (AllRollResults, AllRollResults)
    player_entity: Player,
}

pub impl Battle for BattleSystem {
    fn attack_against(&mut self, player_attack_roll, enemy_attack_roll) {
        if 
    }
}

