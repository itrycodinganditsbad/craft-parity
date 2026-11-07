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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [865fa9afb0](https://github.com/storytold/cadcraft/commit/865fa9afb047e6a5053b8f51c3ad4feef90804d2) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [170c5591b5](https://github.com/storytold/deckcraft/commit/170c5591b5d73f8e09827eb6d418e5a1d1ab5de4) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [e692677a18](https://github.com/storytold/designcraft/commit/e692677a180ed1200e9fa52d17016b188a61369b) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [bb2d2005ce](https://github.com/storytold/effectcraft/commit/bb2d2005cee68458a10ae2cc32d609d070b21744) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [da535c1843](https://github.com/storytold/filmcraft/commit/da535c18433dbe046553733b0ab08ce9bd736a51) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [6a8eb4ff42](https://github.com/storytold/gridcraft/commit/6a8eb4ff42cfa88b5bdd3eec13c6db3874d95214) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [8a0b91a95c](https://github.com/storytold/lightcraft/commit/8a0b91a95c30618b690374dd2db9d0684ebc9b23) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [f42fda7342](https://github.com/storytold/pdfcraft/commit/f42fda734243b66f54db87923d7c981d5221419e) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [eb3b17b765](https://github.com/storytold/photocraft/commit/eb3b17b76568dc6923d635d74534fb01aa177d88) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [bb0e070ab9](https://github.com/storytold/soundcraft/commit/bb0e070ab93d13f7c5acb5a3608e7280f89415cf) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [ff9deee46d](https://github.com/storytold/vectorcraft/commit/ff9deee46d1c59dc9ecef0b668c25e57cd80b55e) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [729cd0a5fc](https://github.com/storytold/wordcraft/commit/729cd0a5fc953bbedbfdec4f6af18ad6d1780b3b) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [79d87b2b54](https://github.com/storytold/artcraft/commit/79d87b2b54b5b27223fa428bd0acd06bdd7abaf5) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [c9c3acab54](https://github.com/storytold/craft-launcher/commit/c9c3acab542f962150ffc78b2d38a59e7a342c82) |
