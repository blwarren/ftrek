# 🗺️ ftrek

A fast and idiomatic Rust utility that visually displays the directory structure of a path.

## 🚀 Features

- Clean, Unicode-based structure output
- Optional support for `.gitignore` rules with `--gitignore`
- Recursive traversal of directories
- Simple and extensible design in pure Rust

## 📦 Installation

```bash
git clone https://github.com/blwarren/ftrek.git
cd ftrek
cargo build --release
```

The executable will be located at `target/release/ftrek`.

## 🔧 Usage

```bash
ftrek [OPTIONS] [DIRECTORY]
```

### Options

- `--gitignore` — Apply `.gitignore` filtering when walking the directory structure.

If no directory is provided, it defaults to the current working directory.

### Example

```bash
ftrek --gitignore src
```

## 📄 License

This project is licensed under the MIT License.
