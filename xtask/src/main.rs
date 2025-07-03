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
    let root_path_str = root_path
        .to_str()
        .ok_or_else(|| anyhow!("项目路径包含无效 Unicode 字符"))?;

    let mut examples = vec![];
    for fp in get_example_files(root_path_str) {
        // println!("{}", fp);
        let ef = Path::new(&fp);
        if let Some(pp) = ef.parent()
            && let Some(f_stem) = ef.file_stem()
            && let Some(f_stem_str) = f_stem.to_str()
            && let Some(pp_str) = pp.to_str()
        {
            let pp_array = pp_str.split("/").collect::<Vec<&str>>();
            let pp_slice = &pp_array[1..];

            let mut pp_vec = pp_slice.to_vec();
            pp_vec.push(f_stem_str);

            let example_name: String = pp_vec.join("_");
            examples.push((example_name, fp));
        }
    }

    for item in examples {
        println!("{}, {}", item.0, item.1);
    }
    Ok(())
}

fn get_example_files(project_root: &str) -> Vec<String> {
    let base_path = Path::new(project_root);
    let examples_path = base_path.join("examples");

    WalkDir::new(&examples_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| {
            e.path()
                .strip_prefix(&base_path)
                .ok()
                .and_then(|p| p.to_str())
                .map(|s| s.to_string())
        })
        .collect()
}
