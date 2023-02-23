use amiquip::{
    ConsumerMessage, ConsumerOptions, ExchangeDeclareOptions, ExchangeType, FieldTable,
    QueueDeclareOptions,
};
use anyhow::Result;
use basic_amqp::{amqp_utils, configuration};

fn main() -> Result<()> {
    println!("Starting fanout...");
    let exchange_name = "app.fanout";
    let queue_name = "faning_out";
    let configuration = configuration::load()?;
    let default_url = "amqp://localhost:15672".to_string();
    let amqp_url = configuration.get("amqp_url").unwrap_or(&default_url);

    let mut connection = amqp_utils::get_connection(&amqp_url)?;
    let channel = connection.open_channel(None)?;
    let exchange = channel.exchange_declare(
        ExchangeType::Fanout,
        exchange_name,
        ExchangeDeclareOptions::default(),
    )?;
    let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;

    queue.bind(&exchange, "", FieldTable::new())?;
    let consumer = queue.consume(ConsumerOptions::default())?;

    println!("Waiting for messages on fanout Q");

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(payload) => {
                let data = String::from_utf8_lossy(&payload.body);
                println!("Got a message {} - {}", i, data);
            }
            _ => {
                eprintln!("Got a disconnect message!");
            }
        }
    }

    connection.close()?;
    Ok(())
}
