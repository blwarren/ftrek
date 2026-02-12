#[derive(Debug)]
pub struct TrekOptions {
    pub root: String,
    pub gitignore: bool,
}

impl TrekOptions {
    pub fn from_args(args: &[String]) -> Self {
        let mut root = ".".to_string();
        let mut gitignore = false;

        for arg in args {
            if arg == "--gitignore" {
                gitignore = true;
            } else if !arg.starts_with('-') {
                root = arg.clone();
            }
        }

        TrekOptions { root, gitignore }
    }
}

#[cfg(test)]
mod tests {
    use super::TrekOptions;

    #[test]
    fn defaults_when_no_args() {
        let args: Vec<String> = vec![];
        let options = TrekOptions::from_args(&args);
        assert_eq!(options.root, ".");
        assert!(!options.gitignore);
    }

    #[test]
    fn parses_gitignore_and_root() {
        let args = vec!["--gitignore".to_string(), "src".to_string()];
        let options = TrekOptions::from_args(&args);
        assert_eq!(options.root, "src");
        assert!(options.gitignore);
    }

    #[test]
    fn uses_last_non_flag_as_root() {
        let args = vec![
            "first".to_string(),
            "--gitignore".to_string(),
            "second".to_string(),
        ];
        let options = TrekOptions::from_args(&args);
        assert_eq!(options.root, "second");
        assert!(options.gitignore);
    }

    #[test]
    fn ignores_unknown_flags_for_root_selection() {
        let args = vec!["--unknown".to_string()];
        let options = TrekOptions::from_args(&args);
        assert_eq!(options.root, ".");
        assert!(!options.gitignore);
    }
}
