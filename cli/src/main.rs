mod bindings;
mod cmd;
mod graphql;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let command = cmd::parse_args();
    command.run().await?;
    Ok(())
}
