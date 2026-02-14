use std::{ffi::OsStr, fs, path::PathBuf};

use crate::module::module::Module;

pub struct ModuleTree {
    pub entry_path: PathBuf,
    pub modules: Vec<Module>,
}

impl ModuleTree {
    pub fn new(path: &PathBuf, entry_file_name: Option<&OsStr>) -> Self {
        let mut modules = vec![];
        walk_dir_and_read_modules(path, &mut modules).expect("Failed to read modules from path");

        let entry_path = if path.is_file() {
            path.clone()
        } else {
            path.join(entry_file_name.unwrap_or(OsStr::new("main.alloy")))
        };

        let tree = Self {
            entry_path,
            modules,
        };

		tree.entry();

		tree
    }

    pub fn entry(&self) -> &Module {
        let entry = self
            .modules
            .iter()
            .find(|m| m.file_path == self.entry_path)
            .expect("Failed to find entry module in modules");
        entry
    }
}

fn walk_dir_and_read_modules<'a>(
    path: &PathBuf,
    modules: &'a mut Vec<Module>,
) -> Result<(), String> {
    if path.is_file() {
        modules.push(read_alloy_file(path)?);
        return Ok(());
    }

    for entry in fs::read_dir(path).map_err(|e| e.to_string())? {
        let entry = entry.map_err(|e| e.to_string())?;
        let path = entry.path();

        if path.is_dir() {
            walk_dir_and_read_modules(&path, modules)?;
        } else if path.extension().is_some_and(|ext| ext == "alloy") {
            modules.push(read_alloy_file(&path)?);
        }
    }

    Ok(())
}

fn read_alloy_file(path: &PathBuf) -> Result<Module, String> {
    if !path.is_file() {
        return Err("Expected a file".to_string());
    }
    let content = fs::read_to_string(path).map_err(|e| e.to_string())?;
    Ok(Module::new(content, path.clone()))
}
