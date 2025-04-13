use super::TrekOptions;
use ignore::{DirEntry, WalkBuilder};
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn run(options: &TrekOptions) -> io::Result<()> {
    if options.gitignore {
        let walker = WalkBuilder::new(&options.root)
            .standard_filters(true)
            .build();

        visit_gitignore(walker, &options.root)?;
    } else {
        visit(&PathBuf::from(&options.root), 0, &mut Vec::new())?;
    }
    Ok(())
}

fn visit(path: &Path, depth: usize, prefix_stack: &mut Vec<bool>) -> io::Result<()> {
    let name = path.file_name().unwrap_or_default().to_string_lossy();

    if depth > 0 {
        for &last in &prefix_stack[..prefix_stack.len() - 1] {
            print!("{}", if last { "    " } else { "│   " });
        }
        let is_last = *prefix_stack.last().unwrap_or(&true);
        print!("{}", if is_last { "└── " } else { "├── " });
    }

    if path.is_dir() {
        if depth == 0 {
            println!("{}/", path.display());
        } else {
            println!("{}/", name);
        }

        let entries: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();

        let len = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let is_last = i == len - 1;
            prefix_stack.push(is_last);
            visit(&entry.path(), depth + 1, prefix_stack)?;
            prefix_stack.pop();
        }
    } else {
        println!("{}", name);
    }

    Ok(())
}

fn visit_gitignore<I>(walker: I, root: &str) -> io::Result<()>
where
    I: IntoIterator<Item = Result<DirEntry, ignore::Error>>,
{
    let mut prefix_stack: Vec<bool> = Vec::new();
    let mut last_components: Vec<String> = vec![];

    for result in walker {
        if let Ok(entry) = result {
            let path = entry.path();
            let rel_path = path.strip_prefix(root).unwrap_or(path);
            let depth = rel_path.components().count();

            if depth == 0 {
                println!("{}/", root);
                continue;
            }

            while prefix_stack.len() >= depth {
                prefix_stack.pop();
                last_components.pop();
            }

            for (i, part) in rel_path
                .components()
                .map(|c| c.as_os_str().to_string_lossy().to_string())
                .enumerate()
            {
                if i >= last_components.len() || last_components[i] != part {
                    if i > 0 {
                        for &last in &prefix_stack[..i - 1] {
                            print!("{}", if last { "    " } else { "│   " });
                        }
                        let is_last = i == depth - 1;
                        print!("{}", if is_last { "└── " } else { "├── " });
                    }
                    println!("{}{}", part, if path.is_dir() { "/" } else { "" });

                    if i >= prefix_stack.len() {
                        prefix_stack.push(i == depth - 1);
                    }
                    if i >= last_components.len() {
                        last_components.push(part);
                    } else {
                        last_components[i] = part;
                    }
                    break;
                }
            }
        }
    }

    Ok(())
}
