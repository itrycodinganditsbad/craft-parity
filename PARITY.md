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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [fbb083803b](https://github.com/storytold/cadcraft/commit/fbb083803bf8e81677a2552bd386c1dffae0e46a) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [b0c77ebd6a](https://github.com/storytold/deckcraft/commit/b0c77ebd6a6fe6c9037cfbe1c8a7f397304b2205) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [7107a64d6c](https://github.com/storytold/designcraft/commit/7107a64d6c7d4506604029110130b1b0be377c35) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [53576d2232](https://github.com/storytold/effectcraft/commit/53576d2232e35501c8b27c7d7cac8ebc78660c40) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [9404f6863d](https://github.com/storytold/filmcraft/commit/9404f6863d454bbaaebcc9a6696766d398f88553) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [c0ee120ac2](https://github.com/storytold/gridcraft/commit/c0ee120ac2d9b59630f5abf1151dd9b471fe2926) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [f72ba11a29](https://github.com/storytold/lightcraft/commit/f72ba11a2939653c866c07cf9189845ec31f1a50) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [dd6c625274](https://github.com/storytold/pdfcraft/commit/dd6c625274e00220f6eab2df0ad0f5ea2ef9a204) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [b044610e5d](https://github.com/storytold/photocraft/commit/b044610e5da43de43142760cbe56badfe5c096da) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [ae652ab34b](https://github.com/storytold/soundcraft/commit/ae652ab34bd4330ec92f223381f5d440991a7abb) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [124e3cc2f1](https://github.com/storytold/vectorcraft/commit/124e3cc2f1ff212e19bc9db1ccb0b91ea6b70343) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [b02baeba43](https://github.com/storytold/wordcraft/commit/b02baeba43129f7acffbcc3f3b7b5d00dd6e94df) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [84a6a7dcf5](https://github.com/storytold/artcraft/commit/84a6a7dcf5ccd5f8f577e5dafe84494532cc14fb) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [ba56c4587f](https://github.com/storytold/craft-launcher/commit/ba56c4587f5701db5722451700f28491da79e034) |
