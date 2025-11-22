use directories::ProjectDirs;

mod file_management;
mod ui;

const PROJECT_QUALIFIER: &str = "com"; // as of 22/11/2025, this is used for project directories on certain platforms.

fn main() {
    let project_dirs = ProjectDirs::from(
        PROJECT_QUALIFIER,
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_NAME"),
    )
    .expect("Could not load project directories.");
    dbg!(&project_dirs);

    let assets_dir = file_management::get_assets_dir();
    dbg!(&assets_dir);
}
