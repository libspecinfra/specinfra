pub struct CommandResult {
    pub stdout: String,
    pub stderr: String,
    pub success: bool,
    pub code: i32,
}

pub struct Command {
    pub string: String,
}

impl Command {
    pub fn new(s: &str) -> Command {
        Command { string: s.into() }
    }

    pub fn and(&mut self, s: &str) -> &Command {
        let c = format!(" && {}", s);
        self.string += &c;
        self
    }

    pub fn or(&mut self, s: &str) -> &Command {
        let c = format!(" || {}", s);
        self.string += &c;
        self
    }

    pub fn pipe(&mut self, s: &str) -> &Command {
        let c = format!(" | {}", s);
        self.string += &c;
        self
    }
}

impl<'a> From<&'a str> for Command {
    fn from(s: &str) -> Command {
        Command::new(s.into())
    }
}
