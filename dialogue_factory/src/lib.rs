pub trait AbstractNameTextDialogue {
    fn name_dialogue(&self, name: String) -> String;
}

pub trait AbstractTextBoxDialogue {
    fn textbox_dialogue(&self, textbox: String) -> String;
}

pub trait AbstractTypeDialogue {
    fn type_dialogue(&self, type_dialogue: String) -> String;
}

// Dynamic abstract factory using Box pointer
pub trait AbstractDialogueFactory {
    fn create_name_text_dialogue(&self) -> Box<dyn AbstractNameTextDialogue>;
    fn create_text_box_dialogue(&self) -> Box<dyn AbstractTextBoxDialogue>;
    fn create_type_dialogue(&self) -> Box<dyn AbstractTypeDialogue>;
}

pub struct DialogueFactory {}

impl AbstractDialogueFactory for DialogueFactory {
    fn create_name_text_dialogue(&self) -> Box<dyn AbstractNameTextDialogue> {
        return Box::new(NameText {});
    }

    fn create_text_box_dialogue(&self) -> Box<dyn AbstractTextBoxDialogue> {
        return Box::new(TextBox {});
    }

    fn create_type_dialogue(&self) -> Box<dyn AbstractTypeDialogue>{
        return Box::new(TypeDialogue {});
    }

}

struct NameText {}

impl AbstractNameTextDialogue for NameText{
    fn name_dialogue(&self, name: String) -> String {
        return String::from(name);
    }
}

struct TextBox {}

impl AbstractTextBoxDialogue for TextBox{
    fn textbox_dialogue(&self, textbox: String) -> String {
        return String::from(textbox);
    }
}

struct TypeDialogue {}

impl AbstractTypeDialogue for TypeDialogue{
    fn type_dialogue(&self, type_dialogue: String) -> String {
        return String::from(type_dialogue);
    }
}
