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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [b04273e897](https://github.com/storytold/cadcraft/commit/b04273e8974245e1c59ec57eb5ad86cec7a200eb) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [82ec8137dc](https://github.com/storytold/deckcraft/commit/82ec8137dc39f29b0f51b18630b409d655066fce) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [f1bdec7fee](https://github.com/storytold/designcraft/commit/f1bdec7fee7a5dfb58c35b6e056b3f9c9bd5110e) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [d025b59e6d](https://github.com/storytold/effectcraft/commit/d025b59e6d77ae038712bb9431c3a548e5c37f17) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [57dc82ea20](https://github.com/storytold/filmcraft/commit/57dc82ea2046c2ec479ab4d4d8167d90e5500b49) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [92cdc2a975](https://github.com/storytold/gridcraft/commit/92cdc2a975b6e014da6ff25477232b500e3c1cf6) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [ac1799db38](https://github.com/storytold/lightcraft/commit/ac1799db38661e07a855653f86feb3e66081102e) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [01ba3ac9d1](https://github.com/storytold/pdfcraft/commit/01ba3ac9d126dd4345c8795e74b46e311f18cc7d) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [9779daf357](https://github.com/storytold/photocraft/commit/9779daf3577eb1d831fa078e3094a7c70b7b23cf) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [aa250cc1d7](https://github.com/storytold/soundcraft/commit/aa250cc1d740013cf26cc449e53a20653ed67e2e) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [94abb6ae9c](https://github.com/storytold/vectorcraft/commit/94abb6ae9cd4db37392cbc7f95490a27b25e424e) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [ffb155a032](https://github.com/storytold/wordcraft/commit/ffb155a0321bba08da495efda66495104c1c8b9a) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [1f969a9224](https://github.com/storytold/artcraft/commit/1f969a9224934a55417daf10359a6fb1458b63f4) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [1f168df0af](https://github.com/storytold/craft-launcher/commit/1f168df0afcf107e4eacac2ed4fb2214136c9fda) |
