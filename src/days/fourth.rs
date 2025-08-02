use std::fs;

pub fn read_dir() -> std::io::Result<()> {
    let files = fs::read_dir(".")?;

    for file in files {
        let file = file?;
        let path = file.path();

        if path.is_dir() {
            println!("{}", path.display());
        }
    }

    Ok(())
}
