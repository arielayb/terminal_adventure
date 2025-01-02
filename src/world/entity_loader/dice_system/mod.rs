use dice::results::AllRollResults;
use dice::dice::roll;
use super::player::PlayerStr;
use super::player::PlayerAgility;
use super::player::PlayerCharm;
use super::player::PlayerLuck;

pub trait DiceRollEvent{
    fn roll_for_event(&mut self) -> &AllRollResults;
    fn get_event_roll(&mut self) -> &AllRollResults;
}

pub trait DiceRollAttack{
    fn roll_for_attack(&mut self) -> &AllRollResults;
    fn get_attack_roll(&mut self) -> &AllRollResults;
}

pub trait DiceRollTech{
    fn roll_for_tech(&mut self) -> &AllRollResults;
    fn get_tech_roll(&mut self) -> &AllRollResults;
}

pub trait DiceRollAgility{
    fn roll_for_agility(&mut self) -> &AllRollResults;
    fn get_agility_roll(&mut self) -> &AllRollResults;
}

pub trait DiceRollCharm{
    fn roll_for_charm(&mut self) -> &AllRollResults;
    fn get_charm_roll(&mut self) -> &AllRollResults;
}

pub trait DiceRollStrength{
    fn roll_for_strength(&mut self) -> &AllRollResults;
    fn get_strength_roll(&mut self) -> &AllRollResults;
}

pub trait DiceRollDark{
    fn roll_for_dark(&mut self) -> &AllRollResults;
    fn get_dark_roll(&mut self) -> &AllRollResults;
}

pub struct DiceEventSystem{
    pub dice_event: AllRollResults,
}

pub struct DiceAttackSystem{
    pub dice_attack: AllRollResults,
}

pub struct DiceDefenseSystem {
    pub equip_defense_val: u32,
}

pub struct DiceTechSystem{
    pub dice_tech: AllRollResults,
}

pub struct DiceAgilitySystem{
    pub dice_agility: AllRollResults,
}

pub struct DiceCharmSystem{
    pub dice_charm: AllRollResults,
}

pub struct DiceStrengthSystem{
    pub dice_strength: AllRollResults,
}

pub struct DiceDarkSystem{
    pub dice_dark: AllRollResults,
}

impl DiceRollEvent for DiceEventSystem{
    fn roll_for_event(&mut self) -> &AllRollResults {
        self.dice_event = roll("1d20");
    
        println!("Event roll, {:?}", &self.dice_event.total);
    
        return &self.dice_event;
    }

    fn get_event_roll(&mut self) -> &AllRollResults {
        return &self.dice_event;
    }
}

impl DiceRollAttack for DiceAttackSystem{
    fn roll_for_attack(&mut self) -> &AllRollResults {
        self.dice_attack = roll("1d20");
    
        println!("Attack roll, {:?}", &self.dice_attack.total);
    
        return &self.dice_attack;
    }

    fn get_attack_roll(&mut self) -> &AllRollResults {
        return &self.dice_attack;
    }
}

impl DiceRollTech for DiceTechSystem{
    fn roll_for_tech(&mut self) -> &AllRollResults {
        self.dice_tech = roll("1d20");
    
        println!("Tech roll, {:?}", &self.dice_tech.total);
    
        return &self.dice_tech;
    }

    fn get_tech_roll(&mut self) -> &AllRollResults {
        return &self.dice_tech;
    }
}

impl DiceRollAgility for DiceAgilitySystem{
    fn roll_for_agility(&mut self) -> &AllRollResults {
        self.dice_agility = roll("1d20");
    
        println!("Agility roll, {:?}", &self.dice_agility.total);
    
        return &self.dice_agility;
    }

    fn get_agility_roll(&mut self) -> &AllRollResults {
        return &self.dice_agility;
    }
}

impl DiceRollCharm for DiceCharmSystem{
    fn roll_for_charm(&mut self) -> &AllRollResults {
        self.dice_charm = roll("1d20");
    
        println!("Charm roll, {:?}", &self.dice_charm.total);
    
        return &self.dice_charm;
    }

    fn get_charm_roll(&mut self) -> &AllRollResults {
        return &self.dice_charm;
    }
}

impl DiceRollStrength for DiceStrengthSystem{
    fn roll_for_strength(&mut self) -> &AllRollResults {
        self.dice_strength = roll("1d20");
    
        println!("Strength roll, {:?}", &self.dice_strength.total);
    
        return &self.dice_strength;
    }

    fn get_strength_roll(&mut self) -> &AllRollResults {
        return &self.dice_strength;
    }
}

impl DiceRollDark for DiceDarkSystem{
    fn roll_for_dark(&mut self) -> &AllRollResults {
        self.dice_dark = roll("1d20");
    
        println!("Dark roll, {:?}", &self.dice_dark.total);
    
        return &self.dice_dark;
    }

    fn get_dark_roll(&mut self) -> &AllRollResults {
        return &self.dice_dark;
    }
}
