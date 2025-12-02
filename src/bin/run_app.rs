use celeb_shortest_distance::configuration;
use celeb_shortest_distance::webapp::Application;
use celeb_shortest_distance::webapp::telemetry;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let subscriber =
        telemetry::get_subscriber("celeb_search".into(), "info".into(), std::io::stdout);
    telemetry::init_subscriber(subscriber);
    let settings = configuration::get_configuration().expect("Failed to read configuration.");
    let app = Application::build(settings).await?;

    let _ = app.run_until_stopped().await;
    Ok(())
}
