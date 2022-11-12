extern crate dotenv_codegen;

use std::fs::File;
//use std::collections::HashMap;
//use std::ops::ControlFlow;
//use reqwest::blocking::Client;
//use std::io;
use reqwest;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::io::prelude::*;

fn main() {
    let api_key: String = get_key();

    let channel_id: String = format!("pewdiepie");

    let playlist_id = get_channel_upload_playlist(&channel_id, &api_key)
        .expect("Failed to find upload playlist id.");

    //println!("{:#?}", playlist_id);

    get_playlist_videos(&playlist_id, &api_key);

    //println!("Hello, world!");
    //let channel_id = "pewdiepie";
    //let channel_name = "pewdiepie";

    //println!("Main Key: {}", &playlist_id);
    //println!("{}", api_key);
}

fn get_key() -> String {
    let mut file = File::open("src/keyid.txt").expect("Can't open File");

    let mut main_key = String::new();

    file.read_to_string(&mut main_key)
        .expect("can not read the file");

    main_key
}

fn get_channel_upload_playlist(channel_id: &String, api_key: &String) -> Option<String> {
    let channel_url:String = format!("https://youtube.googleapis.com/youtube/v3/channels?part=contentDetails&forUsername={}&key={}", channel_id, api_key);

    let channel_json = get_channel_json(&channel_url)?;

    //println!("{:#?}", channel_json);

    Some(String::from(
        channel_json["items"][0]["contentDetails"]["relatedPlaylists"]["uploads"].as_str()?,
    ))
}
// https://youtube.googleapis.com/youtube/v3/channels?part=contentDetails&forUsername=pewdiepie&key=[YOUR_API_KEY]
// https://youtube.googleapis.com/youtube/v3/channels?part=contentDetails&forUsername=AlecBenjaminMusic&key=[YOUR_API_KEY]

fn get_channel_json(channel_url: &String) -> Option<Value> {
    reqwest::blocking::get(channel_url)
        .ok()?
        .json::<Value>()
        .ok()
}

fn get_playlist_videos(playlist_id: &String, api_key: &String) {
    let playlist_url:String = format!("https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2C%20contentDetails&maxResults=5&playlistId={}&key={}", playlist_id, api_key);
    // https://youtube.googleapis.com/youtube/v3/playlistItems?part=snippet%2C%20contentDetails&maxResults=5&playlistId=[PLAYLIST_ID]&key=[PERSONAL_API_KEY]

    //println!("channel info url: {}", playlist_url);

    let playlist_json = get_playlist_json(&playlist_url)
        .expect("Failed to find upload playlist json.");

    display_information(playlist_json);
}

fn display_information(playlist_json: Playlist) {
    for i in 0..4 {
        println!("Title: {}\nRelease Date: {}\n",
        playlist_json.items[i].snippet.title,
        playlist_json.items[i].snippet.published_at,);
    }
}

fn get_playlist_json(playlist_url: &String) -> Option<Playlist> {
    reqwest::blocking::get(playlist_url)
        .ok()?
        .json::<Playlist>()
        .ok()
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Playlist {
    //next_page_token: String,
    items: Vec<Video>,
}

#[derive(Debug, Deserialize)]
struct Video {
    snippet: Snippet,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Snippet {
    published_at: String,
    title: String,
}

// trim removes whitespace at beginning and end of string such as the \n
// parse changes the type example: from string to int
// let guess: u32 = guess.trim().parse().expect("Please type a number!");

// https://www.youtube.com/user/VivaLaDirtLeague
// https://www.youtube.com/user/AlecBenjaminMusic
// https://www.youtube.com/user/PewDiePie
// format for the youtube channels
