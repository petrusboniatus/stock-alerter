# stock-aleter
Performs alerts in telegram and raspberry pi Buzzer if there is stock of certain articles


## Build
Inside raspberry pi
```
cargo build --release
```

## Usage
You will need
1. A telegram bot with a telegram token
2. A chatId with that bot
3. A raspberry-pi with a buzzer connected to line `23`
4. file where each line is a url to the pc-componentes item

```
stock-aleter --web-pages-list-file <web-pages-list-file>
             --telegram-token <telegram-token>
             --chat-id <chat-id>
             --min-duration <min-duration>
             --max-added-duration <max-added-duration>
```