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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [7a9ec9bae3](https://github.com/storytold/cadcraft/commit/7a9ec9bae323bdb548ff4b647a971f5c448e59c2) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [48e2203123](https://github.com/storytold/deckcraft/commit/48e22031239338adc7b9c5a48206f06f5dc07839) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [19c9875770](https://github.com/storytold/designcraft/commit/19c98757704bdeec3e154f13c89422eb4df32759) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [3e0c77ca68](https://github.com/storytold/effectcraft/commit/3e0c77ca6860b94d13f73d96c2a5222e4e353102) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [4a56be9cff](https://github.com/storytold/filmcraft/commit/4a56be9cff419a5a9b16a25259c97dc823ca8bae) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [21920d3d61](https://github.com/storytold/gridcraft/commit/21920d3d612605d1c71c38fb8b2e70926789c797) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [b3b49987d4](https://github.com/storytold/lightcraft/commit/b3b49987d4b2c99f582b80ae91b2ec42ab24d14e) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [f750589812](https://github.com/storytold/pdfcraft/commit/f75058981254114c1141f406f66fbddee4cd5368) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [6ab9915718](https://github.com/storytold/photocraft/commit/6ab99157189f2b9ea79e2404967151d0ba5eb2db) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [76cc2aeac6](https://github.com/storytold/soundcraft/commit/76cc2aeac657aa971303ee522f36cafe86b3ef93) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [0a702cc0a5](https://github.com/storytold/vectorcraft/commit/0a702cc0a5f36346b420b7effc5698ef36b05b2d) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [661a790540](https://github.com/storytold/wordcraft/commit/661a790540de039b6902b5cc9ffe5d49024bf3c7) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [79d87b2b54](https://github.com/storytold/artcraft/commit/79d87b2b54b5b27223fa428bd0acd06bdd7abaf5) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [74f265a349](https://github.com/storytold/craft-launcher/commit/74f265a34961c7c9fa8ae45dbaf5553e8b7da6df) |
