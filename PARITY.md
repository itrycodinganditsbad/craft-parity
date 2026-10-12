# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: gridcraft, lightcraft, photocraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [4cb9e8a535](https://github.com/storytold/cadcraft/commit/4cb9e8a535b2fc51799dc454099c644c235c3efd) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [0ec0cc5e81](https://github.com/storytold/deckcraft/commit/0ec0cc5e815caea982a7deec51bc437f3ba187ad) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [3384224419](https://github.com/storytold/designcraft/commit/3384224419824d5284c68035eb30fad6e3d89c3f) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [9efe75330c](https://github.com/storytold/effectcraft/commit/9efe75330c090748df1818348942d4abc30c9b1f) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [0541d4d182](https://github.com/storytold/filmcraft/commit/0541d4d182d0bb755a2633ef66824cdadb280c09) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [b3acae6b77](https://github.com/storytold/gridcraft/commit/b3acae6b7782ea236d0fb87a0121fd1ba2e14c2b) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [25ef65c81e](https://github.com/storytold/lightcraft/commit/25ef65c81eef81833a50ff5b61f8676d80611b06) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [87b448b664](https://github.com/storytold/pdfcraft/commit/87b448b664e1b3c960b479cc0f2c2e7a82c7ac71) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [3f72158f0a](https://github.com/storytold/photocraft/commit/3f72158f0af959674651bfb8086ee000bf8f5157) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [df0c4790b7](https://github.com/storytold/soundcraft/commit/df0c4790b72972cbb967d5af17228e782dbee420) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [2a0a279c93](https://github.com/storytold/vectorcraft/commit/2a0a279c93efd82ea64787363f361c6d1b6b9c04) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [da5da66e51](https://github.com/storytold/wordcraft/commit/da5da66e51b8937f18ef9542f7fb3dd040358907) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [1aaf7b27be](https://github.com/storytold/craft-launcher/commit/1aaf7b27be2dba970d1a77f96aef6aad52352730) |
