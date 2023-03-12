## ChatGPT CLI

A lightweight ChatGPT CLI - Chat completion. 

Interact with ChatGPT from your terminal and save the conversation in a text file.

![CLI Example](assets/screenshot.png)


## Download 

### Binary

```
# Macos
wget https://github.com/imadelh/Chat-CLI/releases/download/v0.1.2/gli-v0.1.2-x86_64-apple-darwin.tar.gz
tar -xvf gli-v0.1.2-x86_64-apple-darwin.tar.gz

# Linux
wget https://github.com/imadelh/Chat-CLI/releases/download/v0.1.2/gli-v0.1.2-x86_64-unknown-linux-gnu.tar.gz
tar -xvf gli-v0.1.2-x86_64-unknown-linux-gnu.tar.gz
```

### Docker

WiP

## Usage 

```bash
./gli -h 

A simple CLI for ChatGPT.
Requires OPENAI_TOKEN env variable when using OpenAI endpoint.

USAGE:
    gli [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --endpoint <endpoint>          ChatGPT API endpoint URL. [default:
                                       https://api.openai.com/v1/chat/completions]
        --output <output>              Output file path [default: chat_outputs]
        --temperature <temperature>    Temperature (between 0 and 1) [default: 1.0]

```

- Using OpenAI endpoint

```bash
export OPENAI_TOKEN=<put your key here - https://platform.openai.com/account/api-keys>

./gli --output 'my_chat'
```

- Using free endpoint [ChatGPTAPIFree](https://github.com/ayaka14732/ChatGPTAPIFree)
```
# Token is not required
./gli --endpoint 'https://chatgpt-api.shn.hk/v1/'
```

## Dev

```
cargo run -- -h
cargo build --release
```

Contributions are welcome.

TODO: 
- Add more error messages
- Support multiple models
- Continue from an existing conversation (from a file)
- Add tests


----------
