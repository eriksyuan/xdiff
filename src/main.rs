use anyhow::Result;
use clap::Parser;
use tokio;
use xdiff::{
    cli::{Action, Args, RunArgs},
    DiffConfig, ExtraArgs,
};

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();

    match args.action {
        Action::Run(a) => run(a).await?,
        _ => panic!("unknown action"),
    }

    Ok(())
}

async fn run(args: RunArgs) -> Result<()> {
    let config_path = args.config.unwrap_or_else(|| "./xdiff.yml".to_string());
    let config = DiffConfig::load_yaml(&config_path.as_str()).await?;

    let profile = config.get_profile(&args.profile).ok_or_else(|| {
        anyhow::anyhow!(
            "Profile {} not found in config file {}",
            args.profile,
            config_path
        )
    })?;

    let extra_args: ExtraArgs = args.extra_params.into();

    profile.diff(&extra_args).await
}
