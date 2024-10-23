use crate::*;

#[tokio::test]
async fn get_rune_pages() -> anyhow::Result<()> {
    let l = lcu::Connection::new()?;
    println!("{:#?}", l.rune_pages().await?);
    Ok(())
}
