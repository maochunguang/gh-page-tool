为了实现一个命令行工具，我们将按照以下步骤来构建它：

1. **设置 Rust 项目**
2. **添加 `clap` 依赖并配置**
3. **解析命令行参数**
4. **实现将目录内容推送到远程分支的功能**

### 步骤 1：设置 Rust 项目

首先，创建一个新的 Rust 项目：

```bash
cargo new gh_page_tool
cd gh_page_tool
```

### 步骤 2：添加 `clap` 依赖并配置

在您的 `Cargo.toml` 文件中添加 `clap` 依赖。由于您想使用 `clap` 的 `4.4.0` 版本，请确保您的依赖项如下所示：

```toml
[dependencies]
clap = "4.4.0"
```

然后运行 `cargo build` 以安装依赖项。

### 步骤 3：解析命令行参数

在 `src/main.rs` 中，使用 `clap` 来构建命令行接口并解析参数：

```rust
use clap::{App, Arg, Command};

fn main() {
    let matches = Command::new("gh_page")
        .version("1.0")
        .author("Your Name")
        .about("Implements gh-pages like functionality")
        .subcommand(
            App::new("publish")
                .about("Publishes a directory to a gh-pages branch")
                .arg(Arg::new("directory")
                    .short('d')
                    .long("directory")
                    .takes_value(true)
                    .help("Sets the directory to publish"))
                .arg(Arg::new("branch")
                    .short('b')
                    .long("branch")
                    .takes_value(true)
                    .help("Sets the branch to publish to"))
        )
        .get_matches();

    if let Some(matches) = matches.subcommand_matches("publish") {
        let directory = matches.value_of("directory").unwrap_or("book");
        let branch = matches.value_of("branch").unwrap_or("gh-pages");
        publish(directory, branch);
    }
}

fn publish(directory: &str, branch: &str) {
    println!("Publishing directory '{}' to branch '{}'", directory, branch);
    // Add logic here for publishing the directory to the branch
}
```

### 步骤 4：实现目录推送功能

实现 `publish` 函数以完成推送工作。这通常涉及到执行一系列的 Git 命令来将文件推送到指定的分支。您可能需要使用 `std::process::Command` 来执行这些外部命令。

这里是一个基本的框架，展示了如何开始：

```rust
use std::process::Command;
use std::path::Path;

fn publish(directory: &str, branch: &str) {
    if !Path::new(directory).exists() {
        eprintln!("Directory '{}' does not exist.", directory);
        return;
    }

    // 这里可以添加 Git 操作的逻辑
    // 例如：git add, git commit, git push 等
    // 请注意，这可能需要额外的错误处理和复杂的逻辑

    // 示例：执行 git add
    let status = Command::new("git")
        .args(["add", "."])
        .current_dir(directory)
        .status()
        .expect("Failed to execute git add");

    if !status.success() {
        eprintln!("Failed to add files to git.");
        return;
    }

    // 以此类推，添加 git commit 和 git push 的操作
}
```

### 注意事项

- 实现 Git 操作时，您可能需要进行错误处理以及处理各种可能的边缘情况。
- 根据您的需求，您可能还需要实现其他功能，比如临时切换分支、处理不同的 Git 配置等。

这个实现提供了一个基本的框架，您可以根据具体需求进一步扩展和完善。