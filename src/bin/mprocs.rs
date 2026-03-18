#[tokio::main]
async fn main() -> anyhow::Result<()> {
  libmprocs::mprocs::mprocs_main().await
}
