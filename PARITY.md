# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ❌ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ❌ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ➖ | ❌ |

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

### Windows: draw on the GPU that drives the primary display

With a low-power or high-performance preference alone, wgpu can pick a GPU that drives no monitor,
for example a Ryzen iGPU on a desktop whose screens hang off an NVIDIA card. Presenting across GPUs
blacked out every monitor for two minutes in PdfCraft (storytold/pdfcraft#378). The fix asks Windows
which adapter drives the primary display (EnumDisplayDevices) and matches it to the wgpu adapters by
PCI ids.

Missing in: cadcraft, deckcraft, designcraft, effectcraft, filmcraft, gridcraft, lightcraft, photocraft, soundcraft, wordcraft, artcraft, craft-launcher. Port from [storytold/pdfcraft#631](https://github.com/storytold/pdfcraft/pull/631), [storytold/vectorcraft#851](https://github.com/storytold/vectorcraft/pull/851).

## Exemptions

- bidi-text / artcraftx: exempt, web UI only; the browser lays out right-to-left text
- rotating-log-file / artcraft: exempt, Tauri app; logs to a file through tauri-plugin-log
- rotating-log-file / artcraftx: exempt, Tauri app; logs to a file through tauri-plugin-log

## Scanned

| App | Ref | Commit |
|---|---|---|
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [54ea5e743a](https://github.com/storytold/cadcraft/commit/54ea5e743a29acd8f3178cfaa82fb74a38c2de72) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [be14248577](https://github.com/storytold/deckcraft/commit/be14248577b92059eb2b8c0c9460cceffbc1f01d) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [ca343e8985](https://github.com/storytold/designcraft/commit/ca343e8985dc4d92dd65cf77a8198723b5f8a9eb) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [67434e9038](https://github.com/storytold/effectcraft/commit/67434e90388b6dca4417555346fa6850b6f96033) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [0800a8b0aa](https://github.com/storytold/filmcraft/commit/0800a8b0aa4a703ba42172f415b45a61f8d8a5e1) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [177e52a14d](https://github.com/storytold/gridcraft/commit/177e52a14dc1ea484b0f02df157c1093e87b569e) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [76b5377868](https://github.com/storytold/lightcraft/commit/76b5377868d912db69a777bda253a5b1b96b22bd) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [8f8e0236aa](https://github.com/storytold/pdfcraft/commit/8f8e0236aaef16a4f3ca52a074ad5d48da6498f7) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [f92f13490b](https://github.com/storytold/photocraft/commit/f92f13490b24f54869edc0b260b71d6f92f243dc) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [240aa848a0](https://github.com/storytold/soundcraft/commit/240aa848a04811ffff75cf193e69d9d4c404ae68) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [2ac3ba2b86](https://github.com/storytold/vectorcraft/commit/2ac3ba2b866ad9d34b2252dd8024f73e34f4b2b8) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [f8f1d80c38](https://github.com/storytold/wordcraft/commit/f8f1d80c389a365589e2b7a0d53770c0421ce592) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [c273a94a9a](https://github.com/storytold/craft-launcher/commit/c273a94a9a37c7a2d7a07fd6cc9ca44e3dcc6b9c) |
