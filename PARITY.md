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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [9b2308b138](https://github.com/storytold/cadcraft/commit/9b2308b138e90a01d28d46c84021a74b1ae7ef4a) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [1dc69720db](https://github.com/storytold/deckcraft/commit/1dc69720db64323370dda061226becb09ad79fd4) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [a7f3239cfb](https://github.com/storytold/designcraft/commit/a7f3239cfb54415c218e87bf706a47c146c0f197) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [08627d4afa](https://github.com/storytold/effectcraft/commit/08627d4afa39f3ed09fb0fee03ab8d19aab35fb1) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [c2f65871e9](https://github.com/storytold/filmcraft/commit/c2f65871e998cca4700a74dbcf9348cab51fb06b) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [f1894c6da6](https://github.com/storytold/gridcraft/commit/f1894c6da64d4326022ae644c767581816a23c2c) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [f34c7a8a89](https://github.com/storytold/lightcraft/commit/f34c7a8a89e0d1bade0a6889185f40acecc84a9f) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [f0c9901c39](https://github.com/storytold/pdfcraft/commit/f0c9901c398372782a4eb001c95638289a5edab0) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [2a952f164c](https://github.com/storytold/photocraft/commit/2a952f164c0b4cf936b1c6497db10dd095341c00) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [f3a19480d8](https://github.com/storytold/soundcraft/commit/f3a19480d8cc2d7a98cda492f5464d8c54558c9f) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [06b33b0d2d](https://github.com/storytold/vectorcraft/commit/06b33b0d2d49e1d36b8f864e56fd89790a6a804a) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [eae2421761](https://github.com/storytold/wordcraft/commit/eae242176108d328c5dd81fbc10370c51ab90706) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [307efcf45a](https://github.com/storytold/craft-launcher/commit/307efcf45a080af9643e750d43034eb25540cef8) |
