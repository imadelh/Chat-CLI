use std::collections::VecDeque;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};
use structopt::clap::AppSettings;
use structopt::StructOpt;
use termion::color;

use gli::chat::{ChatGPTCall, ChatGPTMessage, OPENAI_DEFAULT_ENDPOINT};
#[derive(StructOpt)]
#[structopt(
    name = "chatgpt-cli",
    about = "
    A simple CLI for ChatGPT.
    Requires OPENAI_TOKEN env variable when using OpenAI endpoint.
    ",
    global_settings = &[AppSettings::ColoredHelp, AppSettings::GlobalVersion]
)]
struct Cli {
    #[structopt(
        long,
        help = "ChatGPT API endpoint URL.",
        default_value = OPENAI_DEFAULT_ENDPOINT
    )]
    endpoint: String,
    #[structopt(long, help = "Temperature (between 0 and 1)", default_value = "1.0")]
    temperature: f32,
    #[structopt(long, help = "Output file path", default_value = "chat_outputs")]
    output: String,
}
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    // Initialize conversation that would keep the last 5 interactions for context
    let vec_messages: VecDeque<ChatGPTMessage> = VecDeque::with_capacity(5);
    let mut conversation = ChatGPTCall {
        messages: vec_messages,
        model: "gpt-3.5-turbo".to_string(), // TODO: add to ChatGPTCall
        temperature: args.temperature,
    };
    let token = env::var("OPENAI_TOKEN");

    // Initialize a text file to save the conversation
    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    // Output file
    // Check if the parent directory of the file exists
    let file_name =
        args.output + &"_".to_string() + since_the_epoch.to_string().as_str() + &".txt".to_string();

    if let Some(parent_dir) = std::path::Path::new(&file_name).parent() {
        if !parent_dir.exists() {
            // Create the parent directory if it doesn't exist
            std::fs::create_dir_all(parent_dir).unwrap();
        }
    }

    let file = File::create(Path::new(&file_name))?;

    println!(
        "{}=== Conversation saved at: {} === \nType 'exit' to exit.
        ",
        color::Fg(color::LightBlue),
        &file_name,
    );
    println!("{}", color::Fg(color::Reset));

    loop {
        let input = read_input("\n>User: ")?;
        if input == "exit".to_string() {
            process::exit(1);
        }
        let one_message = ChatGPTMessage {
            role: "user".to_string(),
            content: input,
        };

        // Save the message to the conversation file
        let _saved = one_message.save(&file);

        // Add the message to the conversation
        conversation.push(one_message);

        // Call ChatGPT API
        let content_response = conversation.api_call(&args.endpoint, &token)?;

        println!(
            "{}\n>Assistant:\n\n{}",
            color::Fg(color::LightGreen),
            content_response
        );

        // Add the reponse to the conversation
        let message = ChatGPTMessage {
            role: "assistant".to_string(),
            content: content_response.to_string(),
        };
        println!("{}", color::Fg(color::Reset));

        let _saved = message.save(&file);

        conversation.push(message);
    }
}

fn read_input(prompt: &str) -> Result<String, std::io::Error> {
    print!("{}", prompt);
    std::io::stdout().flush()?;
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_owned())
}
