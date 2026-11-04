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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [2099288f54](https://github.com/storytold/cadcraft/commit/2099288f5449b1602d2e2eedca92fe99fb19b973) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [23ac4f7efd](https://github.com/storytold/deckcraft/commit/23ac4f7efdbd1e538bd9a7869584d4adeb9039b0) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [df9f0656ca](https://github.com/storytold/designcraft/commit/df9f0656cac78d48fb1de453d209cffb7d6e84f0) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [bb78d55825](https://github.com/storytold/effectcraft/commit/bb78d55825adc879ae522c18ac57c4ce5690318d) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [c9be14245a](https://github.com/storytold/filmcraft/commit/c9be14245a6c9da585ff906a6e0ca9c8200652ea) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [632e4a1bc5](https://github.com/storytold/gridcraft/commit/632e4a1bc51ee86b3f0a55e95ee78def391aab30) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [f30dd5e5f7](https://github.com/storytold/lightcraft/commit/f30dd5e5f71a94c4ade50521a4080ea5a0ad1479) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [8717a085ff](https://github.com/storytold/pdfcraft/commit/8717a085ffec6b6de3ffa86de9c0c51da1488000) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [01ff0e2974](https://github.com/storytold/photocraft/commit/01ff0e29741dec9217237168548abebe1bd3d495) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [5fabf28c31](https://github.com/storytold/soundcraft/commit/5fabf28c315ee72ad5c0234beeb35ec832c11476) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [bfc42c9482](https://github.com/storytold/vectorcraft/commit/bfc42c948291b0bb329eca323ca211f17d027558) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [1350f62bf0](https://github.com/storytold/wordcraft/commit/1350f62bf066b6d062f759eeba6fc118bad70d2f) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [9a317f2a8a](https://github.com/storytold/craft-launcher/commit/9a317f2a8a6282d1a7de436b834f9ed88fac5334) |
