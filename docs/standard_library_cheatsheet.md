# Rust Standard Library Cheat Sheet

This document summarizes the most useful `std` modules and functions for building CLI and system-level projects in Rust.

---

## ✨ Most Common `std` Modules

---

## 📂 `std::fs` — Filesystem
| Function                        | Purpose                                  |
|-------------------------------|------------------------------------------|
| `fs::read_to_string("file")`    | Read file into a `String`                |
| `fs::write("file", "text")`     | Write text to a file                     |
| `fs::copy("src", "dst")`        | Copy a file                              |
| `fs::remove_file("file")`       | Delete a file                            |
| `fs::create_dir_all("dir")`     | Create nested directories                |
| `fs::metadata("file")`          | Check if a file exists / get info        |

---

## 📁 `std::path` — Path Handling
| Type / Function        | Purpose                             |
|------------------------|-------------------------------------|
| `Path::new("file")`     | Create a `Path` object              |
| `PathBuf`              | Growable, owned path object         |
| `path.exists()`        | Check if path exists                |
| `path.extension()`     | Get file extension (e.g., `.txt`)   |
| `path.display()`       | Print path for display              |

---

## 💪 `std::env` — Environment Access
| Function                  | Purpose                                    |
|--------------------------|--------------------------------------------|
| `env::current_dir()`       | Get current working directory              |
| `env::set_current_dir()`   | Change working directory                   |
| `env::var("KEY")`         | Read an environment variable               |
| `env::args()`              | Read command-line arguments                |
| `env::consts::OS`          | Get OS platform string (e.g. `"macos"`)    |

---

## 👨‍💻 `std::io` — Input/Output
| Function / Trait           | Purpose                                  |
|---------------------------|------------------------------------------|
| `stdin().read_line()`     | Read user input from terminal            |
| `stdout().flush()`        | Flush output buffer                      |
| `BufReader`               | Efficient file reading line-by-line      |
| `Write`, `Read`, `Seek`   | Core I/O traits                          |

---

## ⏱ `std::time` — Duration + Timing
| Function                      | Purpose                            |
|------------------------------|------------------------------------|
| `Instant::now()`             | Get high-precision clock time      |
| `Duration::from_secs(5)`     | Represent a duration of 5 seconds  |

For real timestamps, use `chrono` crate instead of `std::time`.

---

## ⚙️ `std::process` — Run External Commands
| Function                          | Purpose                              |
|----------------------------------|--------------------------------------|
| `Command::new("ls").output()`     | Run a shell command                  |
| `status.success()`               | Check if process exited successfully |
| `stdout`, `stderr`               | Get output from command              |

---

## 🔢 `std::collections`
| Type         | Purpose                       |
|--------------|-------------------------------|
| `Vec<T>`      | Growable, ordered list         |
| `HashMap<K,V>`| Key-value store               |
| `HashSet<T>`  | Set (no duplicates)           |
| `BTreeMap<K,V>`| Sorted key-value store       |

---

## ✨ Bonus Combo: Most Common Imports for CLI
```rust
use std::fs;
use std::env;
use std::path::Path;
use std::io::{self, Write};
use std::collections::HashMap;
```

---

## For More:
- [Rust Standard Library Docs](https://doc.rust-lang.org/std/)
- [Crates.io for external libraries](https://crates.io)

---

_This sheet was generated as part of the Bakery Manager CLI project by Sarah._

