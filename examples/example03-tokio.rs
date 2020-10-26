use tokio03 as tokio;

mod library {
    pub async fn read() -> Result<String, std::io::Error> {
        untokio::v03::spawn(async{
            tokio03::fs::read_to_string("Cargo.toml").await
        }).await.unwrap()
    }
}

fn main() {
    untokio::v03::set_runtime(tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap());
    untokio::v03::runtime().block_on(async {
        println!("{}", library::read().await.unwrap());
    });
}
