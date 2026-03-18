#[tokio::main]
async fn main() -> anyhow::Result<()> {
  libmprocs::dekit::dekit_main().await
}
