# Story Stick Rebrand Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Rename the BenchUp fork to Story Stick throughout: code, build, branding, docs, site and GitHub repo. Remove the upstream-only legacy code.

**Architecture:** Remove the legacy code first, so the rename has less to touch. Then do the mechanical rename with one replacement script (ordered, case-aware, with excluded paths), followed by targeted hand edits for branding, docs and the license. Verify with the full local test suite plus a search for leftover names that allows only a fixed list of exceptions.

**Tech Stack:** Rust workspace (eframe/egui, wgpu), Trunk/WASM, Python scripts (Pillow for branding), Node test scripts, GitHub Actions.

**Spec:** `docs/superpowers/specs/2026-11-21-story-stick-rebrand-design.md`

## Global Constraints

- Display name `Story Stick`; crates `story-stick-core` / `story-stick-app`; Rust paths `story_stick_core` / `story_stick_app`; binary `story-stick`.
- App ID `io.github.GrillerGeek.StoryStick`; repository `GrillerGeek/story-stick`; site `https://grillergeek.github.io/story-stick/`.
- Environment variables `STORY_STICK_*`; no `KETCHUP_*` or `BENCHUP_*` fallbacks remain.
- `.bup`, `.k3d` and `.ketchup` remain openable; new saves stay `.bup`. Version stays `0.6.7`.
- **Never alter "SketchUp" / "sketchup" / `.skp`.** Case-insensitive `ketchup` matches inside "SketchUp", so every match must use `(?<![sS])ketchup`.
- Never edit `crates/*/vendor/**` (third-party OpenSKP, fflate, earcut, FlatBuffers).
- Keep `Copyright (c) 2026 BenchUp contributors`. Keep historical entries in `assets/release-notes.json` as they are.
- Keep the AGENTS.md conventions (persistence, modal order, InspectorTable, PDF header, GPU rendering, English UI), updated only for names and paths.

## Review Focus

1. **Old projects stop opening.** A `.bup` saved before the rename must still load. Task 3 adds `example_project_from_before_the_rebrand_still_opens`.
2. **"SketchUp" gets corrupted by the replacement.** Its occurrence count must be the same before and after. Tasks 1 and 3 record and compare it.
3. **The updater or issue links still point upstream.** Task 3 updates the `update_repository.rs` test to assert `https://github.com/GrillerGeek/story-stick` and `system_info.rs` to assert the fork's issue URL.
4. **The browser build breaks** because `index.html` names a binary or canvas ID that no longer exists. Task 6 runs `trunk build` (or `cargo check --target wasm32-unknown-unknown` if Trunk is missing).
5. **Leftover visible "BenchUp"** in UI strings or translation values. Task 6's residual search covers `assets/i18n/en.json`, `index.html` and `site/`.

---

### Task 1: Branch and baseline

**Files:** none modified.

- [ ] **Step 1:** `git switch -c rebrand-story-stick`
- [ ] **Step 2: Baseline Rust tests.** Run `cargo test --workspace 2>&1 | tail -30`. Save the pass/fail summary to the scratchpad as `baseline.txt`. Any failures already there are carried forward, not fixed.
- [ ] **Step 3: Baseline script tests.** Run `python3 -m unittest discover -s scripts -p 'test_*.py'`, and `node scripts/<name>` for each `scripts/test_*.cjs` / `test_*.mjs`. Append the results to `baseline.txt`.
- [ ] **Step 4: Record SketchUp counts.** Run `git grep -oiI 'sketchup' -- . ':!crates/*/vendor' | wc -l` and append the number to `baseline.txt`.

### Task 2: Remove upstream-only legacy code

