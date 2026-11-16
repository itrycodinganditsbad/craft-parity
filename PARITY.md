# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: lightcraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [16341b665f](https://github.com/storytold/cadcraft/commit/16341b665fa358d6fa1847e4f2325b5a5b0ec037) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [6c52db0172](https://github.com/storytold/deckcraft/commit/6c52db01727b4d3cfb2e122770ce76de6a6fa71e) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [a73f46df33](https://github.com/storytold/designcraft/commit/a73f46df3332ae9f6aa2b415ee43f1b688a15da4) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [cb9c1b5e1a](https://github.com/storytold/effectcraft/commit/cb9c1b5e1aed55ee70233340b61fd1a3227df209) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [baeca9db89](https://github.com/storytold/filmcraft/commit/baeca9db89e54723a86e02e0e3a550e5ea2b16d2) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [abcfc19153](https://github.com/storytold/gridcraft/commit/abcfc191533519dde56bfea65fd1d84e0df6f489) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [41d6883085](https://github.com/storytold/lightcraft/commit/41d68830857f37655b7a7277dc986dc109759c56) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [207a50ef13](https://github.com/storytold/pdfcraft/commit/207a50ef133a99d051d94ad655a9d1cf40745199) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [d53b5edb60](https://github.com/storytold/photocraft/commit/d53b5edb600d2e9215104fbea6c4ebc15619ff19) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [648ed9db9a](https://github.com/storytold/soundcraft/commit/648ed9db9a0e98cc55643babffac381521a556c8) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [b352745d44](https://github.com/storytold/vectorcraft/commit/b352745d44e007b3b77a21a2df05a4c9ede125d8) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [cdf146cabd](https://github.com/storytold/wordcraft/commit/cdf146cabd9df5a090a4ae6ef13dfeac7452737c) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [69f53c2f03](https://github.com/storytold/artcraft/commit/69f53c2f0358a91a5e35e5a7b3a91677320c3112) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [2bcbdd1ac0](https://github.com/storytold/artcraftx/commit/2bcbdd1ac0c06666b7fb33fd240e5df0cc3dac86) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [cbe639c592](https://github.com/storytold/craft-launcher/commit/cbe639c5927712db69ce43476509d5ee97dc2cad) |
