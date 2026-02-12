use super::TrekOptions;
use ignore::{DirEntry, WalkBuilder};
use owo_colors::OwoColorize;
use std::fs;
use std::io::{self, IsTerminal};
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};

pub fn run(options: &TrekOptions) -> io::Result<()> {
    let color = io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none();

    if options.gitignore {
        let walker = WalkBuilder::new(&options.root)
            .standard_filters(true)
            .require_git(false)
            .build();

        visit_gitignore(walker, &options.root, color)?;
    } else {
        visit(&PathBuf::from(&options.root), 0, &mut Vec::new(), color)?;
    }
    Ok(())
}

fn visit(path: &Path, depth: usize, prefix_stack: &mut Vec<bool>, color: bool) -> io::Result<()> {
    let kind = entry_kind_for_path(path)?;
    let name = path.file_name().unwrap_or_default().to_string_lossy();

    if depth > 0 {
        for &last in &prefix_stack[..prefix_stack.len() - 1] {
            let segment = if last { "    " } else { "│   " };
            print!("{}", style_branch(segment));
        }
        let is_last = *prefix_stack.last().unwrap_or(&true);
        let connector = if is_last { "└── " } else { "├── " };
        print!("{}", style_branch(connector));
    }

    if matches!(kind, EntryKind::Directory) {
        if depth == 0 {
            println!(
                "{}",
                style_entry(&format!("{}/", path.display()), kind, color)
            );
        } else {
            println!("{}", style_entry(&format!("{name}/"), kind, color));
        }

        let entries: Vec<_> = fs::read_dir(path)?.filter_map(Result::ok).collect();

        let len = entries.len();
        for (i, entry) in entries.into_iter().enumerate() {
            let is_last = i == len - 1;
            prefix_stack.push(is_last);
            visit(&entry.path(), depth + 1, prefix_stack, color)?;
            prefix_stack.pop();
        }
    } else {
        println!("{}", style_entry(name.as_ref(), kind, color));
    }

    Ok(())
}

fn visit_gitignore<I>(walker: I, root: &str, color: bool) -> io::Result<()>
where
    I: IntoIterator<Item = Result<DirEntry, ignore::Error>>,
{
    let mut prefix_stack: Vec<bool> = Vec::new();
    let mut last_components: Vec<String> = vec![];

    for entry in walker.into_iter().flatten() {
        let path = entry.path();
        let rel_path = path.strip_prefix(root).unwrap_or(path);
        let depth = rel_path.components().count();

        if depth == 0 {
            println!(
                "{}",
                style_entry(&format!("{root}/"), EntryKind::Directory, color)
            );
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
                        let segment = if last { "    " } else { "│   " };
                        print!("{}", style_branch(segment));
                    }
                    let is_last = i == depth - 1;
                    let connector = if is_last { "└── " } else { "├── " };
                    print!("{}", style_branch(connector));
                }

                let is_leaf = i == depth - 1;
                let is_directory =
                    !is_leaf || entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);
                if is_directory {
                    println!(
                        "{}",
                        style_entry(&format!("{part}/"), EntryKind::Directory, color)
                    );
                } else {
                    let kind = entry_kind_for_path(path).unwrap_or(EntryKind::Regular);
                    println!("{}", style_entry(&part, kind, color));
                }

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

    Ok(())
}

fn style_branch(segment: &str) -> String {
    segment.to_string()
}

#[derive(Copy, Clone)]
enum EntryKind {
    Directory,
    Symlink,
    Executable,
    Regular,
}

fn style_entry(name: &str, kind: EntryKind, color: bool) -> String {
    if !color {
        return name.to_string();
    }

    match kind {
        EntryKind::Directory => format!("{}", name.blue()),
        EntryKind::Symlink => format!("{}", name.cyan()),
        EntryKind::Executable => format!("{}", name.green()),
        EntryKind::Regular => name.to_string(),
    }
}

fn entry_kind_for_path(path: &Path) -> io::Result<EntryKind> {
    let metadata = fs::symlink_metadata(path)?;
    let file_type = metadata.file_type();

    if file_type.is_symlink() {
        return Ok(EntryKind::Symlink);
    }
    if file_type.is_dir() {
        return Ok(EntryKind::Directory);
    }

    #[cfg(unix)]
    {
        if file_type.is_file() && (metadata.permissions().mode() & 0o111 != 0) {
            return Ok(EntryKind::Executable);
        }
    }

    #[cfg(not(unix))]
    {
        let _ = metadata;
    }

    Ok(EntryKind::Regular)
}

#[cfg(test)]
mod tests {
    use super::{EntryKind, style_entry};

    #[test]
    fn no_color_keeps_plain_text() {
        assert_eq!(
            style_entry("file.txt", EntryKind::Regular, false),
            "file.txt"
        );
    }

    #[test]
    fn regular_files_are_not_colored() {
        assert_eq!(
            style_entry("file.txt", EntryKind::Regular, true),
            "file.txt"
        );
    }

    #[test]
    fn directories_are_colored_when_enabled() {
        let out = style_entry("dir/", EntryKind::Directory, true);
        assert!(out.contains("\u{1b}["));
        assert!(out.ends_with("dir/\u{1b}[39m"));
    }
}
