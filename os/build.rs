use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    // 获取环境变量 ROOT_TASK_BIN
    let root_task_bin = env::var("ROOT_TASK_BIN")
        .expect("ROOT_TASK_BIN environment variable not set");
    let data = fs::read(&root_task_bin).unwrap();
    // 生成 Rust 代码
    let dest_path = PathBuf::from(env::var("GEN_PATH").unwrap());
    fs::write(
        &dest_path,
        format!(
            "#[used]\n\
             #[unsafe(link_section = \".root_task_data\")]\n\
             static ROOT_TASK_DATA: [u8; {}] = *include_bytes!(\"{}\");",
            data.len(), root_task_bin
        ),
    ).expect("Failed to write root_task_data.rs");

    // 告诉 Cargo 如果 ROOT_TASK_BIN 发生变化，重新运行 build.rs
    println!("cargo:rerun-if-env-changed=ROOT_TASK_BIN");
    println!("cargo:rerun-if-changed={}", root_task_bin);
}