# 📔 Olly Journal Backend

A modern journaling backend application powered by Axum and Groq AI. This application allows users to chat with an AI assistant and automatically generate journal entries from their conversations.

## 🚀 Features

- **AI-Powered Conversations**: Chat with Groq's powerful language models (gemma2-9b-it)
- **Automatic Journal Generation**: Convert your conversations into beautifully formatted Markdown journal entries
- **Session Management**: Maintains conversation history for each user session
- **Modern REST API**: Built with Axum, Rust's asynchronous web framework
- **CORS Enabled**: Ready for integration with any frontend application

## 🛠️ Tech Stack

- **[Axum](https://github.com/tokio-rs/axum)**: High-performance, async web framework
- **[Tokio](https://tokio.rs/)**: Asynchronous runtime for Rust
- **[Groq AI API](https://console.groq.com/docs)**: Fast, high-quality language model inference
- **[Serde](https://serde.rs/)**: Serialization/deserialization framework
- **[Parking_lot](https://docs.rs/parking_lot/latest/parking_lot/)**: Efficient synchronization primitives

## 📋 API Endpoints

- **POST /chat**: Send messages to the AI and receive responses
  ```json
  // Request
  {
    "session_id": "optional-session-id",
    "message": "Hello, how can I use journaling to improve my life?"
  }
  
  // Response
  {
    "session_id": "generated-or-provided-session-id",
    "response": "AI's response text here..."
  }
  ```

- **POST /generate-journal**: Convert a session's conversation history into a Markdown journal
  ```json
  // Request
  {
    "session_id": "your-session-id"
  }
  
  // Response
  {
    "journal": "## Journal Entry - March 31, 2025\n\n- User: Hello...\n- AI: ..."
  }
  ```

- **GET /health**: Health check endpoint
  ```
  // Response
  OK
  ```

## 🏗️ Project Structure

```
ollynotes-backend/
├── src/
│   ├── main.rs               # Server setup and configuration
│   ├── api/                  # API endpoints and route handlers
│   │   ├── mod.rs           
│   │   └── routes.rs         
│   ├── models/               # Data structures and types
│   │   └── mod.rs            
│   ├── services/             # Business logic
│   │   ├── mod.rs
│   │   ├── groq.rs           # Groq AI integration
│   │   ├── journal.rs        # Journal generation
│   │   └── session.rs        # Session management
│   └── utils/                # Utility functions
│       └── mod.rs
├── Cargo.toml                # Project dependencies
├── .env                      # Environment variables
└── README.md                 # This file
```

## 🚦 Getting Started

### Prerequisites

- Rust (latest stable version)
- A Groq API key (get one at [console.groq.com](https://console.groq.com))

### Installation

1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/olly-journal-backend.git
   cd olly-journal-backend
   ```

2. Create a `.env` file in the project root and add your Groq API key
   ```
   GROQ_API_KEY=your-groq-api-key-here
   ```

3. Build and run the application
   ```bash
   cargo run
   ```

4. The server will start on `http://localhost:8080`

### Usage Examples

#### Chat with the AI

```bash
curl -X POST http://localhost:8080/chat \
  -H "Content-Type: application/json" \
  -d '{"message": "Tell me about journaling benefits"}'
```

#### Generate a Journal from a Session

```bash
curl -X POST http://localhost:8080/generate-journal \
  -H "Content-Type: application/json" \
  -d '{"session_id": "your-session-id"}'
```

## 📝 Development

### Running Tests

```bash
cargo test
```

### Building for Production

```bash
cargo build --release
```

The optimized binary will be in `target/release/olly-journal-backend`.

## 🔮 Future Plans

- Database integration for persistent storage
- User authentication and accounts
- Advanced journal formatting options
- Sentiment analysis of journal entries
- Frontend integration with Next.js

## 👏 Acknowledgements

- [Groq](https://groq.com/) for their powerful and fast AI models
- [Axum](https://github.com/tokio-rs/axum) project for the excellent web framework
- All open-source contributors whose libraries made this project possible 