use amiquip::{
    Connection, ConsumerOptions, ExchangeDeclareOptions, FieldTable, QueueDeclareOptions, ConsumerMessage,
};
use anyhow::Result;
use basic_amqp::configuration;

fn main() -> Result<()> {
    let default_url = "amqp://localhost:15672".to_string();
    let routing_key = "app.user.creation";
    let exchange_name = "exchange.app";
    let queue_name = "user";

    let configuration = configuration::load()?;
    let amqp_url = configuration.get("amqp_url").unwrap_or(&default_url);

    println!("Initializing consumer to direct Q...");

    let mut connection = match amqp_url.starts_with("amqps") {
        true => Connection::open(&amqp_url),
        false => Connection::insecure_open(&amqp_url),
    }?;

    let channel = connection.open_channel(None)?;
    let mut exchange_options = ExchangeDeclareOptions::default();
    exchange_options.durable = true;

    let exchange = channel.exchange_declare(
        amiquip::ExchangeType::Direct,
        exchange_name,
        exchange_options,
    )?;
    let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;
    queue.bind(&exchange, routing_key, FieldTable::new())?;

    let consumer = queue.consume(ConsumerOptions {
        no_ack: true,
        ..ConsumerOptions::default()
    })?;

    println!("Waiting for messages with routing key '{routing_key}'. Press Ctrl + C to quit");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(message) => {
                let text = String::from_utf8_lossy(&message.body);
                println!("Got message {} - {:?}", i, text);
            },
            _ => {
                println!("Got a close connection message over the wire...");
                break;
            }
        }
    }

    println!("Closing connection");
    connection.close()?;

    Ok(())
}
