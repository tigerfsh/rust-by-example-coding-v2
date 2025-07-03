use anyhow::Result;
use anyhow::anyhow;
use clap::{ArgMatches, Command};
use project_root;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use toml_edit::DocumentMut;

mod build_example;

fn main() -> Result<()> {
    let cmd = cli();
    let mut help_cmd = cmd.clone();

    let matches = cmd.get_matches();

    // 处理子命令
    match matches.subcommand() {
        Some(("build-example", sub_matches)) => build_example::handle_example(sub_matches)?,
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
