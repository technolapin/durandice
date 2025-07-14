use std::env::args;
use rlisp::Lisp;
use serenity::{
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use rusqlite::{Connection, Result};
use chrono::Utc;
use std::convert::From;

#[macro_use] extern crate lalrpop_util;
lalrpop_mod!(pub parser);

use serenity::model::event::MessageUpdateEvent;

/*
struct LastCommand;

impl TypeMapKey for LastCommand
{
    type Value = Arc<RwLock<HashMap<String, String>>>;
}

struct Database;

impl TypeMapKey for Database
{
    type Value = Arc<Connection>;
}
*/

enum BotOutput
{
    Text(String),
    Image{
        url: String,
        maybe_text: Option<String>
    },
    Nothing
}

struct Handler{}

impl Handler
{
    fn new() -> Self
    {
        Self{}
    }
}

fn process_message(ctx: Context, msg: Message)
{
    if msg.content.starts_with("?")
    {
        let realname = String::from(&msg.author.name);
        let username = if let Some(member) = msg.member(&ctx.cache)
        {
            if let Some(nick) = member.nick
            {
                nick.clone()
            }
            else
            {
                msg.author.name.clone()
            }
        }
        else
        {
            msg.author.name.clone()
        };
        
        let conn = Connection::open("database.db").unwrap();

	let mut text = String::from(&msg.content[1..].to_ascii_lowercase());


        println!(r#"ENTRY: {} entered "{}""#, realname, text);

        let out = parse(text, username, realname, &conn);

        match out
        {
            Err(err) =>
            {
                let content = format!("Error: {:?}", err);
                println!("Error: {:?}", err);
                if false
                {
                    if let Err(why) = msg.reply(&ctx.http, &content)
	            {
                        println!("Error sending error message {:?} ({:?})", content, why);
                    }
                }

            },
            Ok(answer) => match answer
            {
                BotOutput::Nothing =>
                {
                    println!("Bot output: No output");
                },
                BotOutput::Text(content) =>
                {
                    if let Err(why) = msg.reply(&ctx.http, &content)
	            {
                        println!("Error sending message: {:?}", why);
                    }
                    else
                    {
                        println!(r#"Bot output: Text "{}""#, content);
                    }
                },
                BotOutput::Image{url, maybe_text} =>
                {
                    if let Err(why) = msg.channel_id
                        .send_files(&ctx.http,
                                    vec![url.as_str()],
                                    |m| {  if let Some(text) = maybe_text.clone()
                                           {m.content(&text)}
                                           else
                                           {m.content("")} })
	            {
                        println!("Error sending message: {:?}", why);
                    }
                    else
                    {
                        println!("Bot output: Image (url: {}, text: {:?})", url, maybe_text);
                    }
                }
            }
            
        }
        /*
        if let Err(why) = msg.channel_id.say(&ctx.http, &out)
	{
        println!("Error sending message: {:?}", why);
    }
         */

        
    }

}


impl EventHandler for Handler
{
    fn message(&self, ctx: Context, msg: Message)
    {
        process_message(ctx, msg)
    }

    fn message_update(
    &self,
    ctx: Context,
    _old_if_available: Option<Message>,
    new: Option<Message>,
    event: MessageUpdateEvent
    )
    {
        println!("MESSAGE UPDATE: {:?}", event);
        if let Some(msg) = new
        {
            process_message(ctx, msg);
        }
    }
    
    fn ready(&self, _: Context, ready: Ready)
    {
        println!("{} is connected!", ready.user.name);
    }
}

fn parse(text: String,
         username: String,
         realname: String,
         conn: &Connection) -> Result<BotOutput, Error>
{
    let maybe_first_word = text.split_whitespace().nth(0);

    // do nothing if no non-whitespace char
    if maybe_first_word.is_none()
    {
        return Ok(BotOutput::Nothing);
    }
    // safe to unwrap
    let first_word = maybe_first_word.unwrap();

    match first_word
    {
        "onebi" =>
        {
            match rand::random::<u32>() % 2 +1
            {
                0 => Ok(BotOutput::Text(format!("<@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> <@!243288980039270402> "))),
                
                1 => Ok(BotOutput::Image
                        {
                            url: String::from("assets/tehc.png"),
                            maybe_text: None
                        }),
                _ => Ok(BotOutput::Image
                        {
                            url: String::from("assets/dorian.png"),
                            maybe_text: None
                        }),
                
            }
            
        },
        "aria" =>
        {
            Ok(BotOutput::Image
               {
                   url: String::from("assets/cheems.jpg"),
                   maybe_text: None
               })
            
        },
        "re" =>
        {
            let name = text.split_whitespace().nth(1).unwrap_or("__last_entry");

            let mut stmt = conn.prepare(
                "SELECT username, command_name, command, date FROM registered_commands
                     WHERE username=(?1) AND command_name=(?2)
                     ORDER BY date DESC;"
            ).unwrap();

            let cmd: Option<String> = stmt.query_map(&[&realname, name], |row| {
                row.get(2)
            }).unwrap().filter_map(|result| result.ok()).next();

            if let Some(ancient) = cmd
            {
                parse(ancient, username, realname, conn)
            }
            else
            {
                Ok(BotOutput::Text(format!("No previous entry for {}", realname)))
            }

        },
        "haha" =>
        {
            Ok(BotOutput::Image{
                url: String::from("assets/haha_nelson.jpg"),
                maybe_text: Some(format!("Haha!"))
            })
        },
        "emig" =>
        {
            Ok(BotOutput::Image{
                url: String::from("assets/emig.png"),
                maybe_text: None
            })
        },
        "list" =>
        {
            let mut stmt = conn.prepare(
                "SELECT username, command_name, command, date FROM registered_commands
                     WHERE username=(?1)
                     ORDER BY date DESC;"
            )?;
            let entries = stmt.query_map(&[&realname], |row| {
                Ok((row.get(1)?, row.get(2)?))
            })?
                .filter_map(|result| result.ok())
                .filter_map(|option: (Option<String>, Option<String>)|
                            match option
                            {
                                (Some(name), Some(code)) => Some(format!("{}: {}", name, code)),
                                _ => None
                            })
                .filter_map(|entry| if entry.chars().next() == Some('_') {None} else {Some(entry)})
                .collect::<Vec<_>>();
            Ok(BotOutput::Text(format!("```{}```", if entries.len() > 0
                               {
                                   entries
                                       .into_iter()
                                       .fold(String::new(), |s, entry| format!("{}{}\n", s, entry))
                               }
                               else
                               {
                                   format!("Pas de macros enregistrées pour {}", realname)
                               })))
                
        },
        "forget" =>
        {
            let name = text.split_whitespace().nth(1)
                .ok_or(Error::new("bad unwrap 1"))?;
            conn.execute(
                "DELETE FROM registered_commands WHERE username = (?1) AND command_name = (?2);",
                &[&realname, name],
            )?;
            
            Ok(BotOutput::Text(format!("Commande {} de {} supprimée", name, realname)))
                

        },
        "register" =>
        {
            println!("REGISTER");
            let code = text[8..].chars()
                .skip_while(|c| c.is_whitespace())
                .skip_while(|c| !c.is_whitespace())
                .collect::<String>();

            let name = text.split_whitespace().nth(1)
                .ok_or(Error::new("bad unwrap 2"))?;
            let date = Utc::now().timestamp();
            conn.execute(
                "INSERT INTO registered_commands (username, command_name, command, date)
                     values (?1, ?2, ?3, ?4)",
                &[&realname, name, &code, &format!("{}", date)],
            )?;
            Ok(BotOutput::Text(format!("Commande enregistrée pour {} en tant que '{}'", realname, name)))
                
        },
        // commands that can be registered and such
        other =>
        {
            let date = Utc::now().timestamp();
            conn.execute(
                "INSERT INTO registered_commands (username, command_name, command, date)
                     values (?1, ?2, ?3, ?4)",
                &[&realname, "__last_entry", &text, &format!("{}", date)],
            )?;

            match other
            {
                "lisp" =>
                {
                    let code = &(&text)[4..];
                    let mut lisp = Lisp::new();
                    let out = lisp.evaluate(code);
                    Ok(BotOutput::Text(
                        match out
                        {
                            Ok(result) =>
                            {
                                format!("```Markdown\n{}\n```", result)
                            },
                            Err(err) => format!("```Markdown\n{:?}\n```", err)
                        }))
                },
                _ =>
                {
                    let result = Error::convert_result(parser::CmdListParser::new().parse(&text))?.execute();
                    Ok(BotOutput::Text(format!("```Markdown\n{}\n```", result)))

                }
            }
        }
    }
    
}


#[derive(Debug, Clone)]
struct Error(String);

impl Error
{
    pub fn new(msg: &str) -> Self
    {
        Self(String::from(msg))
    }

    pub fn from_other<E: std::fmt::Debug>(err: E) -> Self
    {
        Self(format!("{:?}", err))
    }

    pub fn convert_result<T, E: std::fmt::Debug>(res: Result<T, E>) -> Result<T, Self>
    {
        match res
        {
            Ok(thing) => Ok(thing),
            Err(err) => Err(Self::from_other(err))
        }
    }
}



macro_rules! impl_from {
    ($t:path) => {
        impl From<$t> for Error
        {
            fn from(err: $t) -> Self
            {
                Self(format!("{}", err))
            }
        }
    };
}

impl_from!(rusqlite::Error);

fn main() -> Result<(), Error>
{
    let arguments = args().collect::<Vec<_>>();
    if arguments.len() != 2
    {
        panic!("REQUIRE ONE ARGUMENT (the bot token plz)");
    }
    
    let token = &arguments[1];

    let mut client = Client::new(&token, Handler::new()).expect("Err creating client");

    {
        let conn = Connection::open("database.db").unwrap();

        conn.execute(
            "create table if not exists registered_commands (
             id integer primary key,
             username text not null,
             command_name text not null,
             command text not null,
             date integer
             )",
            (),
        ).unwrap();
        
//        data.insert::<Database>(conn.clone());
    }
    
    if let Err(why) = client.start() {
        println!("Clien error: {:?}", why);
    }
    Ok(())
}
