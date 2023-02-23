use amiquip::Publish;
use anyhow::Result;
use basic_amqp::{amqp_utils, configuration};

fn main() -> Result<()> {
    let exchange_name = "app.fanout";
    let configuration = configuration::load()?;
    let default_url = "amqp://localhost:15672".to_string();
    let amqp_url = configuration.get("amqp_url").unwrap_or(&default_url);

    let mut connection = amqp_utils::get_connection(amqp_url)?;
    let channel = connection.open_channel(None)?;
    let exchange = channel.exchange_declare_passive(exchange_name)?;

    let message = r#"{"message": "Faning out!"}"#.as_bytes();
    println!("Sending fanout message");

    exchange.publish(Publish::new(message, ""))?;

    connection.close()?;
    Ok(())
}
