use clap::Parser;

#[derive(Debug, Parser)]
struct Cli {
    dir: String,
    #[arg(long)]
    raw_ext: String,
    #[arg(long)]
    unlink: bool,
    #[arg(long)]
    rename: bool,
}

fn main() {
    let cli = Cli::parse();

    println!(
        "Finding JPEG with existing raw (.{}) files in {}:",
        cli.raw_ext, &cli.dir
    );

    let items = std::fs::read_dir(&cli.dir)
        .unwrap_or_else(|_| panic!("Failed to read directory {}", cli.dir));

    let mut matched_files_count = 0;

    for item in items {
        let item = item.expect("Failed to read item");
        let path = item.path();
        let path_str = path.to_str().expect("Failed to convert path to string");

        if path_str.ends_with(".JPG") || path_str.ends_with(".jpg") {
            let raw_path = path.with_extension(&cli.raw_ext);

            if raw_path.exists() {
                matched_files_count += 1;

                println!(
                    "Found {} with existing raw file {}",
                    path_str,
                    raw_path.to_str().unwrap()
                );

                if cli.unlink {
                    match std::fs::remove_file(path_str) {
                        Ok(()) => {
                            println!("Removed {}", path_str);
                        }
                        Err(_) => {
                            eprintln!("Failed to remove {}", path_str);
                        }
                    }
                } else if cli.rename {
                    let new_path = path
                        .join(format!(
                            ".{}",
                            path.extension()
                                .map(|ext| ext.to_string_lossy())
                                .unwrap_or_else(|| std::borrow::Cow::Borrowed("Unknown"))
                        ))
                        .with_extension("to_delete");

                    match std::fs::rename(path_str, &new_path) {
                        Ok(()) => {
                            println!("Renamed {} to {}", path_str, new_path.to_str().unwrap());
                        }
                        Err(_) => {
                            eprintln!(
                                "Failed to rename {} to {}",
                                path_str,
                                new_path.to_str().unwrap()
                            );
                        }
                    }
                }
            }
        }
    }

    println!("Matched files count: {}", matched_files_count);
}
