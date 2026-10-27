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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [d864a08b01](https://github.com/storytold/cadcraft/commit/d864a08b013353143086bd8bb568c26cb6256796) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [ee875abffa](https://github.com/storytold/deckcraft/commit/ee875abffa1a31cb760c843bb29e79391a90a766) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [50650f74d7](https://github.com/storytold/designcraft/commit/50650f74d7328745cfcdd16098313a9fc31b23d5) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [01c904b041](https://github.com/storytold/effectcraft/commit/01c904b041064ca8ed03e62bffd07f974dee07f6) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [10d626ccc4](https://github.com/storytold/filmcraft/commit/10d626ccc4888e0f020f9ca314c2bc9148ded4f4) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [45f5ab18e4](https://github.com/storytold/gridcraft/commit/45f5ab18e4e57bf50ee88d2526deb4b87c5b24cb) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [662df66f1c](https://github.com/storytold/lightcraft/commit/662df66f1c08bdb1b1f1d0c1c638c4cef936d0b0) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [1dd2737b47](https://github.com/storytold/pdfcraft/commit/1dd2737b47a9c1c9562f0af563053d1bf510e4bf) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [7ac7b8b106](https://github.com/storytold/photocraft/commit/7ac7b8b106acccf6826d45ed5c9e481dfe341035) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [60087e0f46](https://github.com/storytold/soundcraft/commit/60087e0f46032118bf309ed9ea161a9b4a872713) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [443b31b13d](https://github.com/storytold/vectorcraft/commit/443b31b13df3a7ab611c2fe1b95e73898ed74fdd) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [2cf1d6c7ae](https://github.com/storytold/wordcraft/commit/2cf1d6c7ae94723a45bd6e7e8c884fe375148750) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [7aafad661a](https://github.com/storytold/craft-launcher/commit/7aafad661a11594a70ae949471d481f30d8fe64a) |
