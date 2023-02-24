mod config;
mod handlers;
mod filters;
mod images;
mod db;
mod util;
pub mod models;
pub mod schema;
use teloxide::{prelude::*, dptree::filter};
use std::{sync::Arc};
use crate::{handlers::{Command, AdminCommand}};

pub fn filter_true() -> bool {
    return true;
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");
    match config::get_cfg() {
        Ok(config) => {
            let bot = teloxide::Bot::new(config.tokens.token.to_string());
            let mut container = DependencyMap::new();
            container.insert(Arc::new(config));
            let handler = dptree::entry()
            .branch(Update::filter_message().filter_command::<Command>().endpoint(handlers::answer))
            .branch(Update::filter_message().filter_command::<AdminCommand>().endpoint(handlers::answer_admin))
            .branch(Update::filter_message()
                .branch(filter(filters::is_admin)
                        .filter(filters::match_special)
                        .endpoint(handlers::random_special))
                    
                    
                .branch(filter(filters::not_too_recent).filter(filters::is_public)
                    .branch(filter(filters::match_opossum).endpoint(handlers::random_opossum))
                    .branch(filter(filters::match_hiss).endpoint(handlers::random_hiss)))
                .branch(filter(filters::is_private)
                    .branch(filter(filters::match_opossum).endpoint(handlers::random_opossum))
                    .branch(filter(filters::match_hiss).endpoint(handlers::random_hiss)))
                .branch(filter(filters::too_recent).filter(filters::match_hiss).endpoint(handlers::shh))
                .branch(filter(filters::too_recent).filter(filters::match_opossum).endpoint(handlers::shh)));
            

            Dispatcher::builder(bot, handler).dependencies(container).enable_ctrlc_handler().build().dispatch().await;
        },
        Err(_) => {
            log::error!("Failed to get config. Terminating.");
            return;
        }
    }
        
}







