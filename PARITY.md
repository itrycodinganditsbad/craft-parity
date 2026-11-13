# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: lightcraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [bf581c2376](https://github.com/storytold/cadcraft/commit/bf581c2376d29efb2db73a1ccf43796a9c11e3c9) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [ef9ac9ac7a](https://github.com/storytold/deckcraft/commit/ef9ac9ac7a1f610656144981b53688efdb9ef518) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [f24fc1e72b](https://github.com/storytold/designcraft/commit/f24fc1e72b8ed8affcd6bd189a4d90b82c54b652) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [69bce0ec66](https://github.com/storytold/effectcraft/commit/69bce0ec66f4cb2b9a169b6a19f4d2363faadad3) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [4d82e518c5](https://github.com/storytold/filmcraft/commit/4d82e518c507d2e699c112314cd1384f676b9656) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [c0ee120ac2](https://github.com/storytold/gridcraft/commit/c0ee120ac2d9b59630f5abf1151dd9b471fe2926) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [0b64e349c0](https://github.com/storytold/lightcraft/commit/0b64e349c02b25fd607c7877ab4d609e07611e8d) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [fd4d60ff4a](https://github.com/storytold/pdfcraft/commit/fd4d60ff4ae0e2f383ee57218ea554f37dd272d6) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [0b15ad2401](https://github.com/storytold/photocraft/commit/0b15ad2401da2f6ee864a02ec7408e1877f70448) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [6d525c73d4](https://github.com/storytold/soundcraft/commit/6d525c73d411e0a184086920a5c129f17f208438) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [1dbdbce91f](https://github.com/storytold/vectorcraft/commit/1dbdbce91f1aecdfcf4a0b351546261faaa52f70) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [c537ed96b9](https://github.com/storytold/wordcraft/commit/c537ed96b9f1d31402e20b0e1a356368138f78ed) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [84a6a7dcf5](https://github.com/storytold/artcraft/commit/84a6a7dcf5ccd5f8f577e5dafe84494532cc14fb) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [6a6f6dcb3a](https://github.com/storytold/craft-launcher/commit/6a6f6dcb3ac2021428965552981cb19909dc9e39) |
