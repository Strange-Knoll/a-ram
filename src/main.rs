#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(unused_allocation)]

use a_ram::*;

mod style;
use style::*;
use crossterm::style::Color;

use std::{error::Error, fs, io::Write};

use async_openai::{Client, types::{CreateChatCompletionRequestArgs, ChatCompletionRequestMessageArgs, Role,}, config::OpenAIConfig};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    const KEY:&str = "Your API Key";

    let mut memory = Memory::new(MemorySize::Large);
    let stat_sect = memory.add_static_sector("you are a general purpose chat bot.")?;
    let dyn_sect = memory.add_dynamic_sector(128)?;

    loop{

        let mut query = String::new();
        std::io::stdin().read_line(&mut query)?;
        query = format!("\nUSER QUERY: \n{}\n\n", query);
        memory.alloc(dyn_sect, &query)?;

        let request = CreateChatCompletionRequestArgs::default()
        .max_tokens(2048u16)
        .model("gpt-3.5-turbo-16k")     // make a variable for this that matches the memory model types
        .messages([
            ChatCompletionRequestMessageArgs::default()
                .role(Role::System)
                .content(Token::detokenize(memory.get_static_sector(stat_sect)?.data.clone()).as_str())
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::Assistant)
                .content("")
                .build()?,
            ChatCompletionRequestMessageArgs::default()
                .role(Role::User)
                .content(&query)
                .build()?,
        ]).build()?;

        let config = OpenAIConfig::new().with_api_key(KEY);
        let client = Client::with_config(config);

        let response = client.chat().create(request).await?;
        let response = &response.choices[0].message.content.as_ref().unwrap();
        let response = format!("BOT RESPONSE: \n{}\n\n", response);

        memory.alloc(dyn_sect, &response)?;

        println!("{}", response.clone());

        //styled_println(Color::Cyan, Color::Reset, &memory.to_string())?;
        styled_println(Color::Cyan, Color::Reset, format!("MEMORY DUMP: \n{}", memory.to_string()).as_str())?;
        styled_println(Color::Magenta, Color::Reset, Token::tokenize(&memory.to_string()).len().to_string().as_str())?;
    }

    Ok(())

}