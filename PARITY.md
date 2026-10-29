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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [336243e0af](https://github.com/storytold/cadcraft/commit/336243e0afb3019feee58d0d4fc0d218f25530e7) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [f893f6af7d](https://github.com/storytold/deckcraft/commit/f893f6af7d36a5409fd804e81a34fd895cfefedf) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [855d606325](https://github.com/storytold/designcraft/commit/855d6063250b31b30619eaeac15a07e7d37049f7) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [990fa668cd](https://github.com/storytold/effectcraft/commit/990fa668cd3898935de91529425bb691a4dd3cb1) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [8d925e9df3](https://github.com/storytold/filmcraft/commit/8d925e9df32d8755f22bd2635e400a6b3e93f69c) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [2ff751791b](https://github.com/storytold/gridcraft/commit/2ff751791b7e5f3454f1cbd8bf87f04fac9d8649) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [77c0a6adfa](https://github.com/storytold/lightcraft/commit/77c0a6adfa7db6e4f79b80843c0c578e5f16dee1) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [3109bb2359](https://github.com/storytold/pdfcraft/commit/3109bb2359759e24ef490e1cdbdba00722b20972) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [5f9b439187](https://github.com/storytold/photocraft/commit/5f9b4391874aabc9deb2fcb905267613e4d93ec8) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [8ca6939981](https://github.com/storytold/soundcraft/commit/8ca6939981a1cf5d4b0f39cbcd2b0c6a3dedadb0) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [5cb5f4514d](https://github.com/storytold/vectorcraft/commit/5cb5f4514d3cb59997c26c8a0932ef9a53d5bbce) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [aadb077e07](https://github.com/storytold/wordcraft/commit/aadb077e072b7de05f724e359d8db82e151f125c) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [b827fed74a](https://github.com/storytold/craft-launcher/commit/b827fed74ad1bb3ca2f4456edecdf4d27c832a0c) |
