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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [1977a493c7](https://github.com/storytold/cadcraft/commit/1977a493c7d8340249185126744c5143f096dc86) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [b47195feec](https://github.com/storytold/deckcraft/commit/b47195feec0f0fe4220d8943ad1c8add5dad8072) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [e74208dd7e](https://github.com/storytold/designcraft/commit/e74208dd7e0c801d15dfdf926d64cfe5e3b346d3) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [9581b53c3f](https://github.com/storytold/effectcraft/commit/9581b53c3f8945683252af0eb5fac79e0e43aed4) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [abf44fda90](https://github.com/storytold/filmcraft/commit/abf44fda9015f4b7a96dc338f6042499bf8e9d9f) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [ac8da7d49c](https://github.com/storytold/gridcraft/commit/ac8da7d49cbeb4be6aa1ad5869f3d29fe2da8cd9) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [23e181283c](https://github.com/storytold/lightcraft/commit/23e181283cb448205d865e8cd288b56afe987986) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [0dc77fe4ed](https://github.com/storytold/pdfcraft/commit/0dc77fe4ed8810b9e9f25b9dd050afae662c062a) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [ff47388101](https://github.com/storytold/photocraft/commit/ff4738810134d7f7e68b49f2e4f231b686282ff3) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [52471b37e9](https://github.com/storytold/soundcraft/commit/52471b37e957feaea56dfd48932ef16d314cc74a) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [4a7cba0dd3](https://github.com/storytold/vectorcraft/commit/4a7cba0dd32e4ac249e4dc50af486b89531481aa) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [3a4770ef8c](https://github.com/storytold/wordcraft/commit/3a4770ef8ce037d2650c93e2d5764536549357c7) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [800217ed04](https://github.com/storytold/artcraft/commit/800217ed04d10fc12706c28e6e91ae2a6b286092) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [9419d8977b](https://github.com/storytold/craft-launcher/commit/9419d8977bc70f80373da8558356e4ba7387c7b2) |
