use clap::{Parser, Subcommand};
use ethers::prelude::*;
use std::io::Write;
use std::sync::Arc;

mod abi;
use abi::contributions::Contributions;
use abi::project::Project;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
    #[arg(long, env, required = true)]
    rpc_url: String,
    #[arg(long, env, required = true)]
    private_key: String,
}

#[derive(Debug, Subcommand)]
enum Command {
    Create {
        project_name: String,
    },
    Initialize {
        #[arg(long, env, required = true)]
        project_address: String,
    },
    SubmitCommits {
        start: String,
        end: String,
        #[arg(short, long, env, required = true)]
        description: String,
        #[arg(long, env, required = true)]
        project_address: String,
    },
    CheckoutProposal {
        proposal_id: u128,
        #[arg(long, env, required = true)]
        project_address: String,
    },
    CastVote {
        approve: bool,
        proposal_id: u128,
        #[arg(long, env, required = true)]
        project_address: String,
    },
    IncludeProposal {
        proposal_id: u128,
        #[arg(long, env, required = true)]
        project_address: String,
    },
    Aggregate {
        #[arg(long, env, required = true)]
        project_address: String,
    },
    // Clone,
    // Pull,
    // Publish,
}

pub async fn cli() -> eyre::Result<()> {
    let Cli { command, rpc_url, private_key } = Cli::parse();

    let provider = Provider::<Http>::try_from(rpc_url)?;
    let chain_id: u64 = provider.get_chainid().await?.try_into().unwrap();

    let deployer: LocalWallet = private_key.parse()?;
    let deployer =
        SignerMiddleware::new(provider, deployer.with_chain_id(chain_id));
    let deployer = Arc::new(deployer);

    match command {
        Command::Create { project_name } => {
            println!("Deploying a new project contract...");

            let contract = Project::deploy(deployer.to_owned(), project_name)?
                .send()
                .await?;

            println!(
                "Created a new project contract: {}",
                ethers::utils::to_checksum(&contract.address(), None)
            );

            let mut env_file = std::fs::OpenOptions::new()
                .append(true)
                .create(true)
                .open(".env")?;

            env_file.write_all(
                format!(
                    "\nPROJECT_ADDRESS={}",
                    ethers::utils::to_checksum(&contract.address(), None)
                )
                .as_bytes(),
            )?;

            println!("Updated the .env file with the contract address");
        }
        Command::Initialize { project_address } => {
            println!("Creating a patch file from the initial commit...");

            std::process::Command::new("bash").args([
                "-c",
                "git show $(git rev-list --max-parents=0 HEAD) --patch > /tmp/init.patch",
            ]).output()?;

            println!("Adding the patch file to IPFS...");

            let hash = std::process::Command::new("ipfs")
                .args(["add", "-Q", "/tmp/init.patch"])
                .output()?
                .stdout;

            std::fs::remove_file("/tmp/init.patch")?;

            println!("Initializing the project contract...");

            let address: Address = project_address.parse()?;
            let contract = Project::new(address, deployer);
            contract.initialize(hash.into()).await?;

            println!("Initialized the project contract");
        }
        Command::SubmitCommits { start, end, description, project_address } => {
            println!("Bundling the commits between {start} and {end} into a patch file...");

            std::process::Command::new("bash").args([
                "-c",
                &format!(
                    "git format-patch --stdout {start}..{end} > /tmp/patches.patch",
                    start = start,
                    end = end
                ),
            ]).output()?;

            println!("Adding the patch file to IPFS...");

            let hash = std::process::Command::new("ipfs")
                .args(["add", "-Q", "/tmp/patches.patch"])
                .output()?
                .stdout;

            std::fs::remove_file("/tmp/patches.patch")?;

            println!("Submitting the patch to the project contract...");

            let address: Address = project_address.parse()?;
            let project = Project::new(address, deployer);

            let proposal_id = project
                .submit_contributions_request(hash.into(), description)
                .await?;

            println!("Submitted the patch to the project contract!");
            println!("Proposal ID: {proposal_id}");
        }
        Command::CheckoutProposal { proposal_id, project_address } => {
            let address: Address = project_address.parse()?;
            let project = Project::new(address, deployer.to_owned());

            let ipfs_hash =
                project.get_proposal_ipfs_hash(proposal_id.into()).await?;

            std::process::Command::new("bash")
                .args(["-c", &format!("ipfs cat {ipfs_hash} | git apply")])
                .output()?;
        }
        Command::CastVote { approve, proposal_id, project_address } => {
            let address: Address = project_address.parse()?;
            let project = Project::new(address, deployer.to_owned());

            let support = if approve { u8::MAX } else { 0 };

            project.cast_vote(proposal_id.into(), support).await?;
        }
        Command::IncludeProposal { proposal_id, project_address } => {
            let address: Address = project_address.parse()?;
            let project = Project::new(address, deployer.to_owned());

            project.include_proposal(proposal_id.into()).await?;
        }
        Command::Aggregate { project_address } => {
            std::process::Command::new("git")
                .args(["diff", "--exit-code"])
                .output()?;

            println!("Creating a new branch to aggregate the changes...");
            std::process::Command::new("git")
                .args(["checkout", "-b", "aggregate"])
                .output()?;

            println!("Resetting the working tree to the initial commit...");
            std::process::Command::new("git")
                .args([
                    "reset",
                    "--hard",
                    "$(git rev-list --max-parents=0 HEAD)",
                ])
                .output()?;

            let address: Address = project_address.parse()?;
            let project = Project::new(address, deployer.to_owned());

            let contributions = project.tracker_contract().await?;
            let contributions = Contributions::new(contributions, deployer);

            let patch_count: u128 =
                contributions.patch_count().await?.as_u128();

            println!("Found {patch_count} patches to apply on top of the initial commit...");

            for token_id in 1..patch_count {
                let ipfs_hash =
                    contributions.get_patch(token_id.into()).await?;

                println!("Applying patch {token_id}...");
                std::process::Command::new("bash")
                    .args(["-c", &format!("ipfs cat {ipfs_hash} | git apply")])
                    .output()?;

                println!("Committing the patch...");

                let description =
                    contributions.get_description(token_id.into()).await?;

                std::process::Command::new("git")
                    .args(["add", "."])
                    .output()?;

                std::process::Command::new("git")
                    .args(["commit", "-m", &description])
                    .output()?;
            }

            println!("Finished aggregating on-chain patches!")
        }
    }

    Ok(())
}
