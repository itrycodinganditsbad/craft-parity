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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [b23d362f6a](https://github.com/storytold/cadcraft/commit/b23d362f6af1dd77e319728f1d85882b098626fe) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [22db9261f9](https://github.com/storytold/deckcraft/commit/22db9261f9d1a214c296fd390173130c45d76d66) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [7cb8b12507](https://github.com/storytold/designcraft/commit/7cb8b12507def48fb9346e5004340b5e8e4cfcb7) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [f36b2b3e6b](https://github.com/storytold/effectcraft/commit/f36b2b3e6bd121dc52b00503cdf8d60f42ac242a) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [48088e3a8e](https://github.com/storytold/filmcraft/commit/48088e3a8e50bd4a7a71d1ebdfde0835b01a3051) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [c30fa3f1cf](https://github.com/storytold/gridcraft/commit/c30fa3f1cf324603f3bfc0171689b1d4083b40e5) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [4f0b110bcb](https://github.com/storytold/lightcraft/commit/4f0b110bcbd2e350c6d97b47f1d824c2222295c4) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [50f261d70c](https://github.com/storytold/pdfcraft/commit/50f261d70c8e199600a82dbaa6912c6fbc4b4612) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [8d2e615ab9](https://github.com/storytold/photocraft/commit/8d2e615ab93d424c4ec25ab15b4edb5368d1f9a6) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [c1bf044a85](https://github.com/storytold/soundcraft/commit/c1bf044a85b55e9aa083624c39fabddbb08dadeb) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [73f40e3fd3](https://github.com/storytold/vectorcraft/commit/73f40e3fd3fd6957508f18af1bf3bff93a9eb3e7) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [4a251d4b9c](https://github.com/storytold/wordcraft/commit/4a251d4b9c724a5bd3dd75f7c91c6aae77c6650e) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [d4e4ac69bb](https://github.com/storytold/artcraft/commit/d4e4ac69bbd93d237eaf5d0260bc6a5ff3a5e373) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [98c2a48940](https://github.com/storytold/artcraftx/commit/98c2a48940aa0a54604637f665ee539c122fa707) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [ad7f24a912](https://github.com/storytold/craft-launcher/commit/ad7f24a912d6f6cd586f5306d85ab22ef9bf2cbe) |
