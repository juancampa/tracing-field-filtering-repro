use tracing::{info, info_span};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::{filter::EnvFilter, registry};

pub fn setup_tracing() {
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("[{method=\"GET\"}]=warn,debug"))
        .unwrap();

    let subscriber = registry::Registry::default()
        .with(filter_layer)
        .with(tracing_subscriber::fmt::Layer::default());

    tracing::subscriber::set_global_default(subscriber).expect("setting global default failed");
}

#[tokio::main]
async fn main() {
    setup_tracing();
    {
        let span = info_span!("span-1", method = "GET");
        let _enter = span.enter();
        info!("This should not show");
    }
    {
        let span = info_span!("span-2", method = "POST");
        let _enter = span.enter();
        info!("This should show");
    }
}
