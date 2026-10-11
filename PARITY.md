# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ❌ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ❌ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: gridcraft, lightcraft, photocraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

### macOS DMG: branded window background and icon layout

Without it the disk image opens as a plain Finder window with no hint to drag the app into
Applications.

Missing in: craft-launcher. Port from [storytold/photocraft#1201](https://github.com/storytold/photocraft/pull/1201), [storytold/wordcraft#21](https://github.com/storytold/wordcraft/pull/21).

### Desktop: log to a rotating log file as well as standard error

An app started from the Dock, Start menu or a launcher has no terminal, so its log records are lost
and a crash report comes without a log. The fix keeps the last few runs' logs in the app's data
folder, capped in size.

Missing in: effectcraft, craft-launcher. Port from [storytold/pdfcraft#239](https://github.com/storytold/pdfcraft/pull/239), [storytold/photocraft#1143](https://github.com/storytold/photocraft/pull/1143), [storytold/wordcraft#17](https://github.com/storytold/wordcraft/pull/17).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [54ea5e743a](https://github.com/storytold/cadcraft/commit/54ea5e743a29acd8f3178cfaa82fb74a38c2de72) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [be14248577](https://github.com/storytold/deckcraft/commit/be14248577b92059eb2b8c0c9460cceffbc1f01d) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [ca343e8985](https://github.com/storytold/designcraft/commit/ca343e8985dc4d92dd65cf77a8198723b5f8a9eb) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [67434e9038](https://github.com/storytold/effectcraft/commit/67434e90388b6dca4417555346fa6850b6f96033) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [0800a8b0aa](https://github.com/storytold/filmcraft/commit/0800a8b0aa4a703ba42172f415b45a61f8d8a5e1) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [a5137872c1](https://github.com/storytold/gridcraft/commit/a5137872c15fac7bf260fa2d25ea8f7e66b19dc2) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [76b5377868](https://github.com/storytold/lightcraft/commit/76b5377868d912db69a777bda253a5b1b96b22bd) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [3d2bdfe71a](https://github.com/storytold/pdfcraft/commit/3d2bdfe71ada10f6bc39afc680dce61bbf080cbc) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [72bf3e13b6](https://github.com/storytold/photocraft/commit/72bf3e13b6c897e8e3a4b91e214d92919cbcd382) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [240aa848a0](https://github.com/storytold/soundcraft/commit/240aa848a04811ffff75cf193e69d9d4c404ae68) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [2ac3ba2b86](https://github.com/storytold/vectorcraft/commit/2ac3ba2b866ad9d34b2252dd8024f73e34f4b2b8) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [2e9d4a22ce](https://github.com/storytold/wordcraft/commit/2e9d4a22ced1fc98201aed09bc380f652a262aa0) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [6e7a122866](https://github.com/storytold/craft-launcher/commit/6e7a122866591c067344687c45c92f735a1d189b) |
