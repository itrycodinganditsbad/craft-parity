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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [a3d9348f63](https://github.com/storytold/cadcraft/commit/a3d9348f63c2ad35391cc93980dc7ee7366f2ad6) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [0623b43032](https://github.com/storytold/deckcraft/commit/0623b430324cf4c3d1962e6ae7885cf30c011c5e) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [32a1dbc734](https://github.com/storytold/designcraft/commit/32a1dbc7345fe14e010a36c691f1095562c387a9) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [544eebf904](https://github.com/storytold/effectcraft/commit/544eebf90486c654c0a415c1d8547df1bad884a7) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [5d05264820](https://github.com/storytold/filmcraft/commit/5d05264820ca256e62380ad01a86008c4baf9768) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [7102429a9a](https://github.com/storytold/gridcraft/commit/7102429a9a2fd9f6fba8e06238a1cce5807fb1b0) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [f6b57b0b59](https://github.com/storytold/lightcraft/commit/f6b57b0b59f45f3cfae1fb8d26a834e336be5a11) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [b6cbf9bb30](https://github.com/storytold/pdfcraft/commit/b6cbf9bb300424b8c14f4d2b62f5e7407e8b6805) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [1372bae3ea](https://github.com/storytold/photocraft/commit/1372bae3ea5bd9525811d974efeedbc64edad7b7) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [ef58a90f4a](https://github.com/storytold/soundcraft/commit/ef58a90f4a418a282a15fc71d2e03c3d8e8f40fa) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [1eaf678083](https://github.com/storytold/vectorcraft/commit/1eaf6780832cb9552441df953b91f7c46e7a562d) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [d102b2f08b](https://github.com/storytold/wordcraft/commit/d102b2f08b78904d6dd7ee1e9612c6e74868ab13) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [56b3090924](https://github.com/storytold/artcraft/commit/56b30909244e432784348f4346ec8074779c04b6) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [f4eb7d4358](https://github.com/storytold/craft-launcher/commit/f4eb7d435856653ca3e80717397486c195faed72) |
