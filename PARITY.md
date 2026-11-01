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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [d7ef7da872](https://github.com/storytold/cadcraft/commit/d7ef7da872fbd4158a5b2fc493f2da191f461d03) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [5fb694d991](https://github.com/storytold/deckcraft/commit/5fb694d9917d232fbf5b2e0f42ac15d73febde7c) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [008f519c46](https://github.com/storytold/designcraft/commit/008f519c46ae82c281bea921e297b3f2e37d99f2) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [e71dd629c5](https://github.com/storytold/effectcraft/commit/e71dd629c56c78a82959ee50c271da9209c55b80) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [9c6d66114c](https://github.com/storytold/filmcraft/commit/9c6d66114c71179f66d21933b6bf6b88311adf46) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [5e415cdd34](https://github.com/storytold/gridcraft/commit/5e415cdd34d287b90a4a9df67eb4f532f6f72d96) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [07301cbd1e](https://github.com/storytold/lightcraft/commit/07301cbd1e1cab5c152228891d9c9ed87e263003) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [47f42ba07b](https://github.com/storytold/pdfcraft/commit/47f42ba07b61553da50ca2df91b070519bd3386e) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [97a83f074f](https://github.com/storytold/photocraft/commit/97a83f074f5407841a1444ee3f9be478ce03368b) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [7169dc492e](https://github.com/storytold/soundcraft/commit/7169dc492eab3d068baa05bd2b0fd303409a86ac) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [05d5f99fd7](https://github.com/storytold/vectorcraft/commit/05d5f99fd7e7dc00db3a9754d594b1214945d855) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [a88e005256](https://github.com/storytold/wordcraft/commit/a88e005256087285bc7d34310cc0f5f932bc8562) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [ffa3440c15](https://github.com/storytold/craft-launcher/commit/ffa3440c153f08c015f23ebcf9dfe0ec8025d0aa) |
