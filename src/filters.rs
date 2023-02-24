// // use crate::{config::{
// //     counters::{Counter, CounterEvent},
// //     Command, Config,
// // }, checks::*};
// // use crate::db::*;
// // use once_cell::sync::Lazy;
// use regex::Regex;
// // use std::sync::atomic::{AtomicU16, Ordering};
// // use std::sync::Arc;
// use teloxide::prelude::*;

// // static MESSAGES_COUNT: Lazy<AtomicU16> = Lazy::new(AtomicU16::default);

// pub fn not_blacklisted(msg: Message, blacklist: Vec<u64>) -> bool {
//     !is_blacklisted(&msg, &blacklist)
// }

// pub fn not_outdated(msg: Message) -> bool {
//     !is_outdated(&msg)
// }

// pub fn not_forwarded(msg: Message) -> bool {
//     msg.forward() == None
// }

// pub fn reply_to_bot(msg: Message, bot_id: u64) -> bool {
//     is_reply_to_bot(&msg, &bot_id)
// }

// pub fn reply_not_to_bot(msg: Message, bot_id: u64) -> bool {
//     if msg.reply_to_message().is_none() {
//         return false;
//     }
//     if is_reply_to_bot(&msg, &bot_id) {
//         return false;
//     }
//     true
// }

// pub fn matches_substitution(msg: Message, config: Arc<Config>) -> bool {
//     if msg.reply_to_message().is_none() {
//         return false;
//     }
//     check_regex(&msg, &config.regex.substitution)
// }

// pub fn fuck_you(msg: Message, config: Arc<Config>) -> bool {
//     let regex = &config.regex.fuck_you;
//     match (msg.text(), msg.voice()) {
//         (Some(text), _) => regex.is_match(text),
//         (_, Some(_)) => (msg.id * 32) % 20 > 3,
//         _ => false,
//     }
// }

// pub fn answer(msg: Message, config: Arc<Config>) -> bool {
//     let regex = &config.regex.answer;
//     if msg.chat.is_private() {
//         return true;
//     }
//     if let Some(text) = &msg.text() {
//         return regex.is_match(text);
//     }
//     false
// }

// pub fn question_counter(msg: Message, config: Arc<Config>) -> bool {
//     let question_delay = config.question_delay;
//     if !msg.chat.is_group() && !msg.chat.is_supergroup() {
//         return false;
//     }
//     if MESSAGES_COUNT.fetch_add(1, Ordering::Relaxed) >= question_delay {
//         MESSAGES_COUNT.store(msg.id as u16 % (question_delay / 2), Ordering::Relaxed);
//         return true;
//     }
//     false
// }

// pub fn matches_expand(msg: Message, config: Arc<Config>) -> bool {
//     let text = msg.text().unwrap();
//     for expand in &config.expands {
//         if expand.regex.is_match(text) {
//             return true;
//         }
//     }
//     false
// }

// pub fn matches_reply(msg: Message, config: Arc<Config>) -> bool {
//     let text = msg.text().unwrap();
//     for reply in &config.replies {
//         if reply.regex.is_match(text) {
//             return true;
//         }
//     }
//     false
// }

// pub fn is_rudelisted(msg: Message, config: Arc<Config>) -> bool {
//     if msg.id * 32 % 81 > 65 {
//         return false;
//     }
//     let user = match msg.from() {
//         Some(user) => user,
//         None => return false,
//     };
//     let chat_id = msg.chat.id;
//     match open_db_connection(&config.db.path) {
//         Ok(connection) => {
//             if let Err(err) = clean_rudelisted(&connection) {
//                 log::error!("Cannot clean rudelist\n{}", err);
//                 return false;
//             }
//             let rudelisted = match get_rudelisted(&connection) {
//                 Ok(rudelisted) => rudelisted,
//                 Err(err) => {
//                     log::error!("Cannot get rudelisted from db\n{}", err);
//                     return false;
//                 }
//             };
//             crate::checks::is_rudelisted(user, chat_id.0, &rudelisted)
//         }
//         Err(err) => {
//             log::error!("Cannot open db connection\n{}", err);
//             false
//         }
//     }
// }

