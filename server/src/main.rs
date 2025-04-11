mod collector;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let handle = tokio::spawn(collector::data_collector());
    handle.await??;
    Ok(())
}
