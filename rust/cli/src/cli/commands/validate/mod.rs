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
            Box::new(|json_string, filepath, schema_name| {
                if filepath == "-" {
                    return;
                }
                let final_filepath = std::path::Path::join(
                    self.config.parent().unwrap(),
                    filepath,
                );
                crate::cli::utils::does_file_have_changes(
                    &final_filepath,
                    json_string,
                    schema_name,
                )
                .unwrap();
            }),
        )
        .unwrap();
    }
}
