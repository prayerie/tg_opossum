use rand::Rng;
use teloxide::{Bot, requests::{Requester, ResponseResult}, types::ChatId};

use crate::config::Config;

pub fn parse_some_num(s: String) -> String {
    return s
        .chars()
        .take_while(|c| c.is_ascii_digit())
        .collect::<String>();
}

pub fn hiss_text() -> String {
    let num_s = rand::thread_rng().gen_range(1..8);
    let chance_caps = rand::thread_rng().gen_range(0..11);
    let mut s: String = "his".to_string();

    for _ in 0..num_s {
        s.push('s')
    }

    if chance_caps >= 6 {
      s = s.to_uppercase();
    }

    s
}

pub async fn log_error(bot: Bot, error: String, cfg: Config) -> ResponseResult<()> {
  log::error!("{}", error);
  bot.send_message(ChatId { 0: cfg.admin_chat }, error).await?;
  Ok(())
}
