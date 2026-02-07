# Kindle Sender

[![Rust](https://img.shields.io/badge/language-Rust-orange)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Version](https://img.shields.io/badge/version-0.1.0-green.svg)](https://github.com/nathan-mittelette/kindle-sender)

A Rust CLI application that automatically sends e-book files to your Kindle device via email, leveraging Microsoft Azure/Graph API for secure authentication and email delivery.

## 📚 Features

- **Email-Based Delivery**: Sends e-books directly to your Kindle's email address
- **Microsoft Graph API Integration**: Uses Azure for secure authentication and email sending
- **Automatic File Management**: Moves files from a "to-send" directory to a "sent" directory after processing
- **Token Caching**: Securely stores authentication tokens for seamless reuse
- **Batch Processing**: Send multiple e-book files in one command
- **Secure Authentication**: OAuth 2.0 flow with automatic token refresh

## 🚀 Installation

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install) (2021 edition or later)
- Microsoft Azure account with an application registered
- Kindle device with Send-to-Kindle email address

### Building from Source

```bash
# Clone the repository
git clone https://github.com/nathan-mittelette/kindle-sender.git
cd kindle-sender

# Build the project
cargo build --release

# Run the application
./target/release/kindle-sender send
```

## ⚙️ Configuration

Create a `config.json` file in the project root directory with the following structure:

```json
{
  "callback_uri": "http://localhost:8080/callback",
  "ebook_to_send_directory": "/path/to/your/ebooks/to/send",
  "ebook_sent_directory": "/path/to/your/sent/ebooks",
  "receivers": ["your-kindle-email@kindle.com"],
  "azure": {
    "client_id": "your-azure-app-client-id",
    "client_secret": "your-azure-app-client-secret",
    "tenant_id": "common"
  }
}
```

### Azure Application Setup

1. Register a new application in the [Azure Portal](https://portal.azure.com)
2. Add the Microsoft Graph API permission `Mail.Send`
3. Configure a redirect URI as `http://localhost:8080/callback`
4. Create a client secret and note both the client ID and secret

### Kindle Email Setup

1. Find your Kindle's email address in your Amazon account settings
2. Add your sending email address to the approved senders list in Amazon settings

## 🔧 Usage

```bash
# Send all e-books from the configured directory
kindle-sender send
```

When you run the application for the first time, it will:
1. Open a browser window for you to authenticate with your Microsoft account
2. Ask for permission to send emails on your behalf
3. Store the authentication token securely for future use

## 📁 Project Structure

- `src/main.rs` - Entry point with CLI interface definition
- `src/commands/` - Implementation of CLI commands
- `src/models/` - Data structures and error types
- `src/services/` - Core functionality services:
  - `azure_service.rs` - Authentication with Microsoft Azure
  - `kindle_service.rs` - Email sending to Kindle devices
  - `file_service.rs` - File system operations
  - `send_service.rs` - Orchestration service

## 🔒 Security

- Authentication tokens are stored securely in the user's home directory
- Application uses OAuth 2.0 flow with proper token refresh
- No plaintext credentials are stored in the application

## 🛠️ Development

### Building with Optimizations

The project includes custom optimization profiles:

```bash
# Development build with basic optimizations
cargo build

# Release build with maximum size optimizations
cargo build --release
```

### Adding Support for New File Types

The application currently supports all file formats accepted by Kindle devices:
- AZW, AZW3 (Kindle Format)
- MOBI (Mobipocket)
- PDF (Portable Document Format)
- TXT (Plain Text)
- EPUB (with automatic conversion by Amazon)

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🙏 Acknowledgements

- [Rust](https://www.rust-lang.org/) programming language
- [Microsoft Graph API](https://developer.microsoft.com/en-us/graph)
- [Tokio](https://tokio.rs/) for async runtime
- [Reqwest](https://github.com/seanmonstar/reqwest) for HTTP requests
- [Warp](https://github.com/seanmonstar/warp) for the OAuth callback server

## Useful commands

```bash
# Build for Linux GNU from MacOS (arm64)
brew install zig
cargo install cargo-zigbuild
cargo zigbuild --target x86_64-unknown-linux-gnu --release
```