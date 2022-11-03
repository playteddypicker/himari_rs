# himari_rs

이게 뭐냐면..  
알려드렸습니다~

음악봇(discordMusicbotTemplate https://github.com/playteddypicker/discordMusicbotTemplate)  
Rust로 새로 짜고있는거

기능 여러개 넣을거임  
구상해둔거 많음ㅋ

![ㅎㅁㄹ](https://media.discordapp.net/attachments/934297359209340939/1026216079619539114/unknown.png)

## 개발일지

#### 2022.10.04

- 페이지처럼 넘기는 임베드 프레임워크 만드는중인데 수명때매 걸림
- interaction 만들때 InteractionResponseData가 if let 구문 끝나면 사라지는데 이를 await으로 수명을 고정시켜야함
- 그래야 미리 해제가 안되서 interaction failed(메모리가 이미 해제되서 강제로 없어짐)가 뜨는 에러를 방지할수있음
- dynamic Future + lifetime 좀 엄밀하게 공부해놔야 해결가능함

#### 2022.10.07

- 수명 관련 에러 해결. 임베드 데이터가 쓰이는 범위를 체크해서 그냥 그 자체를 넘기면 수명 체크 안해도 됨
- reactionpages 구현 완료
- 콘솔 에러를 log 크레이트를 이용해서 좀 더 가시적 가독성 좋게 보이게끔 함

#### 2022. 10. 14

- /start로 봇 업데이트 관리같은걸 할건데, command_handler에서 객체 상속을 다시짜야함
- 근데 러스트는 상속을 지원 안함. 러스트만의 OOP 핸들링 방식이 있어서 그거 따르는중인데 너무 어려움
- 그래서 커맨드 핸들러를 다시 짜는중임. 버튼임베드는 나중에..

#### 2022. 10. 21

- 커맨드 핸들러 짜는거 길을 찾았는데 static 수명 이슈로.. 이 씨발 좆같

#### 2022. 10. 22

- 커맨드 핸들러 다짬 ㅅㅂ 뒤ㅣㅈ는줄알ㅇㅁ
- 시스템언어라 그런지 JS에서는 async/await 하나면 될거를 ㅅ.ㅂ 별의별 이상한 어? 하..
- 너무 많은일이 있었어 너무힘들다증말

#### 2022. 10. 28

- 아무말 명령어 캐릭터별로 다르게끔 하게 수정
- /start 커맨드로 봇 업데이트 체크 or 명령어 등록하게끔 구현함(수명에러 해결ㅋ)
- 이제 기본적인 명령어 구현은 다 끝남. 버튼 상호작용 후 reply하는거만 추가하면 됨

#### 2022. 11. 03

- 명령어로 연결하는거 핸들러 구현 완료
- 이제 검색하는거 구현해야함

# TODO LIST

### 큰거 세개

#### 1. CommandHandler <<완료

#### 2. Button_embed <<진행중

#### 3. VoiceEventHandler

#### 4. lavalink 통합

```
[ ] : 아직 안만듬
[~] : 작업중
[C] : 완료, 디버깅만 남음

src/
  utils/
  	structures/
	  [ ] server_info.rs
	  [ ] server_db_schema.rs
	frameworks/
	  [C] reaction_page.rs
	  [~] button_embed.rs
	music_modules/
      [ ] lavalinik_get_info.rs
      [ ] get_music_info.rs
      [ ] music_info_search.rs
  command_handler/
    [~] command_handler.rs
    [C] assign_command.rs
    [C] assign_checker.rs
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
	  	[C] start.rs
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
  - [x] /start
  - [ ] /refresh
  - [ ] /reset
  - [ ] /debug
- 플레이리스트 명령어 목록
  - [ ] /playlist
  - [ ] /recommend
  - [ ] /board

**플레이어 기능**

- [ ] 할거 존나맣ㄴ네 시발
