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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [e707ab4d7c](https://github.com/storytold/cadcraft/commit/e707ab4d7cc10102e550ad4c5bcecef1084a9baa) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [457406a7a5](https://github.com/storytold/deckcraft/commit/457406a7a5b62425e915926c8ccb0984e61a198c) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [43a46c04bf](https://github.com/storytold/designcraft/commit/43a46c04bf8cd8c8a48a347f6deaa9613223aa54) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [549e9d8251](https://github.com/storytold/effectcraft/commit/549e9d825132986d6167c1432cd3328efcbb0278) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [cee1feb185](https://github.com/storytold/filmcraft/commit/cee1feb185e675a108f78067f71ad86a640ace3d) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [84acf6293b](https://github.com/storytold/gridcraft/commit/84acf6293b1dc1fc09837524e035d674eb4d6741) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [a4b53b28cc](https://github.com/storytold/lightcraft/commit/a4b53b28cc52b68d9f5bdca09fa5666cdad5ad5f) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [c1a41cc5c7](https://github.com/storytold/pdfcraft/commit/c1a41cc5c7c2460fb60af880af69ee28a0c71bd5) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [ed7df56a24](https://github.com/storytold/photocraft/commit/ed7df56a2455375240959bae6e5273a71d858d84) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [c0cb993408](https://github.com/storytold/soundcraft/commit/c0cb99340891d306e216582db465f3594b89dc1d) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [a067177778](https://github.com/storytold/vectorcraft/commit/a06717777828867557d759c674283304f63c04ae) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [04f6f53b8a](https://github.com/storytold/wordcraft/commit/04f6f53b8a4d8140c7bb62f949b0a6099a473da1) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [0f295d4b9c](https://github.com/storytold/craft-launcher/commit/0f295d4b9c90d15c2e38e36c322bdbc0fa943a54) |
