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
