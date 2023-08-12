use clap::{Parser, Subcommand};
use ethers::prelude::*;
use std::sync::Arc;

mod abi;
use abi::project::Project;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    #[arg(long, env, required = true)]
    rpc_url: String,
}

#[derive(Debug, Subcommand)]
enum Commands {
    Create {
        project_name: String,
        #[arg(long, env, required = true)]
        private_key: String,
    },
    // Initialize,
    // Clone,
    // Pull,
    // Publish,
}

pub async fn cli() -> eyre::Result<()> {
    let Cli { command, rpc_url } = Cli::parse();

    match command {
        Commands::Create { project_name, private_key } => {
            let provider = Provider::<Http>::try_from(rpc_url)?;
            let chain_id: u64 =
                provider.get_chainid().await?.try_into().unwrap();

            let deployer: LocalWallet = private_key.parse()?;
            let deployer = SignerMiddleware::new(
                provider,
                deployer.with_chain_id(chain_id),
            );
            let deployer = Arc::new(deployer);

            let contract = Project::deploy(deployer.to_owned(), project_name)?
                .send()
                .await?;

            println!(
                "Created a new project contract: {}",
                ethers::utils::to_checksum(&contract.address(), None)
            );
        }
    }

    Ok(())
}
