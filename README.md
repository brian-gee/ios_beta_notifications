# iOS Beta Update Notifier

This Rust script monitors the Apple Developer RSS feed for new iOS beta releases and sends notifications to a Discord channel using a webhook.

## Features

- Periodically checks the Apple Developer RSS feed for updates
- Sends Discord notifications for new iOS beta releases
- Configurable check interval
- Test mode for easier debugging and testing

## Prerequisites

- Rust and Cargo installed on your system
- A Discord webhook URL

## Setup

1. Clone the repository:

```sh
git clone https://github.com/brian-gee/ios_beta_notifications.git
cd ios_beta_notifications
```

2. Create a `.env` file in the project root directory with the following content:

```sh
DISCORD_WEBHOOK_URL=your_discord_webhook_url_here
```

3. Build the project:

```sh
cargo build --release
```

## Usage

Run the script with:

```sh
cargo run --release
```

For testing purposes, you can set `TEST_MODE` to `true` in the script. This will consider entries from the last 24 hours as new and reduce the check interval to 1 minute.

## Deployment

To run the script continuously on a Raspberry Pi or other system:

1. Build the release version: `cargo build --release`
2. Use a tool like `screen` or `tmux` to keep the script running in the background, or set it up as a systemd service.

Example using `screen`:

```sh
screen -S ios-beta-notifier
./target/release/ios_beta_notifications
```

Detach from the screen session with `Ctrl+A`, then `D`.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the GPL-3.0 license - see the [LICENSE](LICENSE) file for details.
