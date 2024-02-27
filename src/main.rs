use clap::Parser;
use ollama_rs::{
    generation::chat::{request::ChatMessageRequest, ChatMessage, ChatMessageResponseStream},
    Ollama,
};
use tokio::io::{stdout, AsyncWriteExt};
use tokio_stream::StreamExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    // The IP or hostname the Ollama host
    ollama_host: String,

    #[arg(long, default_value = "11434")]
    /// The port the Ollama host is listening on
    ollama_port: u16,

    #[arg(long, default_value = "gemma:latest")]
    /// The model to use
    #[arg(short, long)]
    model: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let ollama = Ollama::new(format!("http://{}", args.ollama_host), args.ollama_port);

    let mut stdout = stdout();

    let mut messages: Vec<ChatMessage> = vec![];

    println!("Type 'exit' or 'quit' to exit Rusty Chat.");

    loop {
        stdout.write_all(b"\n> ").await?;
        stdout.flush().await?;

        let mut input = String::new();
        std::io::stdin().read_line(&mut input)?;

        let input = input.trim_end();
        if input.eq_ignore_ascii_case("exit") || input.eq_ignore_ascii_case("quit") {
            break;
        }

        let user_message = ChatMessage::user(input.to_string());
        messages.push(user_message);

        let mut stream: ChatMessageResponseStream = ollama
            .send_chat_messages_stream(ChatMessageRequest::new(
                "gemma:latest".to_string(),
                messages.clone(),
            ))
            .await?;

        let mut response = String::new();
        while let Some(Ok(res)) = stream.next().await {
            if let Some(assistant_message) = res.message {
                stdout
                    .write_all(assistant_message.content.as_bytes())
                    .await?;
                stdout.flush().await?;
                response += assistant_message.content.as_str();
            }
        }
        messages.push(ChatMessage::assistant(response));
    }

    Ok(())
}
