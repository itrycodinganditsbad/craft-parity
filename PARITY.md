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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [0cf3bdea83](https://github.com/storytold/cadcraft/commit/0cf3bdea83ebbe02fb2e674b23f07de4acfa73da) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [63831cdfa9](https://github.com/storytold/deckcraft/commit/63831cdfa9e13ad503d18e6ee210767919352128) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [3ce3a24f08](https://github.com/storytold/designcraft/commit/3ce3a24f08d03935bc8cf7cb23755aca8065558c) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [9db7f3b217](https://github.com/storytold/effectcraft/commit/9db7f3b21700fd821f215782ed1c82afe077880f) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [d643a88665](https://github.com/storytold/filmcraft/commit/d643a886650bfbf16f24762274054575aa1e8f20) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [37e41424ed](https://github.com/storytold/gridcraft/commit/37e41424ed425b33520ae98c6b8f660e518cecad) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [abb49bdd76](https://github.com/storytold/lightcraft/commit/abb49bdd76e64acd20afe2e4d7bb1cf71c548d1d) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [4164951453](https://github.com/storytold/pdfcraft/commit/416495145354828a9efefc39df381ba3ee8f864f) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [04d0212c7a](https://github.com/storytold/photocraft/commit/04d0212c7a1006b00f6ec9dd144affc98f8e7d83) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [7b339fe921](https://github.com/storytold/soundcraft/commit/7b339fe921dfc1231051cd1263d582f6da589e75) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [5b77e422ab](https://github.com/storytold/vectorcraft/commit/5b77e422ab3529b4239abcebedd377c4c706b95a) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [e05b7fe86f](https://github.com/storytold/wordcraft/commit/e05b7fe86f3b85454000dffc321de638d7009817) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [40e04793b5](https://github.com/storytold/artcraft/commit/40e04793b504147de97ba1f0659d3975167524e5) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [c6b5d33830](https://github.com/storytold/artcraftx/commit/c6b5d338303251d8bcb93ec6806b412f95127936) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [ba3b0345f0](https://github.com/storytold/craft-launcher/commit/ba3b0345f0b14f5cb892dcb8b917ca212c124cf9) |
