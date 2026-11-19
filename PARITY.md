# Craft fix parity

Which Craft apps carry a fix that already landed in a sibling app. ✅ fixed, ❌ missing, ➖ does not apply, ❔ not scanned.

| Fix | cadcraft | deckcraft | designcraft | effectcraft | filmcraft | gridcraft | lightcraft | pdfcraft | photocraft | soundcraft | vectorcraft | wordcraft | artcraft | artcraftx | craft-launcher |
|---|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|:-:|
| [Text: right-to-left scripts laid out with the Unicode BiDi algorithm](fixes/bidi-text.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ | ✅ | ❌ | ❌ | ✅ | ✅ | ❌ | ➖ | ❌ |
| [macOS DMG: branded window background and icon layout](fixes/macos-dmg-background.toml) | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Desktop: log to a rotating log file as well as standard error](fixes/rotating-log-file.toml) | ✅ | ✅ | ✅ | ❌ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ | ➖ | ➖ | ✅ |
| [Windows: draw on the GPU that drives the primary display](fixes/windows-primary-gpu.toml) | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ➖ | ✅ | ➖ | ➖ | ✅ | ➖ | ➖ | ➖ | ➖ |

## Gaps

### Text: right-to-left scripts laid out with the Unicode BiDi algorithm

Arabic and Hebrew come out in reverse order, and mixed left-to-right and right-to-left runs land in
the wrong places, unless lines are reordered with UAX #9.

Missing in: lightcraft, photocraft, soundcraft, artcraft, craft-launcher. Port from [storytold/deckcraft#4](https://github.com/storytold/deckcraft/pull/4), [storytold/pdfcraft#285](https://github.com/storytold/pdfcraft/pull/285), [storytold/pdfcraft#403](https://github.com/storytold/pdfcraft/pull/403).

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
| [cadcraft](https://github.com/storytold/cadcraft) | origin/HEAD | [a770fa5a5e](https://github.com/storytold/cadcraft/commit/a770fa5a5e5f9d4ffe4dc0f60db5eacb6665c980) |
| [deckcraft](https://github.com/storytold/deckcraft) | origin/HEAD | [e4f86d11fe](https://github.com/storytold/deckcraft/commit/e4f86d11fe8a9db91a6c433b7a3032b7b2801a98) |
| [designcraft](https://github.com/storytold/designcraft) | origin/HEAD | [88f5a00e38](https://github.com/storytold/designcraft/commit/88f5a00e389b5378816dc2daa8c180d90081b393) |
| [effectcraft](https://github.com/storytold/effectcraft) | origin/HEAD | [13e6e6c67e](https://github.com/storytold/effectcraft/commit/13e6e6c67e805f443a87e3bd3160095d30b5cf3a) |
| [filmcraft](https://github.com/storytold/filmcraft) | origin/HEAD | [f67d38b35c](https://github.com/storytold/filmcraft/commit/f67d38b35cc5af6853ef5a6921ccba014512694b) |
| [gridcraft](https://github.com/storytold/gridcraft) | origin/HEAD | [1687a9c05b](https://github.com/storytold/gridcraft/commit/1687a9c05b0c54fdfd0f40bc13698015e94cf494) |
| [lightcraft](https://github.com/storytold/lightcraft) | origin/HEAD | [78b575658a](https://github.com/storytold/lightcraft/commit/78b575658a61d17bd392b4b81b64c92c8beecbac) |
| [pdfcraft](https://github.com/storytold/pdfcraft) | origin/HEAD | [f0bbce5b30](https://github.com/storytold/pdfcraft/commit/f0bbce5b3076e6b694ea3c4ffe015d7a62f809f3) |
| [photocraft](https://github.com/storytold/photocraft) | origin/HEAD | [dea3494561](https://github.com/storytold/photocraft/commit/dea3494561294a96d3e490a8eb0c16bbcc901431) |
| [soundcraft](https://github.com/storytold/soundcraft) | origin/HEAD | [a02d892f83](https://github.com/storytold/soundcraft/commit/a02d892f831f0a4e1220e0357c7b941030b9cd6f) |
| [vectorcraft](https://github.com/storytold/vectorcraft) | origin/HEAD | [ddb9495876](https://github.com/storytold/vectorcraft/commit/ddb9495876a222f1135d12a77f53052ccc28d820) |
| [wordcraft](https://github.com/storytold/wordcraft) | origin/HEAD | [6ef13b2cc8](https://github.com/storytold/wordcraft/commit/6ef13b2cc8a60e6391d7a5b92459ec47598d58a9) |
| [artcraft](https://github.com/storytold/artcraft) | origin/HEAD | [a6d52df45e](https://github.com/storytold/artcraft/commit/a6d52df45e553a6119116e560403155b0c65dc60) |
| [artcraftx](https://github.com/storytold/artcraftx) | origin/HEAD | [fba78de478](https://github.com/storytold/artcraftx/commit/fba78de4782fe727739b4e34ed8b547cbd601632) |
| [craft-launcher](https://github.com/storytold/craft-launcher) | origin/HEAD | [59e3f27f9e](https://github.com/storytold/craft-launcher/commit/59e3f27f9e27a5b71f4798f60ae35c186b94b315) |
