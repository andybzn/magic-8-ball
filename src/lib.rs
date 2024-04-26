use clap::Parser;

#[derive(Parser, Debug)]
#[command(
    author,
    version,
    about = "Ask magic8 a question, get an answer",
    long_about = None,
    help_template = "\n\n{before-help}{name} (v{version})\nAuthors: {author-with-newline}{about-with-newline}\n{all-args}{after-help}"
)]
pub struct Config {
    /// The question you wish to ask
    pub question: String,

    /// Output the answer's raw text only
    #[arg(
        short = 'a',
        long = "answer-only",
        default_value = "false",
        conflicts_with = "output_json"
    )]
    pub output_only: bool,

    /// Output the question and answer as a JSON object
    #[arg(
        short = 'j',
        long = "json",
        default_value = "false",
        conflicts_with = "output_only"
    )]
    pub output_json: bool,
}

impl Config {
    /// # Errors
    /// Will error if args fail to parse
    pub fn build() -> Result<Self, &'static str> {
        let args = Self::parse();

        Ok(Self {
            question: args.question,
            output_only: args.output_only,
            output_json: args.output_json,
        })
    }
}
