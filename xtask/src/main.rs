use anyhow::Result;
use anyhow::anyhow;
use clap::{ArgMatches, Command};
use project_root;
use std::path::Path;
use walkdir::WalkDir;

fn main() -> Result<()> {
    let cmd = cli();
    let mut help_cmd = cmd.clone();

    let matches = cmd.get_matches();

    // 处理子命令
    match matches.subcommand() {
        Some(("build-example", sub_matches)) => handle_build_example(sub_matches)?,
        Some((cmd, sub_matches)) => println!("未匹配命令: {}, 参数: {:?}", cmd, sub_matches),
        None => help_cmd.print_help().unwrap(),
    }

    Ok(())
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
fn handle_build_example(_matches: &ArgMatches) -> Result<()> {
    let root_path = project_root::get_project_root()?;
    let root_path = root_path
        .to_str()
        .ok_or(anyhow!("项目路径包含无效 Unicode 字符"))?;

    for fp in get_example_files(root_path) {
        println!("{}", fp);
    }

    Ok(())
}

fn get_example_files(project_root: &str) -> Vec<String> {
    WalkDir::new(Path::new(project_root).join("examples"))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.path().to_str().map(|s| s.to_string()))
        .collect()
}
