use anyhow::Result;
use xdiff::DiffConfig;
use tokio;

#[tokio::main]
async fn main() -> Result<()> {
    let content = include_str!("../fixtures/test.yml");
    let config = DiffConfig::from_yaml(&content);



    print!("{:#?}",config);
    Ok(())
}