**Files:**
- Delete: `crates/benchup-app/src/update_transfer_tests.rs`, `crates/benchup-app/tests/fixtures/update-transfer/` (the whole directory), `scripts/verify_update_transfer.py`, `scripts/test_verify_update_transfer.py`
- Modify: `crates/benchup-app/src/lib.rs` (or wherever `mod update_transfer_tests` is declared)
- Modify: `crates/benchup-app/src/update_repository.rs`: delete the `official_owner` allow-list (lines ~26) and its owner-loop test (~46-60). `url()` normalizes any `owner/name`.
- Modify: `crates/benchup-app/build.rs`: drop the `KETCHUP_UPDATE_REPOSITORY` and `KETCHUP_SERVER_URL` fallbacks and their `rerun-if-env-changed` lines.
- Modify: `crates/benchup-app/src/server_config.rs`: delete `LEGACY_KEY` and the legacy-precedence tests (lines ~131-138).
- Modify: `crates/benchup-app/src/identity.rs`: delete `LEGACY_APP_ID`, the `ketchup.{suffix}` storage fallbacks, the `KETCHUP_DATA_DIR` fallback, the old-ID migration list (lines ~78-80) and its test (~147).
- Modify: `crates/benchup-app/src/storage.rs`: delete the `_ketchup_autosave` fallback (~30), the `ProjectDirs::from("", "Ketchup", "Ketchup")` copy (~119) and the legacy test (~234).
- Modify: `crates/benchup-app/src/browser_storage.js:61`: drop the `ketchup.` key fallback. Modify `index.html:12`: drop the `ketchup.theme.v1` fallback.
- Modify: `crates/benchup-app/src/app.rs:2443`: drop the `ketchup-geometry-v1` clipboard prefix fallback.
- Modify: `crates/benchup-app/src/updates.rs:~371`: replace the `ketchup-app.exe` migration-test fixture with the current exe name, or delete that migration test if it only covers the Ketchup transfer.
- Modify: `scripts/desktop_release.py:91` and `.github/workflows/build.yml:74`: drop the `KETCHUP_REQUIRE_SIGNING` fallback. Modify `scripts/capture_readme_desktop.py:164-170`: replace the `KETCHUP_*` names with the `BENCHUP_*` equivalents (Task 3 renames those). Modify `scripts/test_desktop_release.py:72`: delete the legacy-PackageId case.

**Interfaces:** Produces nothing new. The update repository is still read from the `BENCHUP_UPDATE_REPOSITORY` build variable, which Task 3 renames to `STORY_STICK_UPDATE_REPOSITORY`.

- [ ] **Step 1:** Make the deletions and edits above. Remove any helpers left unused by the deleted tests.
- [ ] **Step 2: Check no legacy references remain.** Run `git grep -nIP '(?<![sS])(?i:ketchup)' -- . ':!crates/*/vendor' ':!assets/release-notes.json' ':!docs/superpowers'`. Expected matches only: README and AGENTS (Task 5), `.k3d`/`.ketchup` extension code and tests, `docs/*.md` mentions of the `.ketchup` extension, and `X-Ketchup-Guest` (Task 3).
- [ ] **Step 3: Run tests.** `cargo test --workspace` gives the baseline result minus the deleted tests. `python3 -m unittest discover -s scripts -p 'test_*.py'` passes.
- [ ] **Step 4: Commit.** `git commit -am "Remove upstream Ketchup/BenchUp transfer and legacy migration code"` (stage the deletions with `git add -A`).

### Task 3: Mechanical rename

**Files:**
- Create: `scripts/rebrand.py` (a one-off; deleted in Step 7)
- Move: `crates/benchup-core` → `crates/story-stick-core`, `crates/benchup-app` → `crates/story-stick-app`, `packaging/linux/io.github.BenchUp3D.BenchUp.desktop` → `packaging/linux/io.github.GrillerGeek.StoryStick.desktop`
- Modify: every tracked text file except the exclusions below
- Test: `crates/story-stick-app/src/project_file_tests.rs`, `update_repository.rs`, `system_info.rs`

**Interfaces:**
- Produces: package `story-stick-app` with `[[bin]] name = "story-stick"` and `path = "src/main.rs"`; package `story-stick-core`; type `StoryStickApp` (formerly `BenchUpApp`); `identity::APP_ID = "io.github.GrillerGeek.StoryStick"`; `update_repository::REPOSITORY = "GrillerGeek/story-stick"`.

- [ ] **Step 1: Write the failing regression test** in `project_file_tests.rs`:

```rust
#[test]
fn example_project_from_before_the_rebrand_still_opens() {
    let bytes = include_bytes!("../../../assets/examples/Example.bup");
    let doc = decode(bytes).expect("pre-rebrand .bup must still decode");
    assert!(!doc.model.is_empty(), "example model should contain geometry");
}
```

  Use whichever emptiness accessor `Document`/`Model` actually exposes. The test passes before the rename; it guards the rename.
