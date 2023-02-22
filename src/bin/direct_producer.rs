use amiquip::{Connection, Publish, ExchangeDeclareOptions};
use anyhow::Result;
use basic_amqp::configuration;

fn main() -> Result<()> {
    let configuration = configuration::load()?;
    let default_url = "amqp://localhost:15672".to_string();
    let routing_key = "app.user.creation";
    let exchange_name = "exchange.app";

    let amqp_url = configuration.get("amqp_url").unwrap_or(&default_url);

    let mut connection = match amqp_url.starts_with("amqps") {
        true => Connection::open(&amqp_url),
        false => Connection::insecure_open(&amqp_url),
    }?;
    let channel = connection.open_channel(None)?;
    let mut exchange_options = ExchangeDeclareOptions::default();
    exchange_options.durable = true;
    let exchange = channel.exchange_declare_passive(exchange_name)?;

    println!("Publishing message");
    exchange.publish(Publish::new("Hello!".as_bytes(), routing_key))?;

    connection.close()?;

    println!("Done!");
    Ok(())
}
