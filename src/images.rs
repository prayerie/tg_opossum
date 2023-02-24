use std::fs::{self, DirEntry};

use rand::seq::SliceRandom;
use teloxide::types::{InputFile, InputMedia};

use crate::{db, util::parse_some_num};

fn iter_files(path: String) -> Vec<DirEntry> {
    let paths = fs::read_dir(path).unwrap();
    let mut vec: Vec<DirEntry> = Vec::new();
    for path in paths.into_iter() {
        match path {
            Ok(p) => {
                vec.push(p);
            }
            Err(_) => {}
        }
    }

    return vec;
}

pub fn random_file_in(path: String) -> Option<(InputFile, i32)> {
    let all_paths = iter_files(path);
    let banned = db::get_banned();
    return if all_paths.is_empty() {
        None
    } else {
        match all_paths.choose(&mut rand::thread_rng()) {
            Some(chosen_path) => {
                let mut chosen_path_final = chosen_path;
                loop {
                    let img =
                        parse_some_num(chosen_path_final.file_name().to_string_lossy().to_string());
                    let img_p = img.parse::<i32>().unwrap_or(0);
                    if banned.contains(&img_p) {
                        chosen_path_final = all_paths.choose(&mut rand::thread_rng()).unwrap();
                    } else {
                        return Some((InputFile::file(chosen_path_final.path()), img_p));
                    }
                }
            }
            None => None,
        }
    };
}

pub fn specific_file_in(path: String, img_id: String) -> Option<InputFile> {
    let all_paths = iter_files(path);
    let banned = db::get_banned();
    println!("requested image {}", img_id);
    return if all_paths.is_empty() {
        println!("empty what");
        None
    } else {
        match all_paths.iter().find(|d| d.path().to_string_lossy().to_string().contains(&img_id)) {
            Some(chosen_path) => {
                let chosen_path_final = chosen_path;
                let img =
                    parse_some_num(chosen_path_final.file_name().to_string_lossy().to_string());
                let img_p = img.parse::<i32>().unwrap();
                println!("found with int {}", img_p);
                if banned.contains(&img_p) {
                    return None;
                } else {
                    println!("sending now {:?}", chosen_path_final.path().to_str());
                    return Some(InputFile::file(chosen_path_final.path()));
                }
            }
            None => None,
        }
    };
}