use loco_rs::cli;
use krimmer::app::App;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    cli::main::<App>().await
}
