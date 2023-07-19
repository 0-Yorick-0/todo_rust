// Traits are like interfaces in other languages
// but they can provide default implementations
pub trait Command {
    fn handle(&self) -> i32;
}

pub struct AddCommand {
    args: Vec<String>,
}

impl AddCommand {
    // no arguments mean that the method can be called statically
    pub fn new(args: Vec<String>) -> Self {
        AddCommand { args }
    }
}

// When defining interface implementations, all method are public by default
impl Command for AddCommand {
    fn handle(&self) -> i32 {
        let description_option = &self.args.get(2);
        if let Some(descirption) = description_option {
            println!("Adding todo: {}", descirption);
            0
        } else {
            println!("Please provide a description");
            1
        }
    }
}

pub struct ListCommand {}

impl ListCommand {
    // no arguments mean that the method can be called statically
    pub fn new() -> Self {
        ListCommand {}
    }
}

impl Command for ListCommand {
    fn handle(&self) -> i32 {
        println!("Listing todo");
        0
    }
}
