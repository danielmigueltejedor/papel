# Papel

Editor de documentos PDF de Nodalix.

El motor de este repositorio es el código de [storytold/printcraft](https://github.com/storytold/printcraft) en `55c581760ab389b88f08997ab235eecdf1a1b002`.
La licencia de ese código se conserva, igual que `NOTICE` y `docs/upstream-readme.md`.
Papel es el nombre del producto. El motor puede cambiar sin esperar a upstream.
Un sync posterior compara commits. No se sustituye este árbol con un reset.

## Compilar

```bash
cargo run -p printcraft
```

El binario de escritorio se llama `papel`.

## Estado

La shell que entra en este import es la de upstream. La interfaz GTK 4 / libadwaita de Nodalix no está cableada todavía. El motor sí está en este repositorio, con sus tests.
