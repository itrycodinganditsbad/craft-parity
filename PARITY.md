# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: lightcraft, photocraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [b23d172885](https://github.com/storytold/cadcraft/commit/b23d17288555cd24827fc0587d91fbd4f327f069) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [5e36a3552e](https://github.com/storytold/deckcraft/commit/5e36a3552e1bb43a4ae64df4362b244281b2a220) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [f5a94959f9](https://github.com/storytold/designcraft/commit/f5a94959f920f7455d05f83c72bfc3d02bcce90c) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [3f1cd00745](https://github.com/storytold/effectcraft/commit/3f1cd00745d41345632b38beece0b09c9e265e69) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [688444e75d](https://github.com/storytold/filmcraft/commit/688444e75d059c0d2a5c13be75fc3cb210efebd1) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [aef4ceb10d](https://github.com/storytold/gridcraft/commit/aef4ceb10dd62dcd0f06e611485ab32635ba7c58) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [d5382f20b4](https://github.com/storytold/lightcraft/commit/d5382f20b4e9e12f329e1b806333514ba7498ff8) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [e56b03fbae](https://github.com/storytold/pdfcraft/commit/e56b03fbaeb8c37f78f0b917c20bc7d93dbfbfe6) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [5718a98c1f](https://github.com/storytold/photocraft/commit/5718a98c1fa8c935c9007e24db9d82372b52d574) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [56d5c38547](https://github.com/storytold/soundcraft/commit/56d5c38547fbfd5a9eb1543da9b7558f3a836a8d) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [dce8a0c05e](https://github.com/storytold/vectorcraft/commit/dce8a0c05ea215d113663eb4800edcad23e970fa) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [4e8c9d5531](https://github.com/storytold/wordcraft/commit/4e8c9d5531f29f059755e1ca205f6f6104096616) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [a6d52df45e](https://github.com/storytold/artcraft/commit/a6d52df45e553a6119116e560403155b0c65dc60) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [fba78de478](https://github.com/storytold/artcraftx/commit/fba78de4782fe727739b4e34ed8b547cbd601632) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [b67298858f](https://github.com/storytold/craft-launcher/commit/b67298858fb64830169007fb04eefe4a2a1ae74e) |
