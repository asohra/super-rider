use std::{env::current_exe, path::PathBuf};

fn get_data_path() -> Result<PathBuf, std::io::Error> {
    current_exe()
        // TODO: append data folder to this path. If data folder doesn't exist yet, create it.
}
