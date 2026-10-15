# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: gridcraft, lightcraft, photocraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

### Desktop: log to a rotating log file as well as standard error

An app started from the Dock, Start menu or a launcher has no terminal, so its log records are lost
and a crash report comes without a log. The fix keeps the last few runs' logs in the app's data
folder, capped in size.

Missing in: effectcraft. Port from [storytold/pdfcraft#239](https://github.com/storytold/pdfcraft/pull/239), [storytold/photocraft#1143](https://github.com/storytold/photocraft/pull/1143), [storytold/wordcraft#17](https://github.com/storytold/wordcraft/pull/17).

## Exemptions

- bidi-text / artcraftx: exempt, web UI only; the browser lays out right-to-left text
- rotating-log-file / artcraft: exempt, Tauri app; logs to a file through tauri-plugin-log
- rotating-log-file / artcraftx: exempt, Tauri app; logs to a file through tauri-plugin-log
- windows-primary-gpu / cadcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / deckcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / designcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / effectcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / filmcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / gridcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / lightcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / photocraft: exempt, its adapter selector ranks discrete GPUs first, so #378 can't happen
- windows-primary-gpu / soundcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / wordcraft: exempt, defaults to the high-performance GPU, so #378 can't happen; the rule would move hybrid laptops to the iGPU
- windows-primary-gpu / artcraft: exempt, Tauri app; the webview draws the window, no Rust code uses wgpu
- windows-primary-gpu / craft-launcher: exempt, its adapter selector ranks discrete GPUs first unless WGPU_POWER_PREF asks for low power

## Scanned

| App | Ref | Commit |
|---|---|---|
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [fc4e98315a](https://github.com/storytold/cadcraft/commit/fc4e98315a8d771b5ab832f0ade0b5e7b106df7f) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [9859da3925](https://github.com/storytold/deckcraft/commit/9859da39255c0262cd592bb57101d7f2fa961f72) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [a930276fd7](https://github.com/storytold/designcraft/commit/a930276fd7462f3d9abd2fbb38544c94c0c911c8) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [0311bdc34d](https://github.com/storytold/effectcraft/commit/0311bdc34d0dc10c2d18ff7875b4f52d483e5e27) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [8309f21a6c](https://github.com/storytold/filmcraft/commit/8309f21a6c3f34ddf190af7fd9bfd913053a1fd5) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [e6b6b34970](https://github.com/storytold/gridcraft/commit/e6b6b349708ab4978cb3f061f6bd6ecbc25667cb) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [0da998f24d](https://github.com/storytold/lightcraft/commit/0da998f24dfc03069d49867f8877af2bf7c51f94) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [afa8e1843c](https://github.com/storytold/pdfcraft/commit/afa8e1843cfed613f3d403119f6c2649bce824c9) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [df055f3dcb](https://github.com/storytold/photocraft/commit/df055f3dcb0bb0312af97548ee6c6f9628f21dd7) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [3a28a65769](https://github.com/storytold/soundcraft/commit/3a28a657692c1b98b28c276d8f61df9471074f0f) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [b12a52f724](https://github.com/storytold/vectorcraft/commit/b12a52f7249e0aa73f030587e05ed9ce0c73ab67) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [fd0bb2fb1e](https://github.com/storytold/wordcraft/commit/fd0bb2fb1ec081e3eddc4d01917637fef0cafdfe) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [3fd2d56999](https://github.com/storytold/craft-launcher/commit/3fd2d56999fa00a7b90f159cb686a5fc64347e7e) |
