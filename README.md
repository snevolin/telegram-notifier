# telegram-notifier

CLI in Rust + teloxide for sending Telegram notifications and a "listen" mode that prints incoming messages.

## Install

- Build binary: `cargo build --release` (Rust toolchain required).
- Sample config: `etc/telegram-notifier/config.toml` (copy to `/etc/telegram-notifier/config.toml` or `~/.config/telegram-notifier/config.toml`).

## Configuration

Priority of token/`chat_id` sources:
1. CLI flags `--token`, `--chat-id`.
2. User config `~/.config/telegram-notifier/config.toml`.
3. System config `/etc/telegram-notifier/config.toml`.

TOML format:

```toml
bot_token = "123456:ABC"
chat_id = 123456789
```

`chat_id` is needed only for sending; you can get it in `listen` mode.

## Getting a bot token (BotFather)

1. In Telegram, open [@BotFather](https://t.me/BotFather) and send `/start`.
2. Send `/newbot`, pick a display name, then pick a unique username (must end with `bot`).
3. BotFather will return `HTTP API` token — copy it into your config or pass via `--token`.
4. (Optional) Send `/setdescription`, `/setabouttext`, `/setuserpic` to customize the bot.
5. To revoke/regenerate a token later, send `/revoke` to BotFather and update your config.

## Usage

- Listen for incoming messages and print date/time, `chat_id` (red), username, text:
  ```bash
  telegram-notifier listen --token "$BOT_TOKEN"
  ```

- Send a message to a given `chat_id`:
  ```bash
  telegram-notifier send --message "done" --token "$BOT_TOKEN" --chat-id 123456789
  ```

### Typical workflow
1. Start listening to discover your `chat_id`:
   ```bash
   telegram-notifier listen --token "$BOT_TOKEN"
   ```
2. In Telegram, open the bot and send `/start` (or any message); the console output will show your `chat_id`.
3. Copy that `chat_id` into your config or pass via `--chat-id`.
4. Send notifications:
   ```bash
   telegram-notifier send --message "task finished" --token "$BOT_TOKEN" --chat-id <your_chat_id>
   ```
