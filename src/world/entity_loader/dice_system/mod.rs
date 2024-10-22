use dice::results::AllRollResults;
use dice::dice::roll;

pub trait DiceRollEvent{
    fn roll_for_event(&mut self) -> AllRollResults;
}

pub struct DiceSystem{
    pub roll_result: AllRollResults
}

impl DiceRollEvent for DiceSystem{
    fn roll_for_event(&mut self) -> AllRollResults {
        let results = roll("1d20");
    
        println!("{:?}", results);
    
        return results;
    }
}
