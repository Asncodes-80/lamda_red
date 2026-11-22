# Lamda RED Preview

<div align="center">
    <img src="https://raw.githubusercontent.com/Asncodes-80/lamda_red/main/images/logo_lamda_red.png" width="300" height="300" />
    <h1>lmdred VSCode Preview Extension</h1>
</div>

Compile Drizzle diagrams when you save and see the PNG beside the source
editor in desktop VS Code. Lamda RED Preview uses the Drizzle syntax
highlighting extension and the Lamda RED compiler (`lmdred`).

## Use the preview

1. [Install the compiler](https://github.com/Asncodes-80/lamda_red#installation)
   and restart VS Code so it picks up your PATH. Run `lmdred --version`
   to verify the installation.
2. Install [Drizzle](https://marketplace.visualstudio.com/items?itemName=OpenLamda-RED.drizzle)
   for syntax highlighting, then install Lamda RED Preview from its VSIX.
   See [Develop and package](#develop-and-package) to build the VSIX.
3. Open a `.zz`, `.drizzle`, or `.rd` file. Save with **Ctrl+S** on
   Windows or Linux, or **Cmd+S** on macOS. Each save updates the export
   files and refreshes the preview while focus stays in the source editor.

Use the preview icon in the editor toolbar or run
**Lamda RED: Open Preview to the Side** from the Command Palette to
preview the current buffer before saving. Select **Actual size** in the
preview to scroll through a large diagram at full resolution. If a
compilation fails, the panel shows the compiler error and keeps the last
successful render for that file.

## Export diagram files

Save the file or click **Export diagram** above the preview to create or
reuse `lmdred-out/` beside the Drizzle source. The button exports the
current editor contents, including unsaved edits, while leaving the
source file on disk as it was. It works with export on save turned off.

Each successful export replaces the four files for that source. The names
include the source extension so `sample.zz` and `sample.rd` in the same
folder each keep their own exports.

| File for `sample.zz` | Use |
| --- | --- |
| `sample.zz.xml` | Editable mxGraph diagram for diagrams.net/draw.io |
| `sample.zz-compact.png` | Small PNG at 1× for the preview and quick sharing |
| `sample.zz-document.png` | PNG at 2× for inclusion in documents and PDFs |
| `sample.zz-high.png` | PNG at 4× for close inspection or larger placement |

Each PNG is rendered at its own resolution. For unusually large diagrams,
the compiler reduces the three resolutions proportionally to stay within
its PNG canvas limits. Syntax or rendering errors preserve the last
successful exports. If a filesystem error interrupts an update, resolve
the reported error and export again. **Open Preview to the Side** renders
in a temporary directory. Use **Export diagram** to write the files from
that preview.

Export requires compiler version `1.2.0` or later. Confirm that
`lmdred --help` lists `--export-set`. To use a local build, run
`cargo build --locked --release` at the repository root and set
`lamdaRed.compilerPath` to the resulting executable's absolute path.

| Setting | Default | Effect |
| --- | --- | --- |
| `lamdaRed.previewOnSave` | `true` | Export on save and refresh the preview for the active file. Set it to `false` to export only from the preview button. |
| `lamdaRed.compilerPath` | Empty | Use `lmdred` from PATH. Set this in User or Remote settings to an absolute path to the executable. |

With WSL, SSH, or Dev Containers, install the compiler on the remote
extension host.

### Use a new compiler release

Run the [compiler installer](https://github.com/Asncodes-80/lamda_red#installation)
for the release you want. Restart VS Code and run `lmdred --version` in
its integrated terminal to check which version is on PATH. If
`lamdaRed.compilerPath` is set, update the executable at that path or
point the setting at the new executable. The next preview runs that
compiler. An existing copy of the extension can use the new release as
long as the compiler still accepts the same arguments and produces a PNG.

## Develop and package

Open the `vscode-preview` folder in desktop VS Code and press **F5**.
The launch configuration loads both Lamda RED Preview and the local
Drizzle extension. In the Extension Development Host, open `sample.zz`
from the repository, edit a label, and save it. Edit another label and
click **Export diagram** to export the unsaved edit. Introduce a syntax
error and export again to check the error display.

Package the companion extension from its directory and install the
generated VSIX with **Extensions: Install from VSIX...**.

```sh
cd vscode-preview
npx @vscode/vsce package
```

## Release alongside a compiler update

The GitHub workflow for `lmdred-v*` tags publishes the compiler and its
installers. Publish the VS Code companion separately under
`OpenLamda-RED.lamda-red-vsc-preview`. The compiler, the companion, and the
Drizzle syntax extension each have their own version.

1. Install the new compiler release or build it with
   `cargo build --locked --release` from the repository root. To test a
   local build, set `lamdaRed.compilerPath` in the Extension Development
   Host to the absolute path to `target/release/lmdred` (or
   `target/release/lmdred.exe` on Windows). Run that executable with
   `--version` to confirm the version under test.
2. Open a representative Drizzle file, save it, and check that the PNG
   refreshes beside the editor and all four files update in `lmdred-out/`.
   Save an invalid edit to check that the compiler error leaves the
   existing exports intact, then fix it and confirm the preview recovers.
   Test an example that exercises any new syntax or rendering behavior.
3. Publish a new companion version when the preview code, compatibility
   requirements, or packaged documentation changes. For example, if the
   compiler changes its CLI arguments or output filenames, update
   `extension.js` to match. If the compiler changes the Drizzle grammar,
   review the TextMate grammar in `drizzle/` and publish an update to
   `OpenLamda-RED.drizzle` if syntax highlighting needs to change. Renderer
   fixes with the same CLI and PNG output reach users through the compiler
   installer alone.
4. In `vscode-preview/package.json`, bump the extension's own `version`
   (for example, `0.1.0` to `0.1.1` for a compatibility fix). Run
   `npx @vscode/vsce package` from `vscode-preview/`. Install the generated
   VSIX locally and repeat the save and error checks.
5. Upload that tested VSIX under **Open Lamda-RED** in the
   [Marketplace publisher dashboard](https://marketplace.visualstudio.com/manage/publishers/).
   Create a listing on the first publication. For later versions, upload
   to the existing **Lamda RED Preview** listing and verify the version in
   its version history. For direct distribution, attach the VSIX to the
   corresponding GitHub release and tell users to install it through
   **Extensions: Install from VSIX...**.

In the release notes, name the compiler version you tested and link to
its installer so users can update both components when needed.
[VS Code extension publishing documentation](https://code.visualstudio.com/api/working-with-extensions/publishing-extension).
