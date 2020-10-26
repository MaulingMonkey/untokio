mod library {
    pub async fn read() -> Result<String, std::io::Error> {
        untokio::v03::spawn(async{
            tokio03::fs::read_to_string("Cargo.toml").await
        }).await.unwrap()
    }
}

fn main() {
    println!("{}", futures::executor::block_on(library::read()).unwrap());
}
