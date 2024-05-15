use celeb_shortest_distance::webapp::launch;


#[tokio::main]
async fn main() -> Result<(), std::io::Error>{
    println!("Hello, world!");
    launch().await;
    Ok(())
}
