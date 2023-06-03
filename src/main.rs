use tide::prelude::*;

#[async_std::main]
async fn main() -> tide::Result<()> {
    async_std::task::spawn(async {
        let mut app = tide::new();
        app.at("/").get(|_| async { Ok("Hello, World!") });
        app.listen("127.0.0.1:8000").await?;
        Result::<_, std::io::Error>::Ok(())
    })
    .await
    .unwrap();
    
    Ok(())
}
