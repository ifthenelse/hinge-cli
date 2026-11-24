# hinge-cli — Logo Guidelines (compact)

## 1. The logo
- **Idea**: a terminal prompt chevron (`>`) whose bend doubles as a hinge pin — a CLI tool first, a nod to "Hinge" second.
- **Versions**: symbol (`assets/mark.svg`) · small-size cut for ≤24px use (`assets/mark-small.svg`, no pin hole) · horizontal lockup with wordmark (`assets/lockup.svg`).
- **Files**: `assets/*.svg` for anything that scales · `assets/*.png` (512px) for places that need a raster · `assets/favicon.ico` / `assets/icon-*.png` for web/app icons.
- No print variant exists yet; this is a developer tool, not a physical product.

## 2. Clear space
Keep a clear zone of **1 × symbol height** around the mark on all sides. The zone scales with the logo — never use a fixed pixel distance.

## 3. Minimum size
| Version | Screen |
|---|---|
| Lockup (symbol + wordmark) | 120 px wide |
| Symbol alone | 24 px (use `mark.svg`) |
| Symbol alone, ≤24 px | use `mark-small.svg` instead — the pin hole closes below 24 px |

## 4. Colour
| Name | HEX | Use |
|---|---|---|
| Ink | `#111111` | Default mark colour, on white/light backgrounds |
| Brand blue | `#2563EB` | Accent only — app icon fill, links, highlight; not the default mark colour |
| White | `#FFFFFF` | Mark on dark backgrounds (terminal, dark mode) |

**Approved pairs**: black mark on white · white mark on near-black (`#111111`) · white mark on brand blue (app icon only). No gradients.

## 5. Typography
- Wordmark: set in a monospace face (SF Mono / Consolas / Menlo stack) to read as a CLI name, then **outlined to paths** for the shipped lockup — never re-typed in a different font.
- Docs/README body text: the system/GitHub default; this project has no custom typeface.

## 6. Don'ts
Don't stretch or squash the chevron · don't recolour outside the palette above · don't rotate it · don't add shadows, outlines or gradients · don't separate the symbol and wordmark at non-standard spacing · don't use the pin-hole version below 24 px · don't retype the wordmark in a different font instead of using `lockup.svg`.

## 7. Source files and provenance
- Built with the `logo-design` skill; audited with `svg_audit.py` (symbol: 98/100, small cut: 100/100, lockup: 98/100 after outlining text).
- Wordmark text was converted to paths with Inkscape's "Object to Path" (no live `<text>` in any shipped file).
- Master SVGs: `brand/assets/`. Presentation board: `brand/presentation.html` and `brand/slides/`.
- No trademark search was performed — recommend one before wide public use, especially given the name's reference to Hinge, Inc.

## Contact
Questions about usage: open an issue in this repository. Master files: `brand/assets/`.
