use regex::Regex;
use songbird::input::error::Error;
use songbird::input::Input;
use songbird::input::Metadata;

use super::parse_song_info::{yt_single, SongMetadata};

enum SearchRequestType {
    YoutubeSingleUrl,
    YoutubePlaylistUrl,
    SoundcloudSingleUrl,
    SoundcloudPlaylistUrl,
    SoundcloudUnresolvedUrl,
    SpotifySingleUrl,
    SpotifyPlaylistUrl,
    NotSupportingUrl,
    SearchString,
}

const REGEX_URLS: [&'static str; 11] = [
    //&index=뭐시기 있으면 플리에서 곡 하나 트는거임. 이거는 단일 링크로 취급 ㄱ
    r"^https?://(www.youtube.com|youtube.com|youtu.be|m.youtube.com).*(list=PL|list=OL)([^#\&\?]*)(&index=)",
    //이거 두개는 전부 플리링크
    r"^https?://(www.youtube.com|youtube.com|youtu.be|m.youtube.com).*(list=PL|list=OL)([^#\&\?]*)",
    r"^https?://(www.youtube.com|youtube.com|youtu.be|m.youtube.com)/playlist(.*)$",
    //플리에 전부 안걸리면 단일 링크임
    r"^https?://(www.youtube.com|youtube.com|youtu.be|m.youtube.com)/(.*)$",
    //사클 링크
    r"^https?://(soundcloud.com|snd.sc|m.soundcloud.com)/(.*)/(sets)/(.*)$",
    r"^https?://(soundcloud.com|snd.sc|m.soundcloud.com)/(.*)$",
    //사클 단축링크. resolve해봐야 알음
    r"^https?://(on.soundcloud.com)/(.*)$",
    //스포티파이 링크
    r"^https?://(open.spotify.com)/(track)/(.*)$",
    r"^https?://(open.spotify.com)/(playlist)/(.*)$",
    r"^https?://(open.spotify.com)/(album)/(.*)$",
    //기타 url. 아직 지원하지 않음.
    r"^https?://(.*)$",
];

fn match_request_type(search_string: &String) -> SearchRequestType {
    let mut regex_code = 99999;

    for (idx, rex) in REGEX_URLS.iter().enumerate() {
        if Regex::new(rex).unwrap().is_match(search_string) {
            regex_code = idx;
            break;
        }
    }

    return match regex_code {
        0 | 3 => SearchRequestType::YoutubeSingleUrl,
        1 | 2 => SearchRequestType::YoutubePlaylistUrl,
        4 => SearchRequestType::SoundcloudPlaylistUrl,
        5 => SearchRequestType::SoundcloudSingleUrl,
        6 => SearchRequestType::SoundcloudUnresolvedUrl,
        7 => SearchRequestType::SpotifySingleUrl,
        8 | 9 => SearchRequestType::SpotifyPlaylistUrl,
        10 => SearchRequestType::NotSupportingUrl,
        _ => SearchRequestType::SearchString,
    };
}

async fn get_song_stream(search_url: String) {}

//여기서는 url 구분하고 그거에 맞는
pub async fn request_main(search_query: (String, bool)) -> Option<SongMetadata> {
    return match match_request_type(&search_query.0) {
        SearchRequestType::YoutubeSingleUrl => Some(SongMetadata::from_youtube_single(
            yt_single(search_query.0).await.unwrap(),
        )),
        _ => None, /*SearchRequestType::YoutubePlaylistUrl => "유튜브 플레이리스트 링크입니다".to_string(),
                   SearchRequestType::SoundcloudSingleUrl => "사운드클라우드 단일 링크입니다.".to_string(),
                   SearchRequestType::SoundcloudPlaylistUrl => {
                       "사운드클라우드 플레이리스트 링크입니다.".to_string()
                   }
                   SearchRequestType::SoundcloudUnresolvedUrl => {
                       "사운드클라우드 단축 링크입니다.".to_string()
                   }
                   SearchRequestType::SpotifySingleUrl => Ok("스포티파이 단일 링크입니다".to_string()),
                   SearchRequestType::SpotifyPlaylistUrl => {
                       "스포티파이 플레이리스트 링크입니다".to_string()
                   }
                   SearchRequestType::NotSupportingUrl => {
                       "지원하지 않는 링크입니다. 지금은요..".to_string()
                   }
                   SearchRequestType::SearchString => "링크가 아닙니다. 검색합니다..".to_string(),*/
    };
}
