use dice::results::AllRollResults;
use dice::dice::roll;

pub trait DiceRollEvent{
    fn roll_for_event(&mut self) -> &AllRollResults;
    fn roll_for_attack(&mut self) -> &AllRollResults;
    fn roll_for_tech(&mut self) -> &AllRollResults;
}

pub struct DiceSystem{
    pub dice_event: AllRollResults,
    pub dice_attack: AllRollResults,
    pub dice_tech: AllRollResults
}

impl DiceRollEvent for DiceSystem{
    fn roll_for_event(&mut self) -> &AllRollResults {
        self.dice_event = roll("1d20");
    
        println!("{:?}", &self.dice_event.total);
    
        return &self.dice_event;
    }

    fn roll_for_attack(&mut self) -> &AllRollResults {
        self.dice_attack = roll("1d20");
    
        println!("{:?}", &self.dice_attack.total);
    
        return &self.dice_attack;
    }

    fn roll_for_tech(&mut self) -> &AllRollResults {
        self.dice_tech = roll("1d20");
    
        println!("{:?}", &self.dice_tech.total);
    
        return &self.dice_tech;
    }
}
