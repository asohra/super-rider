use directories::ProjectDirs;

mod ui;

const PROJECT_QUALIFIER: &str = "com";

fn main() {
    let project_dirs = ProjectDirs::from(
        PROJECT_QUALIFIER,
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_NAME"),
    )
    .expect("Could not load project directories.");
    dbg!(&project_dirs);
}
