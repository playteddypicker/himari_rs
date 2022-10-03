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

# TODO LIST

**Event Handler**

- [x] Basic Structure
- guildCreate
  - [x] 초대된 서버에 start 커맨드 등록
- Ready
  - [x] Ready되면 콘솔에 띄우기
- InteractionCreate
  - [x] Interaction 발생하면 Command인지 검사 후 Handler로 넘겨주기
  - [ ] Interaction 발생하면 Button인지 검사 후
    - [ ] Page 넘기기인지 검사
    - [ ] Component Reaction인지 검사
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
