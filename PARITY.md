# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ❌ | ✅ | ❌ | ❌ | ✅ | ❌ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: gridcraft, lightcraft, photocraft, soundcraft. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

### Desktop: log to a rotating log file as well as standard error

An app started from the Dock, Start menu or a launcher has no terminal, so its log records are lost
and a crash report comes without a log. The fix keeps the last few runs' logs in the app's data
folder, capped in size.

Missing in: effectcraft. Port from [storytold/pdfcraft#239](https://github.com/storytold/pdfcraft/pull/239), [storytold/photocraft#1143](https://github.com/storytold/photocraft/pull/1143), [storytold/wordcraft#17](https://github.com/storytold/wordcraft/pull/17).

### Windows: draw on the GPU that drives the primary display

With a low-power or high-performance preference alone, wgpu can pick a GPU that drives no monitor,
for example a Ryzen iGPU on a desktop whose screens hang off an NVIDIA card. Presenting across GPUs
blacked out every monitor for two minutes in PdfCraft (storytold/pdfcraft#378). The fix asks Windows
which adapter drives the primary display (EnumDisplayDevices) and matches it to the wgpu adapters by
PCI ids.

Missing in: cadcraft, deckcraft, designcraft, effectcraft, filmcraft, gridcraft, lightcraft, photocraft, soundcraft, wordcraft. Port from [storytold/pdfcraft#631](https://github.com/storytold/pdfcraft/pull/631), [storytold/vectorcraft#851](https://github.com/storytold/vectorcraft/pull/851).

## Scanned

| App | Ref | Commit |
|---|---|---|
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [8f59410c33](https://github.com/storytold/cadcraft/commit/8f59410c33e5640b5010124c93e5ebcfd35f404f) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [be14248577](https://github.com/storytold/deckcraft/commit/be14248577b92059eb2b8c0c9460cceffbc1f01d) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [ca343e8985](https://github.com/storytold/designcraft/commit/ca343e8985dc4d92dd65cf77a8198723b5f8a9eb) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [67434e9038](https://github.com/storytold/effectcraft/commit/67434e90388b6dca4417555346fa6850b6f96033) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [0800a8b0aa](https://github.com/storytold/filmcraft/commit/0800a8b0aa4a703ba42172f415b45a61f8d8a5e1) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [a0cb8f2ca2](https://github.com/storytold/gridcraft/commit/a0cb8f2ca286f82c60b68dc4a24a268963288be8) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [76b5377868](https://github.com/storytold/lightcraft/commit/76b5377868d912db69a777bda253a5b1b96b22bd) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [046df9dfe4](https://github.com/storytold/pdfcraft/commit/046df9dfe4269711f6b4774006e82e4379dbf9f9) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [f8af27d261](https://github.com/storytold/photocraft/commit/f8af27d2618c48575f80a71a79cd795931b5020f) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [240aa848a0](https://github.com/storytold/soundcraft/commit/240aa848a04811ffff75cf193e69d9d4c404ae68) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [bab78767ca](https://github.com/storytold/vectorcraft/commit/bab78767ca3c4d8652e5760bde8cb177c9ce4851) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [3042e5251e](https://github.com/storytold/wordcraft/commit/3042e5251e64e3ba984acdbb0ba1947313ea8df5) |
