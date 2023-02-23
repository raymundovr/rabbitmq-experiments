use amiquip::{ExchangeType, FieldTable, ConsumerOptions, ConsumerMessage};
use anyhow::Result;
use basic_amqp::{amqp_utils::get_connection, configuration};

fn main() -> Result<()> {
    let default_url = "amqp://localhost:15672".to_string();
    let topic = "app.user.*";
    let exchange_name = "exchange.app_user";

    let config = configuration::load()?;
    let amqp_url = config.get("amqp_url").unwrap_or(&default_url);

    let mut connection = get_connection(amqp_url)?;
    let channel = connection.open_channel(None)?;
    let exchange = channel.exchange_declare(
        ExchangeType::Topic,
        exchange_name,
        amiquip::ExchangeDeclareOptions::default(),
    )?;

    let queue = channel.queue_declare("", amiquip::QueueDeclareOptions::default())?;
    queue.bind(&exchange, topic, FieldTable::default())?;

    println!("Listening to messages on Q {} for topic {topic}", queue.name());

    let consumer = queue.consume(ConsumerOptions::default())?;
    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            ConsumerMessage::Delivery(payload) => {
                let data = String::from_utf8_lossy(&payload.body);
                println!("Received message {i}: {data}");
            },
            _ => {
                eprintln!("Received a disconnect message!");
            }
        }
    }

    connection.close()?;

    Ok(())
}
