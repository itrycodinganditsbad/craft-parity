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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [3b7eb1b7bc](https://github.com/storytold/cadcraft/commit/3b7eb1b7bcbb5dd9a4f71ac1997f0e31105f2e82) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [9a95c3a639](https://github.com/storytold/deckcraft/commit/9a95c3a63942377b1a4c8d79844a94a54ffac370) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [8fce28258d](https://github.com/storytold/designcraft/commit/8fce28258def76e0079848933dbc31c9665b6767) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [7fc1be0bf2](https://github.com/storytold/effectcraft/commit/7fc1be0bf2d5701ea1fd5aeb9158a1d6c9f90d3f) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [4b3dcf3c48](https://github.com/storytold/filmcraft/commit/4b3dcf3c4811365a99667f94ee5073413c176e0b) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [ff35c6b89d](https://github.com/storytold/gridcraft/commit/ff35c6b89db7c3572292579803da4b876af15ec7) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [081190c92f](https://github.com/storytold/lightcraft/commit/081190c92f4f74340a7fc4125fce1fe6d2b60ed9) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [dc5cb122db](https://github.com/storytold/pdfcraft/commit/dc5cb122db4e79e795e92073340262f5c40d1344) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [7904eee761](https://github.com/storytold/photocraft/commit/7904eee7610e1ef7c0fd65384170ff1545af55fd) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [e66a5e7c67](https://github.com/storytold/soundcraft/commit/e66a5e7c67ce790ec133f985d7cdc1a128b1001d) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [f3d9970f69](https://github.com/storytold/vectorcraft/commit/f3d9970f690fdc1a018980b00fc68863efb3f006) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [247c7feaab](https://github.com/storytold/wordcraft/commit/247c7feaab699e3df36879bba3072593de2c65cb) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [58f5944fe6](https://github.com/storytold/artcraft/commit/58f5944fe66e4cfdfd69fc3eebcc67e38f75a1a9) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [0027613a90](https://github.com/storytold/craft-launcher/commit/0027613a909edfaa80eefd0b4027168dd3d5c876) |
