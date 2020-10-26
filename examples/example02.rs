mod library {
    use reqwest010 as reqwest; // relies on tokio 0.2

    pub async fn download() -> Result<String, reqwest::Error> {
        untokio::v02::spawn(async {
            reqwest::get("http://example.com/").await?.text().await
        }).await.unwrap()
    }
}

fn main() {
    println!("{}", futures::executor::block_on(library::download()).unwrap());
}
