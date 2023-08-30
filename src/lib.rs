pub struct Config {
    pub question: String,
}

impl Config {
    /// # Errors
    /// Will error if no parameters are provided
    pub fn build(
        mut args: impl Iterator<Item = String>,
    ) -> Result <Self, &'static str> {
        args.next();

        let question: String = match args.next() {
            Some(arg) => arg,
            None => return Err("You didn't ask a question"),
        };

        Ok(Self {
            question,
        })
    }
}
