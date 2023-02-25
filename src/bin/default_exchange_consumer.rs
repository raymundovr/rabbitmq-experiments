use amiquip::{
    ConsumerOptions, QueueDeclareOptions, ConsumerMessage,
};
use anyhow::Result;
use basic_amqp::{configuration, amqp_utils};

fn main() -> Result<()> {
    let default_url = "amqp://localhost:15672".to_string();
    let queue_name = "queue.app.anon";

    let configuration = configuration::load()?;
    let amqp_url = configuration.get("amqp_url").unwrap_or(&default_url);

    println!("Initializing consumer to direct Q on default exchange...");

    let mut connection = amqp_utils::get_connection(amqp_url)?;

    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare(queue_name, QueueDeclareOptions::default())?;

    let consumer = queue.consume(ConsumerOptions {
        no_ack: true,
        ..ConsumerOptions::default()
    })?;

    println!("Waiting for messages with routing key '{queue_name}'. Press Ctrl + C to quit");

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
