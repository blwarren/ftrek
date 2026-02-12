use clap::Parser;

#[derive(Debug, Parser)]
#[command(
    name = "ftrek",
    about = "A fast Rust utility that visually displays a directory tree."
)]
pub struct TrekOptions {
    #[arg(value_name = "DIRECTORY", default_value = ".")]
    pub root: String,
    #[arg(
        long,
        help = "Apply .gitignore filtering when walking the directory structure."
    )]
    pub gitignore: bool,
}

impl TrekOptions {
    #[cfg(test)]
    pub fn try_from_args(args: &[String]) -> Result<Self, clap::Error> {
        let mut argv = Vec::with_capacity(args.len() + 1);
        argv.push("ftrek".to_string());
        argv.extend(args.iter().cloned());
        Self::try_parse_from(argv)
    }
}

#[cfg(test)]
mod tests {
    use super::TrekOptions;

    #[test]
    fn defaults_when_no_args() {
        let args: Vec<String> = vec![];
        let options = TrekOptions::try_from_args(&args).expect("parse args");
        assert_eq!(options.root, ".");
        assert!(!options.gitignore);
    }

    #[test]
    fn parses_gitignore_and_root() {
        let args = vec!["--gitignore".to_string(), "src".to_string()];
        let options = TrekOptions::try_from_args(&args).expect("parse args");
        assert_eq!(options.root, "src");
        assert!(options.gitignore);
    }

    #[test]
    fn rejects_multiple_roots() {
        let args = vec![
            "first".to_string(),
            "--gitignore".to_string(),
            "second".to_string(),
        ];
        let err = TrekOptions::try_from_args(&args).expect_err("expected parse failure");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }

    #[test]
    fn rejects_unknown_flags() {
        let args = vec!["--unknown".to_string()];
        let err = TrekOptions::try_from_args(&args).expect_err("expected parse failure");
        assert_eq!(err.kind(), clap::error::ErrorKind::UnknownArgument);
    }
}
