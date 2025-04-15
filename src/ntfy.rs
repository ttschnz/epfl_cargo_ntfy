use reqwest::blocking::Client;

pub fn send_notification(topic: &str, message: &str) -> Result<(), Box<dyn std::error::Error>> {
    println!("sending notification: {}", message);
    let client = Client::new();
    client
        .post(&format!("https://ntfy.sh/{}", topic))
        .header("Content-Type", "text/plain")
        .body(message.to_string())
        .send()?;
    Ok(())
}
