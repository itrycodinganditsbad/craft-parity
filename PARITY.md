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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [6ca414c344](https://github.com/storytold/cadcraft/commit/6ca414c344777128a7af7536e20e4c0b029f2254) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [daeedd5d19](https://github.com/storytold/deckcraft/commit/daeedd5d1904d79e2f0fa44c5e1e00982b1508b1) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [0e85e9e5ec](https://github.com/storytold/designcraft/commit/0e85e9e5ec6913c5189d2d3e575ca7834473f0de) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [a439b1bb19](https://github.com/storytold/effectcraft/commit/a439b1bb19051c339e46c59166f39fac25812406) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [d24ce15e29](https://github.com/storytold/filmcraft/commit/d24ce15e292d9b1d658327e088bedaba39580f2c) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [6ea9f538a7](https://github.com/storytold/gridcraft/commit/6ea9f538a74353d85872de2202179e8204b64152) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [eab130ecee](https://github.com/storytold/lightcraft/commit/eab130ecee86ac6eabe75b457dc59452306a1b07) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [96fe832e12](https://github.com/storytold/pdfcraft/commit/96fe832e12c6de3ba88005496ba3d57cc3039fa9) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [384ff664d6](https://github.com/storytold/photocraft/commit/384ff664d69128880f1715078d456484133940d0) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [1db9563379](https://github.com/storytold/soundcraft/commit/1db9563379428e58a4eb0032d3942c823c0b49e9) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [f1ee91fcd9](https://github.com/storytold/vectorcraft/commit/f1ee91fcd9de01472f6d5d8cda4e4f8e0f4a5af3) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [d4bbe8ba9a](https://github.com/storytold/wordcraft/commit/d4bbe8ba9aeafa5aeb4bf1fdfbc710a168b41352) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [58f5944fe6](https://github.com/storytold/artcraft/commit/58f5944fe66e4cfdfd69fc3eebcc67e38f75a1a9) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [1a3ce2033b](https://github.com/storytold/craft-launcher/commit/1a3ce2033b24d411fa15ba7de0b021efb1a19804) |
