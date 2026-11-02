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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [ebc67fa81d](https://github.com/storytold/cadcraft/commit/ebc67fa81dccb0d89c157babb27e7d5c3a5c1177) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [0d45a64db2](https://github.com/storytold/deckcraft/commit/0d45a64db23baa0a4c8f9f7ac14c59c91b40fec9) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [cc3c70d4be](https://github.com/storytold/designcraft/commit/cc3c70d4be5092262b61c43b607e26b4a0b82fc6) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [9d7baea5cc](https://github.com/storytold/effectcraft/commit/9d7baea5cce3ddb4a1a33a0f1f6c3de439eddae3) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [68919ed87a](https://github.com/storytold/filmcraft/commit/68919ed87aa6a097a4d21414ec7c438883f5fe22) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [488f9982eb](https://github.com/storytold/gridcraft/commit/488f9982ebf05be9bfe581489d0edc6e3918eec7) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [ebe2477108](https://github.com/storytold/lightcraft/commit/ebe247710867f8c47512fab1ff488f17815161e5) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [f47618f59b](https://github.com/storytold/pdfcraft/commit/f47618f59b522a9cc80c061e0d540eef8e494f5d) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [5d9c57e60d](https://github.com/storytold/photocraft/commit/5d9c57e60db966382abda2638d0ccbb7489d8ee7) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [8444edf075](https://github.com/storytold/soundcraft/commit/8444edf075eae24ad37b128675c939fd5d370a60) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [4ee0ef231a](https://github.com/storytold/vectorcraft/commit/4ee0ef231aa31f50192f0072554fa390957532bf) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [242e850aaa](https://github.com/storytold/wordcraft/commit/242e850aaad6bf38e1cd2be4122cc7cb9e5a15a8) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [ec205366eb](https://github.com/storytold/craft-launcher/commit/ec205366ebe42635e155b9ab246f036052fef406) |