// pub fn process_member(msg: Message, config: Arc<Config>) -> bool {
//     let user = match msg.from() {
//         Some(user) => user,
//         None => return true,
//     };
//     with_db_connection(&config.db.path, |connection| {
//         let credentials_equals =
//             |m: &Member| m.credentials_equals(&user.username, &user.first_name, &user.last_name);
//         let update_credentials = |m: &Member| {
//             Member::new(
//                 m.id,
//                 m.chat_id,
//                 user.username.clone(),
//                 user.first_name.clone(),
//                 user.last_name.clone(),
//             )
//         };
//         let members = clean_members(&connection, msg.chat.id.0)?;
//         let current_member = members.iter().map(|(m, _)| m).find(|m| m.id == user.id.0);
//         let res = match current_member {
//             None => add_member(
//                 &connection,
//                 match &Member::from_message(&msg) {
//                     Some(member) => member,
//                     None => return Ok(false),
//                 },
//             ),
//             Some(m) => {
//                 let mut m = m.clone();
//                 if !credentials_equals(&m) {
//                     m = update_credentials(&mut m);
//                 }
//                 update_member(&connection, &m)
//             }
//         };
//         if let Err(e) = res {
//             log::error!("Cannot process member\n{}", e);
//             return Ok(false);
//         }
//         Ok(true)
//     })
//     .unwrap_or(false)
// }

// pub fn kick_trigger(msg: Message, config: Arc<Config>) -> bool {
//     if msg.forward().is_some() {
//         return false;
//     }
//     for r in &config.regex.kick {
//         if check_regex(&msg, r) {
//             return true;
//         }
//     }
//     false
// }

// pub fn is_command(msg: Message, config: Arc<Config>) -> Option<Arc<Command>> {
//     let text = msg.text()?;
//     for (k, v) in &config.commands {
//         if v.is_match(text) {
//             return Some(Arc::new(k.clone()));
//         }
//     }
//     None
// }

// pub fn matches_counter(msg: Message, config: Arc<Config>) -> Option<Arc<(CounterEvent, Counter)>> {
//     let text = msg.text()?;
//     for c in &config.counters {
//         if !c.matches_type(&msg) {
//             continue;
//         }
//         for (e, r) in &c.regex {
//             if r.is_match(text) {
//                 return Some(Arc::new((e.clone(), c.clone())));
//             }
//         }
//     }
//     None
// }

// fn check_regex(msg: &Message, regex: &Regex) -> bool {
//     match msg.text() {
//         Some(text) => regex.is_match(text),
//         None => false,
//     }
// }

use std::sync::Arc;

use teloxide::types::{Message, User};

use crate::{config::Config, db::get_delta_for_chat};

pub fn is_admin(msg: Message, cfg: Arc<Config>) -> bool {
    match msg.from() {
        None => false,
        Some(usr) => cfg.admins.contains(&usr.id.0)
    }
}

pub fn is_private(msg: Message) -> bool {
    return msg.chat.is_private()
}

pub fn is_public(msg: Message) -> bool {
    return !is_private(msg)
}

pub fn not_too_recent(msg: Message, cfg: Arc<Config>) -> bool {
    return get_delta_for_chat(msg.chat.id.0) >= cfg.rate_limit_secs
}

pub fn too_recent(msg: Message, cfg: Arc<Config>) -> bool {
    return !not_too_recent(msg, cfg);
}

pub fn match_opossum(msg: Message, cfg: Arc<Config>) -> bool {
    return match msg.text() {
        Some(content) => {
            let (mut match_1, mut match_2) = (false, false);
            let ct = content.to_lowercase();
            for word in &cfg.triggers.first_trigger {
                if ct.contains(&word.to_lowercase().as_str()) {
                    match_1 = true;
                    break;
                }
            }

            for word in &cfg.triggers.second_trigger {
                if ct.contains(&word.to_lowercase().as_str()) {
                    match_2 = true;
                    break;
                }
            }

            match_1 && match_2
        },
        None => false
    }
}

pub fn match_hiss(msg: Message, cfg: Arc<Config>) -> bool {
    return match msg.text() {
        Some(content) => {
            let mut match_1 = false;
            let ct = content.to_lowercase();
            let words_vec: Vec<&str> = ct.split(" ").collect();
            for word in &cfg.triggers.hiss.first_trigger {
                if words_vec.contains(&word.to_lowercase().as_str()) {
                    match_1 = true;
                    break;
                }
            }

            match_1
        },
        None => false
    }
}

pub fn match_special(msg: Message, cfg: Arc<Config>) -> bool {
    return match msg.text() {
        Some(content) => {
            let mut match_1 = false;
            for phrase in &cfg.triggers.special.first_trigger {
                if content.to_lowercase().contains(phrase) {
                    match_1 = true;
                    break;
                }
            }

            match_1
        },
        None => false
    }
}