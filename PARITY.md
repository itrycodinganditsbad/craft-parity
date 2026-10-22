# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: gridcraft, lightcraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [75d9d72703](https://github.com/storytold/cadcraft/commit/75d9d7270378081f4ffbced674f66433c1acb7ad) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [128cb0b883](https://github.com/storytold/deckcraft/commit/128cb0b883c1210a8aeda30c713b889f5d92a984) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [f3b7c3ee34](https://github.com/storytold/designcraft/commit/f3b7c3ee34e402b1e5c1854e7e8a86e876bd286e) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [f37f20ffd0](https://github.com/storytold/effectcraft/commit/f37f20ffd07d0f79e163eef1ad703b87db427dc3) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [45901fccd0](https://github.com/storytold/filmcraft/commit/45901fccd06f1c2aa1dcdaac67cc00262a85f1be) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [06920de8da](https://github.com/storytold/gridcraft/commit/06920de8da609c19de7446945cfc62b844691efd) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [31ee2643cf](https://github.com/storytold/lightcraft/commit/31ee2643cf208a18cf780b8a63c677e87ab256f7) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [9a0ec64568](https://github.com/storytold/pdfcraft/commit/9a0ec64568088fe7e74ef540817b66b7c1a50553) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [78b46b1a99](https://github.com/storytold/photocraft/commit/78b46b1a9911b9366f673e653043cfdd1a6d4177) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [9b8283b908](https://github.com/storytold/soundcraft/commit/9b8283b90877813a929f5e28b2dc92e5774da933) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [bbaf7b02c3](https://github.com/storytold/vectorcraft/commit/bbaf7b02c3f82084ae5e7fb1eb01a703e17d08f9) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [8e986366c7](https://github.com/storytold/wordcraft/commit/8e986366c7a5a261e058644a9446d4c73aa8a063) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [800217ed04](https://github.com/storytold/artcraft/commit/800217ed04d10fc12706c28e6e91ae2a6b286092) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [bf39e92f30](https://github.com/storytold/craft-launcher/commit/bf39e92f30bcf1f86d07d7aae1e877632f74bce6) |
