use amiquip::{Publish, AmqpProperties};
use anyhow::Result;
use basic_amqp::{configuration, amqp_utils};

fn main() -> Result<()> {
    let configuration = configuration::load()?;
    let default_url = "amqp://localhost:15672".to_string();
    let routing_key = "queue.app.anon";

    let amqp_url = configuration.get("amqp_url").unwrap_or(&default_url);

    let mut connection = amqp_utils::get_connection(amqp_url)?;
    let channel = connection.open_channel(None)?;
    channel.basic_publish("", Publish::with_properties(
        b"Hello, world!",
        routing_key,
        AmqpProperties::default().with_delivery_mode(2),
    ))?;

    connection.close()?;

    println!("Done!");
    Ok(())
}
