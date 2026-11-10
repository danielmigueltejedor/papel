<div align="center">
  <a href="https://getnodalia.com">
    <picture>
      <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-white.svg">
      <img src="https://raw.githubusercontent.com/danielmigueltejedor/nodalia-web/main/assets/brand/nodalia-wordmark-black.svg" width="150" alt="Nodalia">
    </picture>
  </a>
  <br><br>
  <img src="https://raw.githubusercontent.com/danielmigueltejedor/papel/main/assets/app-icon/hicolor/512x512/apps/ai.storyteller.pdfcraft.png" width="160" alt="Papel">
  <h1>Papel</h1>
  <p><strong>A PDF document editor for the Nodalia suite, built on the PdfCraft engine.</strong></p>

  <p>
    <img src="https://img.shields.io/badge/status-early%20stage-orange" alt="Early stage">
    <img src="https://img.shields.io/badge/engine-PdfCraft-68778F" alt="PdfCraft engine">
    <img src="https://img.shields.io/badge/Rust-native-dea584?logo=rust&logoColor=black" alt="Native Rust">
    <img src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-2ea44f" alt="MIT OR Apache-2.0">
  </p>

  <p>
    <a href="#overview">Overview</a> ·
    <a href="#what-you-get-today">Today</a> ·
    <a href="#build-and-run">Build</a> ·
    <a href="#project-status">Status</a> ·
    <a href="#keeping-up-with-upstream">Upstream</a> ·
    <a href="#the-nodalia-suite">Suite</a> ·
    <a href="#license">License</a>
  </p>
</div>

---

## Overview

**Papel** is the PDF document editor of [Nodalia](https://getnodalia.com). It is built on the engine of
[PdfCraft](https://github.com/storytold/pdfcraft), which lives in this repository and is kept up to
date with upstream.

> [!IMPORTANT]
> **Papel is at an early stage.** What runs today is the upstream application under the Papel name.
> Nodalia's own interface is not wired in yet and there are no published builds.

## What you get today

- The PdfCraft engine.
- A desktop binary called `papel`.
- The engine's own feature description, kept as [`docs/upstream-readme.md`](docs/upstream-readme.md).

## Build and run

You need a recent stable Rust toolchain.

```bash
git clone https://github.com/danielmigueltejedor/papel.git
cd papel
cargo run --release -p pdfcraft
```

The desktop binary is called `papel`.

## Project status

| Area | State |
|---|---|
| Engine | Imported from `storytold/pdfcraft` and tracked in [`docs/upstream.md`](docs/upstream.md) |
| Product name | Done: the desktop binary is called `papel` |
| App icon | Done: portrait of Aldus Manutius |
| Nodalia interface | Not started |
| Shared interface resources | Planned: the Nodalia apps will share interface components and styles |
| Installable packages | None published |

## Keeping up with upstream

[`docs/upstream.md`](docs/upstream.md) records which upstream commit is integrated. Updates are
merges of upstream's `main`, so Papel's own changes are kept and conflicts are resolved in the open.

## App icon

Nodalia app icons are black-and-white portraits of historical figures over the app colour.
Papel's is Aldus Manutius, who created the pocket book, the first portable document. Provenance and licence are in [`assets/app-icon/`](assets/app-icon/).

## The Nodalia suite

Papel is one of the applications of [Nodalia](https://getnodalia.com), an independent software
project developed in public by [Daniel Miguel Tejedor](https://danielmigueltejedor.com).

## License

`MIT OR Apache-2.0`, as upstream. The licence files and `NOTICE` of PdfCraft are kept.
See [`LICENSE-MIT`](LICENSE-MIT) and [`LICENSE-APACHE`](LICENSE-APACHE).

## Credits

Papel would not exist without [PdfCraft](https://github.com/storytold/pdfcraft) and its contributors.
