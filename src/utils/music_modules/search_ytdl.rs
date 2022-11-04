use songbird::input::error::Error;
use songbird::input::restartable::Restartable;
use songbird::Call;

async fn make_source(call: Call) {
    let source_handler = Restartable::ytdl("https://youtu.be/ojULkWEUsPs", true)
        .await
        .unwrap();
}

fn error_code(err: Error) -> String {
    match Error {
        Error::Metadata => "영상 정보를 가져오는데 실패했습니다.".to_string(),
        Error::YoutubeDlProcessing(output) => format!("영상 소스를 다운받는 데 실패했습니다."),
        _ => "ㄴㄱㅁ".to_string(),
    }
}
