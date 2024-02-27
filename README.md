# Rusty Chat

Rusty Chat is a simple chat application written in Rust, allowing users to interact with LLM's served using Olamma.

## Prerequisites

Before running Rusty Chat, make sure you have the following installed:

- Docker
- Rust (https://www.rust-lang.org/tools/install)

## Getting Started

1. Start the Ollama server by running the following Docker command:

    ```bash
    docker run -d -v ollama:/root/.ollama -p 11434:11434 --name ollama ollama/ollama
    ```

2. Start the Gemma language model in another terminal with:

    ```bash
    docker exec -it ollama ollama run gemma
    ```

3. Clone this repository:

    ```bash
    git clone https://github.com/sybernomad/rustychat.git
    ```

4. Navigate to the project directory:

    ```bash
    cd rustychat
    ```

5. Run the chat bot:

    ```bash
    cargo run
    ```

## Usage

Once the application is running, you can interact with the chatbot by typing a message. You can type "exit" or "quit" to exit the chat.

