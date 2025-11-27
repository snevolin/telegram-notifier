# telegram-notifier

Утилита на Rust + teloxide для отправки уведомлений в Telegram и режима "прослушивания" входящих сообщений.

## Установка

- Сборка бинарника: `cargo build --release` (понадобится Rust toolchain).
- Пример конфигурации: `etc/telegram-notifier/config.toml` (копируется в `/etc/telegram-notifier/config.toml` или `~/.config/telegram-notifier/config.toml`).

## Конфигурация

Приоритет источников токена/`chat_id`:
1. CLI флаги `--token`, `--chat-id`.
2. Пользовательский конфиг (см. пути ниже).
3. Системный конфиг `/etc/telegram-notifier/config.toml` (Linux).

Расположение пользовательского конфига:
- Linux: `~/.config/telegram-notifier/config.toml`
- macOS: `~/Library/Application Support/telegram-notifier/config.toml`
- Windows: `%APPDATA%\telegram-notifier\config.toml` (например, `C:\Users\<user>\AppData\Roaming\telegram-notifier\config.toml`)

Формат TOML:

```toml
bot_token = "123456:ABC"
chat_id = 123456789
```

`chat_id` нужен только для отправки, его можно узнать в режиме `listen`.

## Получение токена бота (BotFather)

1. В Telegram откройте [@BotFather](https://t.me/BotFather) и отправьте `/start`.
2. Отправьте `/newbot`, задайте отображаемое имя, затем уникальный username (должен заканчиваться на `bot`).
3. BotFather вернёт `HTTP API` токен — скопируйте его в конфиг или передавайте через `--token`.
4. (Опционально) Через `/setdescription`, `/setabouttext`, `/setuserpic` можно настроить описание/аватар.
5. Если нужно отозвать/сгенерировать новый токен позже — команда `/revoke` в BotFather и обновление конфига.

## Использование

- Слушать входящие и выводить дату/время, `chat_id` (красным), username, текст:
  ```bash
  telegram-notifier listen --token "$BOT_TOKEN"
  ```

- Отправить сообщение в заданный `chat_id`:
  ```bash
  telegram-notifier send --message "done" --token "$BOT_TOKEN" --chat-id 123456789
  ```

### Типичный сценарий
1. Запустите прослушивание, чтобы узнать свой `chat_id`:
   ```bash
   telegram-notifier listen --token "$BOT_TOKEN"
   ```
2. В Telegram откройте бота и отправьте `/start` (или любое сообщение); в консоли появится строка с вашим `chat_id`.
3. Скопируйте `chat_id` в конфиг или передавайте через `--chat-id`.
4. Отправляйте уведомления:
   ```bash
   telegram-notifier send --message "task finished" --token "$BOT_TOKEN" --chat-id <ваш_chat_id>
   ```
5. Если токен и `chat_id` уже указаны в конфиге, можно не передавать флаги:
   ```bash
   telegram-notifier send --message "task finished"
   ```
