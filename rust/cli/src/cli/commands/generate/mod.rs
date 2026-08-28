#[derive(clap::Args)]
pub struct Args {
    #[arg(
        short,
        long,
        help = "path to yaml config file",
        default_value = "./gql.yaml"
    )]
    pub config: std::path::PathBuf,
}

impl Args {
    pub fn execute(self: &Self) {
        let config =
            crate::cli::config::Config::from_yaml_file_path(&self.config);
        crate::cli::utils::run_config_action(
            &self.config,
            &config,
            Box::new(|json_string, filepath, _| {
                if filepath == "-" {
                    println!("{}", json_string);
                } else {
                    let final_filepath = std::path::Path::join(
                        self.config.parent().unwrap(),
                        filepath,
                    );
                    std::fs::write(final_filepath, json_string).unwrap();
                }
            }),
        )
        .unwrap();
    }
}
