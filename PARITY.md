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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [e86d895ba1](https://github.com/storytold/cadcraft/commit/e86d895ba1086a7efc2a83048348f89098282d08) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [a0eeb341d2](https://github.com/storytold/deckcraft/commit/a0eeb341d2790be1056e472774cb379d8bf16566) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [7ca39c6c10](https://github.com/storytold/designcraft/commit/7ca39c6c10f123d0cd0176eda1511e59667c89d0) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [991904888a](https://github.com/storytold/effectcraft/commit/991904888a20b7da72b2a4880b21c7d912154736) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [a504f3c5ad](https://github.com/storytold/filmcraft/commit/a504f3c5adc4cd60d9c1120916cddb50c9b24b86) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [17b9f6ff01](https://github.com/storytold/gridcraft/commit/17b9f6ff0125209fe707efc919690632ff49cb0b) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [31ec62cd03](https://github.com/storytold/lightcraft/commit/31ec62cd0387abecd8f2a2da034aee591447897d) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [418ac6a2af](https://github.com/storytold/pdfcraft/commit/418ac6a2af49b7ac2fed44ca2cae8d5d1d2cec0d) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [b0622d20ca](https://github.com/storytold/photocraft/commit/b0622d20cadd7e994a51ce2f48e415efbbdec24f) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [541096d2cb](https://github.com/storytold/soundcraft/commit/541096d2cb5f96a13d5b58935e45711530471480) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [8bf090b253](https://github.com/storytold/vectorcraft/commit/8bf090b25389b48aa6f9d7415de57c9ce7ccf8cb) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [b70c746f9b](https://github.com/storytold/wordcraft/commit/b70c746f9bb70f830c5377f7710fc31dd3e8f195) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [f0cb3b0439](https://github.com/storytold/craft-launcher/commit/f0cb3b0439d15cae02d806e5ddc0132fd487c89c) |