- [ ] **Step 2: Move the directories.** `git mv` the two crate directories and the desktop file.
- [ ] **Step 3: Write `scripts/rebrand.py`.** It walks `git ls-files` and skips: `crates/*/vendor/**`, `assets/release-notes.json`, `LICENSE`, `docs/superpowers/**`, `Cargo.lock`, binary files, and itself. It applies these literal replacements in order:

| # | From | To |
|---|---|---|
| 1 | `io.github.BenchUp3D.BenchUp` | `io.github.GrillerGeek.StoryStick` |
| 2 | `https://benchup3d.github.io/editor/` | `https://grillergeek.github.io/story-stick/` |
| 3 | `BenchUp3D/editor` | `GrillerGeek/story-stick` |
| 4 | `benchup-app` (binary/exe contexts: `data-bin="benchup-app"`, `benchup-app.exe`, `--binary .../benchup-app`) | `story-stick` |
| 5 | `benchup-app` (all remaining) | `story-stick-app` |
| 6 | `benchup-core` | `story-stick-core` |
| 7 | `benchup_core` / `benchup_app` | `story_stick_core` / `story_stick_app` |
| 8 | `BENCHUP_` | `STORY_STICK_` |
| 9 | `BenchUp` followed by an identifier character (e.g. `BenchUpApp`, `BenchUpUbuntu`) | `StoryStick` |
| 10 | `BenchUp` (prose) | `Story Stick` |
| 11 | `benchup` (remaining lowercase: keys, IDs, `benchup.icns`) | `story-stick` in paths, CSS/HTML IDs and file names; `story_stick` in Rust identifiers and translation keys |
| 12 | `X-Ketchup-Guest` | `X-Story-Stick-Guest` |

  Rule 4 must run before rule 5, and rule 9 before rule 10. For rule 11, choose by context: inside `.rs` identifiers and `en.json` keys use `_`; elsewhere use `-`. Translation keys must be renamed identically in `assets/i18n/en.json` and in the `tr!` call sites. In dry-run mode the script prints every file and line it would change.
- [ ] **Step 4: Dry run, review, apply.** Run `python3 scripts/rebrand.py --dry-run | less`, check for `SketchUp` hits (there must be none), then `python3 scripts/rebrand.py`. Add `[[bin]] name = "story-stick"` to `crates/story-stick-app/Cargo.toml`, and update the workspace `members` and `default-members`.
- [ ] **Step 5: Update the test assertions that pin URLs** to the new values: `update_repository.rs` tests expect `https://github.com/GrillerGeek/story-stick`; `system_info.rs` and `feature_suggestion_tests.rs` expect `https://github.com/GrillerGeek/story-stick/issues/new?body=`. Rule 3 should already produce these, so confirm.
- [ ] **Step 6: Build and test.** `cargo build && cargo test --workspace` matches the baseline (minus Task 2 deletions, plus the new test); `Cargo.lock` regenerates with the new names. Python unit tests pass. Run the Node `scripts/test_*.cjs` / `.mjs` tests, which must match the baseline. `git grep -oiI 'sketchup' -- . ':!crates/*/vendor' | wc -l` must equal the Task 1 count.
- [ ] **Step 7: Commit.** Delete `scripts/rebrand.py`, then `git add -A && git commit -m "Rename BenchUp to Story Stick across code, build, and tooling"`.

### Task 4: Placeholder branding

**Files:**
- Modify: `scripts/generate_branding.py` (geometry, docstring, and `.icns` output name `story-stick.icns`)
- Regenerate: `assets/branding/{logo.svg, mark.svg, icon-*.png, apple-touch-icon.png, icon-maskable-512.png, favicon.ico, story-stick.icns}`, and the generated egui geometry file the script writes (`rust_path`, likely `crates/story-stick-app/src/brand_mark.rs`)
- Delete: `assets/branding/benchup.icns`
- Modify: `assets/branding/site.webmanifest` (name and short_name `Story Stick`), any `benchup.icns` references in packaging, workflows or scripts

