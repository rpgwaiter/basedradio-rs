Rewrite of based-radio in rust, WIP

## Dev for web

```sh
dx serve --platform=web --addr=0.0.0.0 --port=8787 --features web --package basedradio-rs
```

### Dev for Desktop
```sh
dx serve --platform=desktop --features desktop --package basedradio-rs
```


### TODO:

#### soon (tm)
- [ ] resizable windows
- [ ] actual image links instead of base64
- [ ] updates window
- [ ] ci for web and desktop w/ releases

#### kinda soon
- [ ] updated readme
- [ ] show current song count
- [ ] add A LOT more songs. We used to have over 400 songs before a data storage failure many years ago
- [ ] .json with info on each game/system/song(?)
- [ ] per-game backgrounds
- [ ] settings panel for backgrounds, other stuff
- [ ] fix the hacky window dragging
- [ ] opus streams (currently only mp3)
- [ ] visualizer like current based.radio has. Easy in web (due to existing code), very tricky for desktop

#### someday
- [ ] rewrite the python api in rust
- [ ] auto updater for desktop
- [ ] nix package (requires a lot of work, or maybe I'm just missing something)
- [ ] listener count (will require an mpd replacement or something really hacky)
- [ ] discord bot
- [ ] different playlists for events, livecasting, etc
- [ ] login system
- [ ] voting w/ weights (would require replacement of mpd playlist management)
- [ ] option to bounce the player window around the screen like those old dvd menus