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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [7cee28580d](https://github.com/storytold/cadcraft/commit/7cee28580d1a459856b8b65c1e8890514ac35243) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [a3fbf02952](https://github.com/storytold/deckcraft/commit/a3fbf02952839c223df600dc33106a2dbc65be1b) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [ea214dd44d](https://github.com/storytold/designcraft/commit/ea214dd44dbfbfed23ffd79e5efaba375510afb4) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [9dd3193cca](https://github.com/storytold/effectcraft/commit/9dd3193cca54ab763fa0392a371e9ab836801672) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [142fddc191](https://github.com/storytold/filmcraft/commit/142fddc1910d179e8111f3263ab3bb3c87fc529b) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [981fad14e0](https://github.com/storytold/gridcraft/commit/981fad14e0b90a7906a02f29785a6d0eb214fee6) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [c4fd54e0a6](https://github.com/storytold/lightcraft/commit/c4fd54e0a62539976e1729073b4da0681de790bb) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [f137a289ba](https://github.com/storytold/pdfcraft/commit/f137a289baadefbd29e0e46930899896068cc41d) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [422ca7beda](https://github.com/storytold/photocraft/commit/422ca7beda90d927643bcd074646ebbd37706d75) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [4179cf88e6](https://github.com/storytold/soundcraft/commit/4179cf88e6d0ec9db09b7292b678bbbf978c2894) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [cf0d4d4a3f](https://github.com/storytold/vectorcraft/commit/cf0d4d4a3f66e88b4cfd6b8dc0c640ebe26dc7e7) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [eecbb8fe57](https://github.com/storytold/wordcraft/commit/eecbb8fe57a56340e7fa109e8da7f1376a03e154) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [68eeee97b8](https://github.com/storytold/craft-launcher/commit/68eeee97b813af4cad59e1f216ea5ca93151e420) |
