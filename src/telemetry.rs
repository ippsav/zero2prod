use tracing::Subscriber;
use tracing::subscriber::set_global_default;
use tracing_bunyan_formatter::{BunyanFormattingLayer, JsonStorageLayer};
use tracing_subscriber::{EnvFilter, Registry, layer::SubscriberExt, fmt::MakeWriter};
use tracing_log::LogTracer;






pub fn get_subscriber<Sink>(name: &'static str, env_filter: &'static str, sink: Sink) -> impl Subscriber + Send + Sync
    where
    Sink: for<'a> MakeWriter<'a> + Sync + Send + 'static
{
    let env_filter_layer = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(env_filter));
    let formatting_layer = BunyanFormattingLayer::new(name.into(), sink);
    Registry::default()
    .with(env_filter_layer)
    .with(formatting_layer)
    .with(JsonStorageLayer)
}

pub fn init_subscriber(subscriber: impl Subscriber + Send + Sync){
    LogTracer::init().expect("could not set global log tracer");
    set_global_default(subscriber).expect("could not set default subscriber");
}