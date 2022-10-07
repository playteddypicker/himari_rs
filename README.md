# himari_rs

이게 뭐냐면..  
알려드렸습니다~

음악봇(discordMusicbotTemplate https://github.com/playteddypicker/discordMusicbotTemplate)  
Rust로 새로 짜고있는거

기능 여러개 넣을거임  
구상해둔거 많음ㅋ

![ㅎㅁㄹ](https://media.discordapp.net/attachments/934297359209340939/1026216079619539114/unknown.png)

## 개발일지

#### 2020.10.04

- 페이지처럼 넘기는 임베드 프레임워크 만드는중인데 수명때매 걸림
- interaction 만들때 InteractionResponseData가 if let 구문 끝나면 사라지는데 이를 await으로 수명을 고정시켜야함
- 그래야 미리 해제가 안되서 interaction failed(메모리가 이미 해제되서 강제로 없어짐)가 뜨는 에러를 방지할수있음
- dynamic Future + lifetime 좀 엄밀하게 공부해놔야 해결가능함

#### 2020.10.07

- 수명 관련 에러 해결. 임베드 데이터가 쓰이는 범위를 체크해서 그냥 그 자체를 넘기면 수명 체크 안해도 됨
- reactionpages 구현 완료
- 콘솔 에러를 log 크레이트를 이용해서 좀 더 가시적 가독성 좋게 보이게끔 함

# TODO LIST

```
[ ] : 아직 안만듬
[~] : 작업중
[C] : 완료, 디버깅만 남음

src/
  utils/
    [ ] music_info_search.rs
    [ ] server_info.rs
    [C] reaction_page.rs
    [ ] timestamp.rs
    [ ] lavalinik_get_info.rs
    [ ] get_music_info.himari_rs
  command_handler/
    [~] command_handler.rs
      music_cmd/
        [ ] play.rs
		[ ] pause.rs
        [ ] skip.rs
        [ ] stop.rs
        [ ] eject.rs
		[ ] queue.rs
        [ ] nowplaying.rs
        [ ] history.rs
        [ ] loop.rs
		[ ] move.rs
        [ ] jump.rs
		[ ] remove.rs
		[ ] shuffle.rs
		[ ] volume.rs
	  server_player_cmd/
		[ ] fast_setup.rs
		[ ] setting.rs
		[ ] get_server_info.rs
	  playlist_cmd/
		[ ] personal_playlist.rs
		[ ] guild_playlist.rs
		[ ] bot_playlist.rs
	  util_cmd/
	  	[~] start.rs
	  other_cmd/
	  	[C] saysomething.rs
  event_handler/
	[~] event_handler.rs
	  events/
		[ ] interaction_create_default_cmd.rs
		[~] interaction_create.rs
		[ ] voice_state_update.rs
		[ ] channel_name_update.rs
		[~] guild_create.rs
		[ ] guild_delete.rs
     	[ ] channel_delete.rs
  [C] main.rs
```

**Event Handler**

- [x] Basic Structure
- guildCreate
  - [x] 초대된 서버에 start 커맨드 등록
- Ready
  - [x] Ready되면 콘솔에 띄우기
- InteractionCreate
  - [x] Interaction 발생하면 Command인지 검사 후 Handler로 넘겨주기
  - [x] Interaction 발생하면 Button인지 검사 후
    - [x] Page 넘기기인지 검사
    - [x] Component Reaction인지 검사
- voiceStateUpdate
  - 나중에 추가 예정

**Command Handler**

- [x] Basic Structure
- [x] Response type에 따라 분류
- 기본 명령어 목록
  - [x] /아무말
  - [ ] /help
- 음악 명령어 목록
  - [ ] /play
  - [ ] /np
  - [ ] /q
  - [ ] /history
  - [ ] /pause
  - [ ] /skip
  - [ ] /stop
  - [ ] /eject
  - [ ] /loop
  - [ ] /shuffle
  - [ ] /volume
  - [ ] /jump
  - [ ] /remove
  - [ ] /move
  - [ ] /volume
- 플레이어 명령어 목록
  - [ ] /setup
  - [ ] /setting
  - [ ] /player
- 유틸 명령어 목록
  - [ ] /start
  - [ ] /refresh
  - [ ] /reset
  - [ ] /debug
- 플레이리스트 명령어 목록
  - [ ] /playlist
  - [ ] /recommend
  - [ ] /board

**플레이어 기능**

- [ ] 할거 존나맣ㄴ네 시발
