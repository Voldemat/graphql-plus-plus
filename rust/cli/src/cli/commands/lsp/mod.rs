mod codec;
mod context;
mod file_type;
mod handlers;
mod location;
mod server;
mod shared;

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

async fn run(
    config_directory_path: std::path::PathBuf,
    config: crate::cli::config::Config,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut reader =
        tokio_util::codec::FramedRead::new(tokio::io::stdin(), codec::LspCodec);
    let writer = std::sync::Arc::new(tokio::sync::Mutex::new(
        tokio_util::codec::FramedWrite::new(
            tokio::io::stdout(),
            codec::LspCodec,
        ),
    ));
    let context = context::ServerContext {
        config_directory_path,
        config,
        buffers: Default::default(),
    };
    let mut server = server::build_jsonrpc_server(&context, writer);
    while let Some(frame) = futures_util::StreamExt::next(&mut reader).await {
        let request_str = match frame {
            Ok(req) => req,
            Err(e) => {
                eprintln!("Header framing error: {}", e);
                continue;
            }
        };

        server.handle_request(&request_str).await?;
    }
    Ok(())
}

impl Args {
    pub fn execute(self: &Self) {
        let config =
            crate::cli::config::Config::from_yaml_file_path(&self.config);
        let runtime = tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap();
        runtime
            .block_on(run(
                std::path::absolute(self.config.parent().unwrap()).unwrap(),
                config,
            ))
            .unwrap();
    }
}
