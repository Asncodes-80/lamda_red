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
   Windows or Linux, or **Cmd+S** on macOS. Each save refreshes the preview
   while focus stays in the source editor.

Use the preview icon in the editor toolbar or run
**Lamda RED: Open Preview to the Side** from the Command Palette to
preview the current buffer before saving. Select **Actual size** in the
preview to scroll through a large diagram at full resolution. If a
compilation fails, the panel shows the compiler error and keeps the last
successful render for that file.

The preview generates files in a temporary directory. For a PNG and
editable XML to keep, run `lmdred sample.zz my-diagram` in the terminal.

| Setting | Default | Effect |
| --- | --- | --- |
| `lamdaRed.previewOnSave` | `true` | Open or refresh the preview on save. Set it to `false` to use the preview command on demand. |
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
from the repository, edit a label, and save it. Introduce a syntax
error and save again to check the error display.

Package the companion extension from its directory and install the
generated VSIX with **Extensions: Install from VSIX...**.

```sh
cd vscode-preview
npx @vscode/vsce package
```

## Release alongside a compiler update

The GitHub workflow for `lmdred-v*` tags publishes the compiler and its
installers. Publish the VS Code companion separately under
`OpenLamda-RED.lamda-red-preview`. The compiler, the companion, and the
Drizzle syntax extension each have their own version.

1. Install the new compiler release or build it with
   `cargo build --locked --release` from the repository root. To test a
   local build, set `lamdaRed.compilerPath` in the Extension Development
   Host to the absolute path to `target/release/lmdred` (or
   `target/release/lmdred.exe` on Windows). Run that executable with
   `--version` to confirm the version under test.
2. Open a representative Drizzle file, save it, and check that the PNG
   refreshes beside the editor. Save an invalid edit to check the compiler
   error, then fix it and confirm the preview recovers. Test an example
   that exercises any new syntax or rendering behavior.
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
