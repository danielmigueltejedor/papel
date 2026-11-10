# Papel app icon

**Figure:** Aldus Manutius. He created the pocket book, the first portable document.

**Style:** a black-and-white portrait cut out over a full-bleed colour field, head and shoulders, in
the Nodalia icon family (one historical figure per app, one colour per app).

**Tile:** 512 × 512 viewBox, rounded square `rx=112`, clipped; no border or roundel.

**Provenance:** the portrait is "Aldus Manutius.png" from
[Wikimedia Commons](https://commons.wikimedia.org/wiki/File:Aldus_Manutius.png) (public domain). The figure was separated from its
background, converted to black and white and placed over the app colour.
Licence: see [LICENSE.txt](LICENSE.txt).

## Files

| File | What it is |
|---|---|
| `printcraft.svg` | master: the portrait as a 1024 px image embedded in an SVG with the tile clip (canonical) |
| `printcraft-small.svg` | lighter 512 px version; also the scalable Linux icon and the web favicon where there is one |
| `printcraft-1024.png` | 1024 px on Apple's grid; also the runtime Dock icon on macOS |
| `printcraft.icns` | macOS icon (16–1024 px) |
| `printcraft.ico` | Windows icon (16–256 px), embedded in `printcraft.exe` by `apps/printcraft/build.rs` |
| `hicolor/<n>x<n>/apps/ai.storyteller.printcraft.png` | Linux hicolor theme, 16–512 px; the 256 px one is the runtime icon on Windows and Linux |
| `hicolor/scalable/apps/ai.storyteller.printcraft.svg` | Linux scalable icon (copy of the master) |

Where it shows: `apps/printcraft/src/main.rs` sets the window icon (Dock, taskbar, Alt-Tab, launcher) and the
Wayland app id `ai.storyteller.printcraft`; `packaging/linux/ai.storyteller.printcraft.desktop` names the
hicolor icon.

## Regenerate

The icon is a raster portrait, so the SVG files embed an image instead of traced paths. The
derived files in this directory were rendered directly from the portrait. `packaging/icons.sh`
still works on `printcraft.svg` (resvg renders embedded images), but its output will differ slightly
from the committed files in resampling.
