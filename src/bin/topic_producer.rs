use amiquip::Publish;
use anyhow::Result;
use basic_amqp::{configuration, amqp_utils::get_connection};

fn main () -> Result<()> {
    let default_url = "amqp://localhost:15672".to_string();
    let created_topic = "app.user.created";
    let deleted_topic = "app.user.deleted";
    let exchange_name = "exchange.app_user";

    let config = configuration::load()?;
    let amqp_url = config.get("amqp_url").unwrap_or(&default_url);

    let mut connection = get_connection(amqp_url)?;
    let channel = connection.open_channel(None)?;
    let exchange = channel.exchange_declare_passive(exchange_name)?;

    println!("Publish a delete message");
    let message = r#"{"message": "User deleted"}"#.as_bytes();
    exchange.publish(Publish::new(message, deleted_topic))?;

    println!("Publish a create message");
    let message = r#"{"message": "User create"}"#.as_bytes();
    exchange.publish(Publish::new(message, created_topic))?;

    println!("Done!");

    Ok(())
}