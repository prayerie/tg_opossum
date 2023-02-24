use std::{sync::Arc, fmt};

use teloxide::{
    prelude::*,
    types::{ChatAction, ParseMode},
    utils::command::BotCommands,
    Bot,
};

use crate::{
    config::Config,
    db::{self},
    images::{self, specific_file_in},
    util,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
pub enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "Send a (random) opossum. Can optionally specify image number.")]
    Opossum,
    #[command(description = "Send a (random) hiss. Can optionally specify image number.")]
    Hiss,
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Admin commands:")]
pub enum AdminCommand {
    #[command(description = "Ban an image.")]
    Ban,
    #[command(description = "Get list of banned images.")]
    Get_Banned,
    #[command(description = "do something magical")]
    Magic,
}

pub async fn answer(bot: Bot, msg: Message, cmd: Command, cfg: Arc<Config>) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Opossum => {
            let text = msg.text().unwrap();
            let args: Vec<&str> = text.split(" ").collect();

            if args.len() == 1 {
                return random_opossum(bot, msg, cfg).await;
            } else {
                match args[1].parse::<u16>() {
                    Ok(image) => {
                        match specific_file_in((&cfg.dirs.opossum).to_string(), image.to_string()) {
                            Some(inpf) => {
                                db::insert_hist(
                                    msg.chat.id.0,
                                    msg.chat.username(),
                                    true,
                                    image.into(),
                                    true,
                                    msg.chat.is_private(),
                                );
                                bot.send_photo(msg.chat.id, inpf)
                                    .reply_to_message_id(msg.id)
                                    .await?
                            }
                            None => {
                                bot.send_message(
                                    msg.chat.id,
                                    format!("Opossum image number `{}` does not exist\\.", image),
                                )
                                .parse_mode(ParseMode::MarkdownV2)
                                .reply_to_message_id(msg.id)
                                .await?
                            }
                        }
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Come again?\n\"`{}`\"", e))
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_to_message_id(msg.id)
                            .await?
                    }
                }
            }
        }
        Command::Hiss => {
            let text = msg.text().unwrap();
            let args: Vec<&str> = text.split(" ").collect();

            if args.len() == 1 {
                return random_hiss(bot, msg, cfg).await;
            } else {
                match args[1].parse::<u16>() {
                    Ok(image) => {
                        match specific_file_in((&cfg.dirs.opossum).to_string(), image.to_string()) {
                            Some(inpf) => {
                                db::insert_hist(
                                    msg.chat.id.0,
                                    msg.chat.username(),
                                    true,
                                    image.into(),
                                    true,
                                    msg.chat.is_private(),
                                );
                                bot.send_photo(msg.chat.id, inpf)
                                    .reply_to_message_id(msg.id)
                                    .caption(util::hiss_text())
                                    .await?
                            }
                            None => {
                                bot.send_message(
                                    msg.chat.id,
                                    format!("Hiss image number `{}` does not exist.", image),
                                )
                                .await?
                            }
                        }
                    }
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Come again?\n\"`{}`\"", e))
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_to_message_id(msg.id)
                            .await?
                    }
                }
            }
        }
    };

    Ok(())
}

pub async fn answer_admin(
    bot: Bot,
    msg: Message,
    cmd: AdminCommand,
    cfg: Arc<Config>,
) -> ResponseResult<()> {
    match cmd {
        AdminCommand::Ban => {
            let text = msg.text().unwrap();
            let args: Vec<&str> = text.split(" ").collect();

            if args.len() == 1 {
                bot.send_message(msg.chat.id, format!("You need to specify an image to ban."))
                    .reply_to_message_id(msg.id)
                    .await?
            } else {
                match args[1].parse::<u16>() {
                    Ok(image) => match db::put_banned(image.into()) {
                        true => {
                            let m = bot
                                .send_message(msg.chat.id, format!("Banned image {image} !"))
                                .reply_to_message_id(msg.id)
                                .await?;
                            println!("{:#?}", db::get_banned());
                            m
                        }
                        false => {
                            bot.send_message(msg.chat.id, format!("Database error 😟"))
                                .reply_to_message_id(msg.id)
                                .await?
                        }
                    },
                    Err(e) => {
                        bot.send_message(msg.chat.id, format!("Come again?\n\"`{}`\"", e))
                            .parse_mode(ParseMode::MarkdownV2)
                            .reply_to_message_id(msg.id)
                            .await?
                    }
                }
            }
        }
        AdminCommand::Get_Banned => {
            let banned = db::get_banned();
            let mut banned_s: Vec<String> = vec![];
            for b in banned {
                banned_s.push(b.to_string());
            }
            let banned_joined = banned_s.join(", ");
            bot.send_message(
                msg.chat.id,
                format!("The following images are banned: \n\"`{}`\"", banned_joined),
            )
            .parse_mode(ParseMode::MarkdownV2)
            .reply_to_message_id(msg.id)
            .await?
        }
        AdminCommand::Magic => return random_special(bot, msg, cfg).await,
    };

    Ok(())
}

// this is all boilerplate which could be reduced maybe

pub async fn random_opossum(bot: Bot, msg: Message, cfg: Arc<Config>) -> ResponseResult<()> {
    let dir = &cfg.dirs.opossum;
    bot.send_chat_action(msg.chat.id, ChatAction::UploadPhoto)
        .await?;
    match images::random_file_in(dir.to_string()) {
        Some(file) => {
            db::insert_hist(
                msg.chat.id.0,
                msg.chat.username(),
                false,
                file.1,
                false,
                msg.chat.is_private(),
            );
            bot.send_photo(msg.chat.id, file.0)
            .caption(format!("{}", file.1))
            .await?;
        }
        None => {}
    }

    Ok(())
}

pub async fn random_hiss(bot: Bot, msg: Message, cfg: Arc<Config>) -> ResponseResult<()> {
    let dir = &cfg.dirs.hiss;
    bot.send_chat_action(msg.chat.id, ChatAction::UploadPhoto)
        .await?;
    match images::random_file_in(dir.to_string()) {
        Some(file) => {
            db::insert_hist(
                msg.chat.id.0,
                msg.chat.username(),
                true,
                file.1,
                false,
                msg.chat.is_private(),
            );
            bot.send_photo(msg.chat.id, file.0)
                .caption(util::hiss_text())
                .await?;
        }
        None => {}
    }

    Ok(())
}

pub async fn random_special(bot: Bot, msg: Message, cfg: Arc<Config>) -> ResponseResult<()> {
    let dir = &cfg.dirs.special;
    bot.send_chat_action(msg.chat.id, ChatAction::UploadPhoto)
        .await?;
    match images::random_file_in(dir.to_string()) {
        Some(file) => {
            db::insert_hist(
                msg.chat.id.0,
                msg.chat.username(),
                false,
                file.1,
                false,
                msg.chat.is_private(),
            );
            bot.send_photo(msg.chat.id, file.0)
                .reply_to_message_id(msg.id)
                .await?;
        }
        None => {}
    }

    Ok(())
}

pub async fn shh(bot: Bot, msg: Message) -> ResponseResult<()> {
    bot.send_message(msg.chat.id, "shh")
        .reply_to_message_id(msg.id)
        .await?;
    Ok(())
}
