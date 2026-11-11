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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [d6d16bd474](https://github.com/storytold/cadcraft/commit/d6d16bd474d3b223f5edd89180204bddf17040da) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [a55457ee53](https://github.com/storytold/deckcraft/commit/a55457ee53d23628c3df77095cf19840d634a25e) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [c64378ded9](https://github.com/storytold/designcraft/commit/c64378ded9c8d664d52dd87767cbe6ff082c8100) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [be5c98506f](https://github.com/storytold/effectcraft/commit/be5c98506f5ebc165cf0cfa78be20ab32ff1e24e) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [26e89c7558](https://github.com/storytold/filmcraft/commit/26e89c7558b677835eed7f75a6e7a2e2704e487e) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [e5ac9474df](https://github.com/storytold/gridcraft/commit/e5ac9474dfc8d0162cb81eefcfff16b73bb1ebe0) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [f786b87533](https://github.com/storytold/lightcraft/commit/f786b87533e58c6d31c7f9f2897814904448f39a) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [059dbd6e00](https://github.com/storytold/pdfcraft/commit/059dbd6e00507cdfedd755bf6b712f883507fce0) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [8783593c23](https://github.com/storytold/photocraft/commit/8783593c23ab4a724b1568c70c424c5120e1e123) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [dc4f82ea9f](https://github.com/storytold/soundcraft/commit/dc4f82ea9fd2d620c65fc74fcce94276f258b9ed) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [58c37a28e9](https://github.com/storytold/vectorcraft/commit/58c37a28e9bfe580f568a6bf0ff5c8e7a6d7ed42) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [060768f1ed](https://github.com/storytold/wordcraft/commit/060768f1ed7f310e639bc49d7af7256e129f7409) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [55e6e5cabb](https://github.com/storytold/artcraft/commit/55e6e5cabbff1466153bb6ce59a1424ed5747eb4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [f6409da7f7](https://github.com/storytold/craft-launcher/commit/f6409da7f7810b03edf4c5f40bc6e6dfc2a22f26) |
