use directories::ProjectDirs;

mod ui;

const PROJECT_QUALIFIER: &str = "com";

fn main() {
    let project_dirs = ProjectDirs::from(
        PROJECT_QUALIFIER,
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_NAME"),
    )
    .unwrap(); // If this panics, so be it. It is crucial to the program running.
    dbg!(&project_dirs);
}
