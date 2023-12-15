use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

pub(crate) fn publish(directory: &str, branch: &str) {
    // 创建临时目录
    let temp_dir = TempDir::new_in(".").expect("Failed to create temp directory");
    let temp_dir_path = temp_dir.path();
    let source_path: &Path = Path::new(directory);
    // 确保目录存在
    if !source_path.exists() {
        panic!("Directory :{} does not exist", directory);
    }

    // 复制文件到临时目录
    // 这里我们假设 `directory` 是一个文件夹路径
    let mut dest_path = temp_dir_path.join("gh_page_content");
    // 目录不存在，创建目录
    let _ = fs::create_dir_all(&dest_path)
        .map(|_| println!("created temp dir {} Success", dest_path.to_str().unwrap()));
    if !dest_path.exists() {
        panic!("Directory :{} does not exist", dest_path.to_str().unwrap());
    }

    // 复制 .git 配置到临时目录
    let git_config_path = Path::new(".git/config");

    copy_dir_all(&source_path, &dest_path);
    
    // 切换到book目录
    dest_path = dest_path.join(directory);
    // 在临时目录中进行 Git 操作
    // 初始化 Git 仓库
    let status = Command::new("git")
        .args(&["init"])
        .current_dir(&dest_path)
        .status()
        .expect("Failed to initialize git");
    if !status.success() {
        panic!("Failed to initialize git repository");
    }
    println!("git init success!");

    // 复制 .git 配置到临时目录
    if git_config_path.exists() {
        let target_config_path = dest_path.join(".git/config");
        println!(
            "git copy config start  source:{} , target:{}",
            git_config_path.to_str().unwrap(),
            target_config_path.to_str().unwrap()
        );

        fs::copy(git_config_path, target_config_path).expect("Failed to copy .git/config");
        println!("git copy config success!");
    } else {
        panic!("No .git/config found in the root directory");
    }
    
    // 切换到临时目录
    std::env::set_current_dir(&dest_path).expect("Failed to change to temp directory");

    let status = Command::new("git")
        .args(&["checkout", "-b", branch])
        .status()
        .expect("Failed to checkout branch");
    if !status.success() {
        panic!("Failed to  checkout branch: {}", branch);
    }
    println!("git checkout {} success!", branch);

    // 添加文件
    let status = Command::new("git")
        .args(&["add", "."])
        .status()
        .expect("Failed to add files");

    if !status.success() {
        panic!("Failed to add files");
    }
    println!("git add success!");

    // 提交更改
    let status = Command::new("git")
        .args(&["commit", "-m", "Publish changes"])
        .status()
        .expect("Failed to commit changes");

    if !status.success() {
        panic!("Failed to commit changes");
    }
    println!("git commit success!");

    // 推送到远程分支
    let status = Command::new("git")
        .args(&["push", "-f", "origin", branch])
        .status()
        .expect("Failed to push changes to {}");

    if !status.success() {
        panic!("Failed to push changes to branch :{}", branch);
    }
    let _ = temp_dir.close().expect("drop temp dir failed!");
    // 退出临时目录
    // 临时目录将在 `temp_dir` 变量离开作用域时自动删除
}

// 递归复制目录
fn copy_dir_all(src: &Path, dst: &Path) {
    println!("source dir is {}", src.to_str().unwrap());
    println!("dst dir is {}", dst.to_str().unwrap());

    let options = fs_extra::dir::CopyOptions::new(); // 默认选项
    fs_extra::dir::copy(src, dst, &options).expect("Failed to copy directory");
    println!("copy_dir_all to temp success!");
}
