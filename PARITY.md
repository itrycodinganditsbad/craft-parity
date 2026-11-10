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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [6f0f1fefa8](https://github.com/storytold/cadcraft/commit/6f0f1fefa81d08031bde56a8b3d5b1556810f397) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [0623b43032](https://github.com/storytold/deckcraft/commit/0623b430324cf4c3d1962e6ae7885cf30c011c5e) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [95abb2b03b](https://github.com/storytold/designcraft/commit/95abb2b03b6d510a72d687b2e5ee41ef3cbd7485) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [f88e7980ae](https://github.com/storytold/effectcraft/commit/f88e7980ae4d69a014579f6b31d8b271cd4c3362) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [2d536e6af1](https://github.com/storytold/filmcraft/commit/2d536e6af1c886887e8299edfb34b5d565e30a69) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [74ab5cc969](https://github.com/storytold/gridcraft/commit/74ab5cc96930cad7d05e4553859b96c746dc5919) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [e9c29e2c5c](https://github.com/storytold/lightcraft/commit/e9c29e2c5c5dd1f86cc98677b5dc389f6612fcac) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [1a159c57fc](https://github.com/storytold/pdfcraft/commit/1a159c57fc48e7106cb3373b17941a3899c9135f) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [ec97a146f5](https://github.com/storytold/photocraft/commit/ec97a146f50bea068dd0ef71be501f55a7aacce0) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [ef58a90f4a](https://github.com/storytold/soundcraft/commit/ef58a90f4a418a282a15fc71d2e03c3d8e8f40fa) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [73dc058596](https://github.com/storytold/vectorcraft/commit/73dc058596e42ad621d3d875a87dfcbde14eac55) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [8548b33be1](https://github.com/storytold/wordcraft/commit/8548b33be1c9238e6d42fb2a72fe27ccdd56be7f) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1510abf563](https://github.com/storytold/artcraft/commit/1510abf5639393fc7dbda01fc17ff59792cbc885) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [753a7102a5](https://github.com/storytold/craft-launcher/commit/753a7102a52623e3cc9087a7a25c65db241519c8) |
