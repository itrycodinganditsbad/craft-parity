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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [f536bf406f](https://github.com/storytold/cadcraft/commit/f536bf406f01541006c81deec47ec842acc83158) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [3a4e30e10e](https://github.com/storytold/deckcraft/commit/3a4e30e10e83c5f988a074dba1a519984828b4fd) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [ea08e4c2ba](https://github.com/storytold/designcraft/commit/ea08e4c2ba290f092063c99d70c4bdd5afc45887) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [ec7ced1f05](https://github.com/storytold/effectcraft/commit/ec7ced1f0515f0b6f376dbcc297f1c441f452d6e) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [fed0f42fd5](https://github.com/storytold/filmcraft/commit/fed0f42fd52012a337d744743a93409c0b1e8f9d) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [68a4b1a1c3](https://github.com/storytold/gridcraft/commit/68a4b1a1c3684fa17553ecb2a1f088c75f6a3567) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [445dd3106e](https://github.com/storytold/lightcraft/commit/445dd3106ef0b9f399584505daa29cbfba636c96) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [1d1cbc4ec9](https://github.com/storytold/pdfcraft/commit/1d1cbc4ec92012767f5b48c92b451e15eb401c42) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [382b047c30](https://github.com/storytold/photocraft/commit/382b047c30483024159aaa63831947bc99a5e28b) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [7902d59b47](https://github.com/storytold/soundcraft/commit/7902d59b475c1df203b212ef3436cc80bee2ccf2) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [3d62a009fd](https://github.com/storytold/vectorcraft/commit/3d62a009fda2cc138a7460c91075da8342133de6) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [3b7f7ed10b](https://github.com/storytold/wordcraft/commit/3b7f7ed10b2eb0ae3c39b71551d36ed2df1d1d8b) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [3fad1d643f](https://github.com/storytold/craft-launcher/commit/3fad1d643f3ad3fd55b1f562efe53b8a334e3a72) |
