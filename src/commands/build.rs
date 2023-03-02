use std::path::Path;
use super::SubCommand;
use clap::Args;
use toml::Table;

#[derive(Args)]
pub struct BuildCommand {
    #[arg(long = "path", default_value = ".")]
    path: String,
}

impl SubCommand for BuildCommand {
    fn execute(&self) {
        println!("Building from path {}", self.path);
        let path = Path::new(&self.path);
        // check if Cargo.toml is present in the path self.path

        if path.join("Cargo.toml").exists() {
            // parse Cargo.toml
            let cargo_toml: Table = toml::from_str(&std::fs::read_to_string(path.join("Cargo.toml")).unwrap()).unwrap();
            let dependencies = cargo_toml.get("dependencies").unwrap().as_table().unwrap();
            if dependencies.contains_key("pgx") {
                println!("Detected that we are building a pgx extension");
                return;
            }
        }
        println!("Did not understand what to build");
    }
}
