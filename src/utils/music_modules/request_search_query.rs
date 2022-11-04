use crate::utils::structures::guild_queue;
use regex::Regex;

enum SearchRequestType {
    YoutubeSingleUrl,
    YoutubePlaylistUrl,
    SoundcloudSingleUrl,
    SoundcloudPlaylistUrl,
    SpotifySingleUrl,
    SpotifyPlaylistUrl,
    SearchString,
}

const REGEX_URLS: [&'static str; 8] = [
    r"^https?:\/\/(www\.youtube\.com|youtube\.com|youtu\.be)\/(.*)$",
    r"^https?:\/\/(www\.youtube\.com|youtube\.com|youtu\.be)\/playlist(.*)$",
    r"^https?:\/\/(www\.youtube\.com|youtube\.com|youtu\.be).*(list=PL|list=OL)([^#\&\?]*)",
    r"^https?:\/\/(soundcloud\.com|snd\.sc)\/(.*)$",
    r"^https?:\/\/(soundcloud\.com|snd\.sc)\/(.*)\/(sets)\/(.*)$",
    r"^https?:\/\/(open\.spotify\.com)\/(track)\/(.*)$",
    r"^https?:\/\/(open\.spotify\.com)\/(playlist)\/(.*)$",
    r"^https?:\/\/(open\.spotify\.com)\/(album)\/(.*)$",
];

fn match_request_type(search_string: String) -> SearchRequestType {
    let mut regex_code = 99999;

    REGEX_URLS.iter().enumerate().for_each(|(idx, rex)| {
        if Regex::new(rex).unwrap().is_match(&search_string) {
            regex_code = idx;
        }
    });

    return match regex_code {
        0 => SearchRequestType::YoutubeSingleUrl,
        1 | 2 => SearchRequestType::YoutubePlaylistUrl,
        3 => SearchRequestType::SoundcloudSingleUrl,
        4 => SearchRequestType::SoundcloudPlaylistUrl,
        5 => SearchRequestType::SpotifySingleUrl,
        6 | 7 => SearchRequestType::SpotifyPlaylistUrl,
        _ => SearchRequestType::SearchString,
    };
}

pub async fn request_search(search_query: (String, bool)) {
    match match_request_type(search_query.0.clone()) {
        SearchRequestType::YoutubeSingleUrl => {}
        SearchRequestType::YoutubePlaylistUrl => {}
        SearchRequestType::SoundcloudSingleUrl => {}
        SearchRequestType::SoundcloudPlaylistUrl => {}
        SearchRequestType::SpotifySingleUrl => {}
        SearchRequestType::SpotifyPlaylistUrl => {}
        SearchRequestType::SearchString => {}
    }
}
