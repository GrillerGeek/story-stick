# Story Stick rebrand — design

Date: 2026-11-21
Status: awaiting approval

## Intent

This repository started as a fork of BenchUp (`BenchUp3D/editor`). Jason is taking it his own way under a new name: **Story Stick**, a woodworking design tool for sketching and drawing projects away from the shop. It starts as a personal tool and may be shared later, so the result must be a clean, self-standing product with no ties to upstream infrastructure.

The name: a story stick is the board a woodworker marks measurements on and carries from piece to piece. This app does the same job with a model.

## Goals

- Every user-visible "BenchUp" / "KetchUp" becomes "Story Stick". That covers the UI, window title, dialogs, PDF header, docs, README, site and release tooling.
- Every internal name changes too (Jason chose the full rename): crates, binary, Rust identifiers, environment variables, scripts and packaging.
- Nothing points at upstream: the updater, issue and feature links, Help menu, site, release downloads and app ID all point at Jason's fork.
- Rename the GitHub repo from `GrillerGeek/be-gg` to `GrillerGeek/story-stick`.
- Remove the upstream-only legacy code: the Ketchup3D → BenchUp3D repo-transfer migration and the Ketchup Server references.
- Keep the MIT notice and add Jason's copyright line. Credit BenchUp as the origin in the README.

## Non-goals

- No feature or behavior changes beyond the rename and the legacy removal.
- No new project file format. New saves stay `.bup`, and `.bup`, `.k3d` and `.ketchup` all still open. A new extension is a later decision, made only if the app gets published.
- No final logo artwork. The new mark is a placeholder that can be swapped later.
- No release or deploy in this pass. Version stays 0.6.7, and the README screenshot refresh waits for the first Story Stick release.

## Anti-goals (a failure even if it compiles)

- Existing `.bup`, `.k3d` or `.ketchup` projects stop opening.
- A build of this fork auto-updates from upstream BenchUp releases, or files issues on the upstream repo.
- Leftover "BenchUp" text anywhere a user can see it.
- Dropping the original MIT copyright notice.

## Approaches considered

- **A. Full rename (chosen by Jason).** Internal names change along with visible ones. The result is clean and fully owned. The cost is that merging upstream later becomes impractical; upstream fixes would be copied by hand.
- **B. Visible-only rename.** Keeps the internal `benchup-*` names so upstream merges stay easy. Rejected because the old name would remain throughout the code and build.

## Naming conventions

| Thing | Old | New |
|---|---|---|
| Display name | BenchUp | Story Stick |
| Crates (directories and package names) | `benchup-core`, `benchup-app` | `story-stick-core`, `story-stick-app` |
| Rust crate paths | `benchup_core`, `benchup_app` | `story_stick_core`, `story_stick_app` |
| Binary / executable | `benchup-app` | `story-stick` |
| Application ID | `io.github.BenchUp3D.BenchUp` | `io.github.GrillerGeek.StoryStick` |
| Repository | `BenchUp3D/editor` (code), `GrillerGeek/be-gg` (actual) | `GrillerGeek/story-stick` |
| Site | `benchup3d.github.io/editor/` | `grillergeek.github.io/story-stick/` |
| Environment variables | `BENCHUP_*` (and legacy `KETCHUP_*` fallbacks) | `STORY_STICK_*`; legacy fallbacks removed |
| Identifiers / translation keys containing `benchup` / `ketchup` | e.g. `project_files.benchup_project` | `story_stick` equivalents |
| Trunk log path in AGENTS.md | `/tmp/ketchup-trunk.log` | `/tmp/story-stick-trunk.log` |

Identifiers that refer to legacy *file formats* (for example code that reads `.ketchup` files or the SketchUp `.skp` reader) keep a name that describes the format rather than the product. Only product-name uses are renamed.

## Decisions

- **Project data on disk:** files written by the app keep reading as before. Any format marker or field name inside saved projects stays unchanged, because those are compatibility keys, not branding. The reader must accept everything it accepts today.
- **Local settings and autosave:** the new app ID means a new data directory and new browser storage keys. Jason accepted that his existing local preferences reset. No migration from the old location.
- **Account sign-in:** keep the optional feature. Describe its server generically ("an external account server, configured with `STORY_STICK_SERVER_URL` or `--server-url`") and remove the `/home/dcoulombe/...` paths and the Ketchup Server sections from README and AGENTS.md.
- **Repo-transfer migration:** remove the transfer logic, its tests, fixtures and scripts (`update_transfer_tests.rs`, `tests/fixtures/update-transfer/`, `scripts/verify_update_transfer.py` and its test). Also remove the owner allow-list in `update_repository.rs`. The default update repository becomes `GrillerGeek/story-stick`, still overridable through the renamed environment variable.
- **CI:** rename the `BENCHUP_*` variables in the workflows and drop the `KETCHUP_*` fallbacks. Secret names that aren't product-branded (`AZURE_*`, `MACOS_*`, `APPLE_*`) are unchanged.
- **Branding assets:** add a new placeholder mark and logo SVG (a story stick: a board with tick marks), and regenerate the PNG, ICO and ICNS icon set plus the webmanifest from it. Rename `benchup.icns` to `story-stick.icns`. The PDF header keeps loading `icon-256.png`, so the new artwork shows up there automatically.
- **Release notes:** keep the existing history in `assets/release-notes.json` as the project's lineage. The rename itself gets an entry with the next release, not now.
- **README:** retitle it, point the download and browser badges at the fork, and add a short "Story Stick began as a fork of [BenchUp](https://github.com/BenchUp3D/editor)" credit. That credit is the one intentional remaining upstream link. Keep the existing screenshot until the first release recaptures it.
- **AGENTS.md:** update the product names, crate paths (for example `crates/story-stick-app/src/sidebar.rs`) and log path; remove the Ketchup Server rule. Every other convention stays.
- **LICENSE:** keep `Copyright (c) 2026 BenchUp contributors` and add `Copyright (c) 2026 Jason Robey`.

## Execution and verification

- Work on a branch (`rebrand-story-stick`).
- Verify locally with `cargo build`, `cargo test --workspace` and the Python script tests (`python3 -m unittest` under `scripts/`). This isn't a deploy, so local builds are allowed. Check the browser build compiles for `wasm32` if the toolchain is installed.
- After the rename, a case-insensitive search for `benchup` and `ketchup` must return only these intentional cases:
  - the README credit line
  - the original LICENSE notice
  - historical release-notes entries
  - legacy file-format compatibility code (the `.k3d` and `.ketchup` extensions and their readers)
- Then commit and show Jason the result. After he approves, push the branch, merge it to main, rename the GitHub repo with `gh repo rename story-stick`, and update the local `origin` URL.

## Left to the builder

- Order of operations for the mechanical rename (directory moves, then textual replacement, then targeted fixes).
- Exact placeholder logo geometry and colors.
- How the icon rasters are produced. `sips` and `iconutil` are available on this Mac; no SVG rasterizer is installed.
