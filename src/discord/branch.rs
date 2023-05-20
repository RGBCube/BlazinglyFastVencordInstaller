use std::path::PathBuf;

const BRANCHES: [&str; 3] = [
    "pbt",
    "canary",
    "development",
];

pub fn get_branch(path: &PathBuf) -> &'static str {
    let name = path.file_name().unwrap().to_string_lossy();

    for branch in BRANCHES {
        if name.ends_with(branch) {
            return branch;
        }
    }

    "stable"
}
