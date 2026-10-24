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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [b9f96d3dd3](https://github.com/storytold/cadcraft/commit/b9f96d3dd3630a8de2cb7885d66ded334682918b) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [f86928d0d5](https://github.com/storytold/deckcraft/commit/f86928d0d50e6fdee166d846ea1a3477ae2658ce) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [c313c53497](https://github.com/storytold/designcraft/commit/c313c53497619b2c66d8fe0fd8c7dd40f61f50ca) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [eb5108786b](https://github.com/storytold/effectcraft/commit/eb5108786b90bf994a6abe4e6c0b2614c6aeaa29) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [552f008d27](https://github.com/storytold/filmcraft/commit/552f008d2757e307194256b3e672c9c1de4fa9b2) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [f284f6a825](https://github.com/storytold/gridcraft/commit/f284f6a8256aced973d41c9e171f0a80e4002067) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [23f54803e6](https://github.com/storytold/lightcraft/commit/23f54803e617ef4aa45b6fec23fc467ef4c2449d) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [c894928e6f](https://github.com/storytold/pdfcraft/commit/c894928e6fdfd3861795e43b1446c6cfc23cc9ab) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [1c3db2e5c3](https://github.com/storytold/photocraft/commit/1c3db2e5c35a4f8c544011e57c8026ccb9741926) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [93800e0764](https://github.com/storytold/soundcraft/commit/93800e0764ca62435ee66a4770f355df954b7fcd) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [75dacb4fec](https://github.com/storytold/vectorcraft/commit/75dacb4fec73dac539538c91f92d41e09b255d23) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [cf2fe4733e](https://github.com/storytold/wordcraft/commit/cf2fe4733e3d308173afda81a74f6a2a81e49018) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [c0fa1c69f4](https://github.com/storytold/craft-launcher/commit/c0fa1c69f452bab61ec3fd806646ef4daff9912a) |
