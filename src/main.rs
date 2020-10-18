use std::env::args;



use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);


struct Handler;

impl EventHandler for Handler
{
    fn message(&self, ctx: Context, msg: Message) {
        
        if msg.content.starts_with("?")
	{
	    let text = msg.content.get(1..).unwrap();
            let username = msg.author.name;
            let out = match parser::CmdParser::new().parse(text)
            {
                Ok(cmd) =>
                {
                    let result = cmd.execute();
                    format!("```Markdown\n# {}\n{}\n```", username, result)
                },
                Err(err) =>
                {
                    format!("syntax error ({})", err)
                }
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, &out)
	    {
                println!("Error sending message: {:?}", why);
            }

        }
    }

    fn ready(&self, _: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);
    }
}

fn main()
{
    let arguments = args().collect::<Vec<_>>();
    if arguments.len() != 2
    {
        panic!("REQUIRE ONE ARGUMENT (the bot token plz)");
    }
    
    let token = &arguments[1];

    let mut client = Client::new(&token, Handler).expect("Err creating client");

    if let Err(why) = client.start() {
        println!("Clien error: {:?}", why);
    }
}