- [ ] **Step 1:** Replace `FACES` with a story stick: one long horizontal board, roughly x 14–102 and y 50–66 on the existing 116-unit canvas, using the same extrusion `DEPTH`. Add 5–7 short darker tick-mark rectangles along the front face, one longer than the rest. Keep the existing `RED`/`FRONT`/`TOP`/`SIDE` palette so the UI theme still matches. Update the comment to "A story stick: a marked layout board."
- [ ] **Step 2:** Run `python3 scripts/generate_branding.py`. All listed outputs are rewritten; `git status` shows them changed and `story-stick.icns` created.
- [ ] **Step 3: Check the result visually.** Read `assets/branding/icon-256.png` and `logo.svg`. The mark must be recognizable at 256 px and still readable at `icon-32.png`.
- [ ] **Step 4:** `cargo test --workspace` passes, including the brand-mark and PDF-header tests (`pdf_header.rs` embeds `icon-256.png`).
- [ ] **Step 5: Commit.** `git add -A && git commit -m "Add placeholder Story Stick mark and icons"`

### Task 5: Docs, README, AGENTS.md, license

**Files:** `README.md`, `AGENTS.md`, `LICENSE`, `docs/*.md`, `site/README.md`, `assets/branding/README.md`, `crates/story-stick-app/vendor/skp/README.md` (only the lines outside the vendor list that say "BenchUp code"; Task 3's exclusion skipped this file, so edit those lines by hand)

- [ ] **Step 1: LICENSE.** Add a line `Copyright (c) 2026 Jason Robey` under the existing copyright line.
- [ ] **Step 2: README.**
  - Retitle to `# Story Stick`.
  - Under the badges, add: "Story Stick began as a fork of [BenchUp](https://github.com/BenchUp3D/editor). Design woodworking projects away from the shop."
  - Replace the Account server section's first paragraph with: "Google login and account sessions require an external account server. Configure its address with `STORY_STICK_SERVER_URL` or `--server-url`."
  - Delete the repository-transfer section (formerly ~lines 439–470) and the legacy-variables paragraph.
  - Keep the screenshot reference.
- [ ] **Step 3: AGENTS.md.**
  - Project names section becomes: "'Story Stick' (formerly BenchUp / KetchUp) refers to the Rust desktop/WebAssembly client at this repository root."
  - Delete the Ketchup Server bullet.
  - Change the trunk log path to `/tmp/story-stick-trunk.log`.
  - Check that every `crates/...` path now reads `crates/story-stick-app/...`.
- [ ] **Step 4:** Read the rendered README and docs once for grammar errors from the replacements ("a Story Stick project", "Story Stick's") and fix any.
- [ ] **Step 5: Commit.** `git commit -am "Update docs, AGENTS.md, and license for Story Stick"`

### Task 6: Full verification

- [ ] **Step 1: Residual search.** Run `git grep -nIiP 'benchup|(?<![sS])ketchup' -- . ':!crates/*/vendor' ':!docs/superpowers'`. Allowed results are only:
  - the README credit line
  - `LICENSE` line 3
  - `assets/release-notes.json`
  - `.k3d`/`.ketchup` extension code, tests and docs
  - the AGENTS.md "formerly" note

  Anything else gets fixed.
- [ ] **Step 2: Clean build and tests.** `cargo clean -p story-stick-app && cargo build && cargo test --workspace` matches the baseline. Then the Python and Node script tests.
- [ ] **Step 3: Browser build.** If installed, `trunk build` succeeds. Otherwise `cargo check -p story-stick-app --target wasm32-unknown-unknown` (if that target is installed). If neither is available, say so.
- [ ] **Step 4: Launch check.** Run `cargo run -p story-stick-app` briefly. The window title reads Story Stick; Help → About/GitHub opens `GrillerGeek/story-stick`. Then quit.
- [ ] **Step 5: Commit** any fixes from Steps 1–4.

### Task 7: Publish (only after Jason approves the branch)

- [ ] **Step 1:** Show Jason `git log --oneline main..` and the diff stats, and ask for go-ahead.
- [ ] **Step 2:** `git switch main && git merge --ff-only rebrand-story-stick && git push origin main`.
- [ ] **Step 3: Rename the GitHub repo.** `gh repo rename story-stick --repo GrillerGeek/be-gg --yes`, then `git remote set-url origin https://github.com/GrillerGeek/story-stick.git`, and confirm with `git remote -v` and `gh repo view`.
- [ ] **Step 4:** Report the GitHub Actions status for the push. Note that Pages and releases need a version tag (separate deploy request), and that the local folder is still named `be-gg`, which Jason can rename himself.
