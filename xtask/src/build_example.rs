use anyhow::Result;
use anyhow::anyhow;
use clap::ArgMatches;
use std::fs;
use std::path::Path;
use walkdir::WalkDir;

use toml_edit::DocumentMut;

// 子命令处理函数示例
pub fn handle_example(_matches: &ArgMatches) -> Result<()> {
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

    examples.sort_by(|a, b| a.0.cmp(&b.0));

    let cargo_toml_path = root_path.join("Cargo.toml");

    let cargo_toml = fs::read_to_string(&cargo_toml_path)?;
    let mut doc = cargo_toml.parse::<DocumentMut>()?;

    let mut doc_example = toml_edit::ArrayOfTables::new();
    for item in examples {
        let mut table = toml_edit::Table::new();
        table["name"] = toml_edit::value(item.0);
        table["path"] = toml_edit::value(item.1);
        doc_example.push(table);
    }

    doc["example"] = toml_edit::Item::ArrayOfTables(doc_example);

    fs::write(cargo_toml_path, doc.to_string())?;

    Ok(())
}

fn get_example_files(project_root: &str) -> Vec<String> {
    let base_path = Path::new(project_root);
    let examples_path = base_path.join("examples");

    WalkDir::new(&examples_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter(|e| e.path().extension().and_then(|ext| ext.to_str()) == Some("rs"))
        .filter_map(|e| {
            e.path()
                .strip_prefix(base_path)
                .ok()
                .and_then(|p| p.to_str())
                .map(|s| s.to_string())
        })
        .collect()
}
