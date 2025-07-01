use clap::{ArgMatches, Command};
use project_root;
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    let cmd = cli();
    let mut help_cmd = cmd.clone();

    let matches = cmd.get_matches();

    // 处理子命令
    match matches.subcommand() {
        Some(("build-example", sub_matches)) => handle_build_example(sub_matches),
        Some((cmd, sub_matches)) => println!("未匹配命令: {}, 参数: {:?}", cmd, sub_matches),
        None => help_cmd.print_help().unwrap(),
    }
}

fn cli() -> Command {
    Command::new("xtask")
        // 子命令
        .subcommand(
            Command::new("build-example")
                .alias("be")
                .about("build example in Cargo.toml"),
        )
}

// 子命令处理函数示例
fn handle_build_example(_matches: &ArgMatches) {
    println!("执行 build 命令");
    match project_root::get_project_root() {
        Ok(p) => println!("Current project root is {:?}", p),
        Err(e) => println!("Error obtaining project root {:?}", e),
    };

    let file_path_in_examples = get_example_files("/home/star/rust-by-example-coding-v2");
    for fp in file_path_in_examples {
        println!("{}", fp);
    }
}

fn get_example_files(project_root: &str) -> Vec<String> {
    WalkDir::new(Path::new(project_root).join("examples"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.path().to_str().map(|s| s.to_string()))
        .collect()
}
