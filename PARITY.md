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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [0b32c2e6a1](https://github.com/storytold/cadcraft/commit/0b32c2e6a14c761835ea62c01e7b20845971d781) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [f34f4121c8](https://github.com/storytold/deckcraft/commit/f34f4121c8ee720ca08e5d47b912155a33504e69) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [443ea4640c](https://github.com/storytold/designcraft/commit/443ea4640ca368fcf0241b04813bd3817acd5df8) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [ef74d82902](https://github.com/storytold/effectcraft/commit/ef74d82902ba486891b02afa87e42963cc9b84cc) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [fb4cbda47a](https://github.com/storytold/filmcraft/commit/fb4cbda47a134eedabd89fe02cd59ebc69462737) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [23829d8c67](https://github.com/storytold/gridcraft/commit/23829d8c67990e6b52a27bab6a38daf83e07dd54) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [d999e382ba](https://github.com/storytold/lightcraft/commit/d999e382bafbb5d5c54482fdb1d11f6a63bfdf69) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [ec0a965074](https://github.com/storytold/pdfcraft/commit/ec0a965074f376a5bf610435948e4e5184bf3299) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [03d0c7e3b3](https://github.com/storytold/photocraft/commit/03d0c7e3b39cfb401cdbba590431141ee4c6c8fa) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [25d52c646a](https://github.com/storytold/soundcraft/commit/25d52c646aea780c8076ff966076f994ad256221) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [9e7929ee89](https://github.com/storytold/vectorcraft/commit/9e7929ee8966977e6daf200f457c570f7aaf7c51) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [9b4b948c6e](https://github.com/storytold/wordcraft/commit/9b4b948c6ecf29401e01ad58626d3c1983b83770) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [7a2935cf9e](https://github.com/storytold/artcraft/commit/7a2935cf9e3922e90e4ed5f3556b1cb107013148) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [7c7271e5f6](https://github.com/storytold/craft-launcher/commit/7c7271e5f62975326d0add2dcf31f03096be4189) |
