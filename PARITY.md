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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [5654240729](https://github.com/storytold/cadcraft/commit/56542407292b911ad30523cfcfa987923fe2f8e2) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [6d09d3770f](https://github.com/storytold/deckcraft/commit/6d09d3770f82364320f1f098a7e74bd9b2ad3ac4) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [2d8207ed3c](https://github.com/storytold/designcraft/commit/2d8207ed3c205f52aa46080ae183fef7692efb7e) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [fa9cf47175](https://github.com/storytold/effectcraft/commit/fa9cf47175834162806b6b03d83fb2f4bdaeeafa) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [c49efe070f](https://github.com/storytold/filmcraft/commit/c49efe070fab862df6c21d61a503605eb6f07105) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [55b5a45af7](https://github.com/storytold/gridcraft/commit/55b5a45af791efd6e0edbd5fdba433d3de729ff0) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [59a5d00c60](https://github.com/storytold/lightcraft/commit/59a5d00c608dd4dc4b3c0d9cd4e914e81aa86ce5) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [20eebc1c33](https://github.com/storytold/pdfcraft/commit/20eebc1c33ff7e6f42ac1c56bf95192f628a9d97) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [1f7fc0f536](https://github.com/storytold/photocraft/commit/1f7fc0f53696e54da79b95015abb37d7c22cdd1a) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [e82c1689b3](https://github.com/storytold/soundcraft/commit/e82c1689b31ae4941472068ebcd37b12bd4cecf0) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [9997ff0215](https://github.com/storytold/vectorcraft/commit/9997ff021503e4f323f084fb7e64618d209d0126) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [c4fafc2154](https://github.com/storytold/wordcraft/commit/c4fafc215490a0186ee750454b1ce73a9823daed) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [af8d35e6a0](https://github.com/storytold/craft-launcher/commit/af8d35e6a08be090f1ab1607c7c0fdd56a725754) |
