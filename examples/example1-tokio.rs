use tokio1 as tokio;

mod library {
    pub async fn read() -> Result<String, std::io::Error> {
        untokio::v1::spawn(async{
            tokio1::fs::read_to_string("Cargo.toml").await
        }).await.unwrap()
    }
}

fn main() {
    untokio::v1::set_runtime(tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap());
    untokio::v1::runtime().block_on(async {
        println!("{}", library::read().await.unwrap());
    });
}
