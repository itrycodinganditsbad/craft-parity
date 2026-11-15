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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [7064a685a8](https://github.com/storytold/cadcraft/commit/7064a685a806a8b134aed65197e96d5dad5d8a0c) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [a4674dbd1c](https://github.com/storytold/deckcraft/commit/a4674dbd1c03ee981b0ebc415a4b3a829479c318) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [b2518507b5](https://github.com/storytold/designcraft/commit/b2518507b5f192bac04e22022589a5fd75bdcbaf) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [d6c9ad6643](https://github.com/storytold/effectcraft/commit/d6c9ad664342c19e1ed473b4356292647117be62) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [01a36c1c49](https://github.com/storytold/filmcraft/commit/01a36c1c493bc2f696383ed098fa1f826e5a0854) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [8bde636f27](https://github.com/storytold/gridcraft/commit/8bde636f272cb5ac1d9b945cecf17791febe605d) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [fa5c438b3d](https://github.com/storytold/lightcraft/commit/fa5c438b3dbcb81dcc02b08a522e5ed2943cf20b) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [070432ca44](https://github.com/storytold/pdfcraft/commit/070432ca4490737cc3116efb508bda5bbad3392c) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [5bdbcc1181](https://github.com/storytold/photocraft/commit/5bdbcc1181bb17134430fb927ec6945e7d3c6a31) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [2688ccac88](https://github.com/storytold/soundcraft/commit/2688ccac8859c45cc0830fb193c5979952507b16) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [99ae951f3b](https://github.com/storytold/vectorcraft/commit/99ae951f3ba9db1110f7e3a976558a20702ce848) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [bc0e51ee4d](https://github.com/storytold/wordcraft/commit/bc0e51ee4db1f23940ac30fc626da02c09541adc) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [84a6a7dcf5](https://github.com/storytold/artcraft/commit/84a6a7dcf5ccd5f8f577e5dafe84494532cc14fb) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [b24238a7d6](https://github.com/storytold/craft-launcher/commit/b24238a7d61b324e5ff9b7ec7f95bf2d2a4a6c14) |
