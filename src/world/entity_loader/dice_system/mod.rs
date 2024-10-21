use dice::dice;
use ::dice::results::AllRollResults;

pub trait DiceRollEvent{
    fn roll_for_event(&mut self) -> AllRollResults;
}

struct DiceRoll{
    rollResult: AllRollResults
}

impl DiceRollEvent for DiceRoll{
    fn roll_for_event(&mut self) -> AllRollResults {
        let results = dice::roll("1d20");
    
        println!("{:?}", results);
    
        return results;
    }
}
