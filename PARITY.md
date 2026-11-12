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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [ca87680fb0](https://github.com/storytold/cadcraft/commit/ca87680fb0f9d7803d3e64c808d0a3f7a196fdaa) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [9b1fe3ff23](https://github.com/storytold/deckcraft/commit/9b1fe3ff23ca55ace1acd97a7376f5dcebfa6e22) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [a17fb5d3d8](https://github.com/storytold/designcraft/commit/a17fb5d3d8ca53072c38366c88916b20fccaa340) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [fbfeed2974](https://github.com/storytold/effectcraft/commit/fbfeed29745840ee8f69de9a7e3a301950de813a) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [5f37739c40](https://github.com/storytold/filmcraft/commit/5f37739c402418f0bc22b27a3f423edad78841c8) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [37941f0624](https://github.com/storytold/gridcraft/commit/37941f06244565bdc91d32394d74fb9b0e48fae4) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [a75497d634](https://github.com/storytold/lightcraft/commit/a75497d63405b448b03227ebcffbb08fca18ef48) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [d749fd8062](https://github.com/storytold/pdfcraft/commit/d749fd8062655c333c0733d8456bd8d4c9925a12) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [7ca5a9c40f](https://github.com/storytold/photocraft/commit/7ca5a9c40f8088c949007905c533b5ff9cd1b624) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [6a0949d9d0](https://github.com/storytold/soundcraft/commit/6a0949d9d0dc80e9d4fe9bdb008fbe0f1654ccd6) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [89fba4ebfd](https://github.com/storytold/vectorcraft/commit/89fba4ebfd5a1aad610ef51c268aea5ccf174a54) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [07f977372d](https://github.com/storytold/wordcraft/commit/07f977372d3767e4abf7f3b0cc77c351289013f2) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [84a6a7dcf5](https://github.com/storytold/artcraft/commit/84a6a7dcf5ccd5f8f577e5dafe84494532cc14fb) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [66beafa52e](https://github.com/storytold/craft-launcher/commit/66beafa52e2fffb082fe2bb79c702520b332c0bb) |
