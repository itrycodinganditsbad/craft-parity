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
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [1629e4d953](https://github.com/storytold/deckcraft/commit/1629e4d9533f1864b0fddd8ea976723eff03f5f6) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [3930cff80d](https://github.com/storytold/designcraft/commit/3930cff80da27a4bdbc4b6ab27a13609bfcf6a93) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [4c38f78307](https://github.com/storytold/effectcraft/commit/4c38f78307e21a07021333c8a109e6fc49d66c3b) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [93a88a839a](https://github.com/storytold/filmcraft/commit/93a88a839aae370740b09434fdec410abb8c24ca) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [d60709a173](https://github.com/storytold/gridcraft/commit/d60709a1738d4f02c7f507848441306c0aacf692) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [27b755a93c](https://github.com/storytold/lightcraft/commit/27b755a93c8091a6f0c71151184a88f0b90144ce) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [271e479c53](https://github.com/storytold/pdfcraft/commit/271e479c53723b1b294c866d5cb2f27604601b29) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [01b79958a7](https://github.com/storytold/photocraft/commit/01b79958a7b73c3d840b76492069e9635f9f7394) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [b47a58653c](https://github.com/storytold/soundcraft/commit/b47a58653c291f320179d20db3eafb40e70b88a7) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [0f8040ef59](https://github.com/storytold/vectorcraft/commit/0f8040ef59b88d33dd97563dbb87e8efedf046ff) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [1a56d041de](https://github.com/storytold/wordcraft/commit/1a56d041defe28f0d35de501ba7e40268a549d26) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [69f53c2f03](https://github.com/storytold/artcraft/commit/69f53c2f0358a91a5e35e5a7b3a91677320c3112) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [fba78de478](https://github.com/storytold/artcraftx/commit/fba78de4782fe727739b4e34ed8b547cbd601632) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [b2de8c8bdd](https://github.com/storytold/craft-launcher/commit/b2de8c8bddac0c14f357f58e1933586ff9e2a4e2) |
