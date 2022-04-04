//! The command trait and commands structs.

pub trait Command {
    fn keywords(&self) -> &[&str];
    fn arguments(&self) -> &[&str];
    fn execute(&self, arg: &str) -> Result<(), String>;
    fn help(&self) -> &str;
}

struct QuitCommand;
impl Command for QuitCommand {
    fn keywords(&self) -> &[&str] {
        &["quit", "exit", "q"]
    }

    fn arguments(&self) -> &[&str] {
        &[]
    }

    fn execute(&self, _arg: &str) -> Result<(), String> {
        println!("Good luck with your tasks ;)");
        std::process::exit(0);
    }

    fn help(&self) -> &str {
        "Exits the program."
    }
}

pub struct Commands {
    pub commands: Vec<Box<dyn Command>>,
}

impl Commands {
    pub fn new() -> Self {
        let commands: Vec<Box<dyn Command>> = vec![Box::new(QuitCommand)];

        Commands { commands }
    }
}

impl Iterator for Commands {
    type Item = Box<dyn Command>;

    fn next(&mut self) -> Option<Self::Item> {
        self.commands.pop()
    }
}