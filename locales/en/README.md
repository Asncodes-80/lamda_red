<div align="center">
    <img src="../../images/logo_lamda_red.png" width="128" height="128" />
    <h1>Lamda RED</h1>
    <p>
        <a href="../en/README.md">English</a> ·
        <a href="../ru/README.md">Русский</a> ·
        <a href="../fa/README.md">فارسی</a> ·
        <a href="../ar/README.md">العربية</a> ·
        <a href="../tr/README.md">Türkçe</a> ·
        <a href="../zh-CN/README.md">简体中文</a> ·
        <a href="../de/README.md">Deutsch</a> ·
        <a href="../es/README.md">Español</a> ·
        <a href="../ko/README.md">한국어</a>
    </p>
    <p>Requirements engineering diagrams from plain text</p>
</div>

Lamda RED turns plain-text `.zz` files into diagrams for requirements engineering. Use it to show what a system must achieve, what could go wrong, and how agents and system objects interact. Keep the source next to your requirements in Git so your team can review diagram changes in a diff and regenerate the images.

Write the diagram in **Drizzle** and compile it with **`lmdred`**. The compiler produces a PNG for documents and editable mxGraph XML for diagrams.net (draw.io). Use `--export-set` for compact, document, and high-resolution PNGs. In desktop VS Code, the companion extension previews the diagram beside your editor and exports on save or from the **Export diagram** button.

## From Drizzle to a diagram

The example shows an agent monitoring a value from one object and controlling another. The diagram below is the rendered output.

![An input object connects to an agent, which connects to an output object](../../docs/images/object-agent.png)

Save this as `object-agent.zz`.

```drizzle
Input = [Object (obj1)]
Monitor = <Agent (Ag)>
Output = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

After installing the compiler, run the command next to that file. It writes `object-agent.png` and `object-agent.xml`.

```sh
lmdred object-agent.zz object-agent
```

[Installation](#installation) · [Preview in VS Code](#live-preview-in-vs-code) · [Read the Drizzle syntax](#drizzle-syntax)

- [CLI usage](#cli-usage)
- [Drizzle syntax](#drizzle-syntax)
- [Live preview in VS Code](#live-preview-in-vs-code)
- [Drizzle syntax highlighting for VS Code](./vscode.md)
- [Installation](#installation)
- [Source and development](#source-and-development)
  - [Set up the source](#set-up-the-source)
  - [Run and debug](#run-and-debug)
  - [Test and check](#test-and-check)
  - [Build and package](#build-and-package)
  - [Publish a compiler release](#publish-a-compiler-release)
- [Developer guide](../../DEVELOPER.md)
- [Contributing](../../CONTRIBUTING.md)
- [Code of Conduct](../../CODE_OF_CONDUCT.md)
- [Changelog](./CHANGELOG.md)
- [Roadmap](#roadmap)
- [License](#license)

## CLI usage

Call the installed command from any directory.

```text
lmdred [--export-set] [input.zz] [output-stem]
lmdred --help
lmdred --version
```

| Argument or option | Meaning | Default |
| --- | --- | --- |
| `input.zz` | Path to a UTF-8 Drizzle source file | `sample.zz` in the current directory |
| `output-stem` | Output path before the appended `.png` and `.xml` extensions | `output` in the current directory |
| `-h`, `--help` | Print command-line help | |
| `-V`, `--version` | Print the compiler version, such as `lmdred 1.2.0` | |
| `--export-set` | Write editable XML and three PNG sizes | |
| `--` | Treat the remaining arguments as file paths | |

### Output paths

Relative paths are resolved from the terminal's current working directory.
Pass an absolute path to select another location. The output directory
must already exist. The compiler appends extensions to the supplied stem.

| Command | Files written relative to the current directory |
| --- | --- |
| `lmdred sample.zz my-new-red` | `my-new-red.png` and `my-new-red.xml` |
| `lmdred sample.zz` | `output.png` and `output.xml` |
| `lmdred` | Reads `sample.zz` and writes `output.png` and `output.xml` |
| `lmdred diagrams/train.zz` | `output.png` and `output.xml` |
| `lmdred diagrams/train.zz reports/train` | `reports/train.png` and `reports/train.xml` |
| `lmdred sample.zz train.v1` | `train.v1.png` and `train.v1.xml` |

Quote paths that contain spaces. Use `--` when a path starts with `-`.

```sh
lmdred "my diagrams/train.zz" "my diagrams/train"
lmdred -- "-file.zz" result
```

### Output formats and errors

The PNG contains the rendered diagram, with a canvas sized to include the
shapes, connections, and labels. The mxGraph XML contains editable shapes,
connections, and waypoints for diagrams.net/draw.io.

When `lmdred --help` lists `--export-set`, run
`lmdred --export-set sample.zz my-diagram` for the XML and the
`my-diagram-compact.png`, `my-diagram-document.png`, and
`my-diagram-high.png` variants. These are rendered at 1×, 2×, and 4×
under normal canvas limits. For very large diagrams, all three scales are
reduced proportionally.

A successful compilation prints its output paths and exits with status
`0`. Failures print an error to standard error and exit with status `1`.
Syntax and reference errors include the file, line, and column.

Existing output files at the chosen paths are overwritten on a successful
write. The compiler checks that an output path would not replace its
source file. It parses and renders before writing any outputs. A
filesystem error while saving can leave only some outputs written, so check the
error message and rerun after correcting the path or permissions.

## Drizzle syntax

Use the PNG in documents and presentations. Keep the Drizzle file with
the requirements so reviewers can trace and revise goals, risks, agents,
and relationships. Each compilation also produces editable mxGraph XML.

### A first diagram

Save this as `first.zz` and run `lmdred first.zz first`.

```drizzle
&& The tracking agent reports train speed and location.
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
```

The output has a hexagonal agent, a rectangular object, and a solid arrow
pointing to the train. The label above the arrow has two lines.

### Declare nodes

Write `ID = shape` so a connection can refer to the node. IDs are
case-sensitive, start with an ASCII letter, and accept letters, digits, and
single underscores between alphanumeric characters. `Train_1` is an ID.
The text inside the shape is its display label. Connections may appear before
or after their declarations. Use `&&` at the start of a line for a comment.
Blank lines and indentation are allowed.

You can also write shapes by themselves, such as `/Safe journey/`. Give a
shape an ID when you want to use it in a connection.

| Shape | Declaration | Result |
| --- | --- | --- |
| Goal | `G = /Safe journey/` | Parallelogram leaning right |
| Risk | `R = \Signal lost\` | Mirrored parallelogram |
| Agent | `A = <Train controller>` | Hexagon |
| Descriptive statement | `D = ~Doors are locked in motion~` | Trapezoid with a wider base |
| Object | `O = [Train]` | Rectangle |
| Action | `Send = (Send command)` | Ellipse |
| Note | `N = {Controller responsibility}` | Light green rectangle with a dashed outline |
| Cloud annotation | `C = {{Reason for the obstacle}}` | Cloud outline |
| Diamond | `Check = ^Speed confirmed?^` | Rhombus |

Prefix a shape with `!` to thicken its outline. The reference notation uses
this for leaves such as requirements and assumptions. The label keeps its
normal weight.

```drizzle
/Goal/
!/Leaf requirement/
~Descriptive~
<Agent 1>
```

#### Labels and line breaks

Labels accept UTF-8, including right-to-left text. The renderer measures the
text, centers it inside the shape, and sizes the shape to fit. Long labels
wrap at words when possible. A long word can wrap within the word. Shape
labels show up to four lines, and connection labels show up to three. Text
that exceeds the limit ends with `...`. The full label remains in the source.

| Syntax | Result |
| --- | --- |
| `/Accurate estimate\nof speed and position/` | A goal label with a line break |
| `[C:\\Logs]` | A rectangle labeled `C:\Logs` |
| `<Speed \> zero>` | An agent labeled `Speed > zero` |
| `/Input \/ output/` | A goal labeled `Input / output` |
| `~Value \~ threshold~` | A trapezoid labeled `Value ~ threshold` |

Write `\n` for a forced line break, `\\` for a literal backslash, and put a
backslash before a delimiter you want to display inside a label.

### Connect nodes

Use named nodes on either side of the operator. The two middle characters
select the line style. The outer `<` and `>` add arrowheads at the left and
right ends respectively. Use spaces between IDs and operators for
readability when needed.

| Direction | Solid | Dashed |
| --- | --- | --- |
| No arrowheads | `A __ B` | `A -- B` |
| Points to B | `A__>B` | `A-->B` |
| Points to A | `A<__B` | `A<--B` |
| Points to both ends | `A<__>B` | `A<-->B` |

Add a colon after the connection and its options to write a label. Two
connections between the same nodes can carry different labels and use
separate tracks. Repeating the same connection draws it once. A connection
from a node to itself forms a loop.

```drizzle
Tracking = <Tracking system>
Train = [Train]
Tracking __> Train : currentSpeed\ncurrentLocation
Tracking <__ Train : measuredSpeed\nmeasuredLocation
Train --> Train : Refresh state
```

#### Add notation to a connection

| Option | Example | Result |
| --- | --- | --- |
| `[o]` | `A __ B [o]` | An open circle on the line |
| `[*]` | `A __> B [*]` | A filled circle on the line |
| `[zigzag]` | `A __ B [zigzag]` | A zigzag for a conflict |
| `[bar]` | `Risk --> Goal [bar]` | A crossbar for an obstacle |
| `[ortho]` | `A __> B [ortho]` | A route built from horizontal and vertical segments |

Use one circle, zigzag, or crossbar per connection. You can combine that
marker with `[ortho]` and with either line style. A crossbar on a
single-arrow connection appears near the arrowhead. A crossbar on a plain
or double-arrow connection appears near the middle of the route.

#### Join several sources

Forward connections to the same target share a final segment when their
line style, marker, label, and routing options match. Explicit detours
keep their own routes. List source IDs with commas to declare a shared
connection directly. An open or filled circle sits at the junction, a
right arrowhead sits on the segment into the target, and a left arrowhead
points into each source of an explicit branch.

```drizzle
layout up
Goal @1,0 = /Safe departure/
Software @0,1 = !/Verify the lock signal/
Assumption @2,1 = !/The door sensor works/

Software, Assumption __> Goal [o]
```

The two leaf nodes join a common line with an open circle and a single
arrow into the goal above them. Writing `Software __> Goal [o]` and
`Assumption __> Goal [o]` on separate lines gives the same result.

### Set the layout and routing

| Control | Example | Result |
| --- | --- | --- |
| Direction | `layout right` | Automatic flow from left to right. This is the default |
| Direction | `layout left` | Automatic flow from right to left |
| Direction | `layout up` | Automatic flow from bottom to top with right-angled routes |
| Direction | `layout down` | Automatic flow from top to bottom with right-angled routes |
| Grid placement | `Goal @2,0 = /Goal/` | Places the shape in column 2, row 0 |
| Outer route | `A --> B [bar] via bottom` | Sends the connection around the bottom of the diagram |

Write at most one layout directive per diagram. Grid coordinates start at
zero, with columns increasing to the right and rows increasing downward.
Each explicit cell holds one node. Lamda RED sizes each row and column to
fit its shapes and sizes the PNG canvas to include the diagram. You can mix
automatic and explicit placement.

The following illustration places the object-agent example in row zero.
The dotted guides and intersection dots illustrate the grid coordinates.
The highlighted `@0,2` intersection is column zero, row two.

![Object-agent diagram over a dotted grid with columns and rows numbered from zero](../../docs/images/grid-placement.png)

```drizzle
Input @0,0 = [Object (obj1)]
Monitor @1,0 = <Agent (Ag)>
Output @2,0 = [Object (obj2)]

Input __> Monitor : Obj1.attribute1\nValue monitoring
Monitor __> Output : Obj2.attribute2\nState variable\nControl
```

To place a note at the highlighted intersection, add
`Explanation @0,2 = {Column zero, row two}`. Grid coordinates select a cell,
and its pixel position depends on the sizes of the shapes in the diagram.

Write `via right`, `via left`, `via top`, or `via bottom` after the connection
options and before its label. This is useful when a conflict or an
obstruction needs to bypass other relationships.

```drizzle
layout up
Login @0,0 = /User can sign in/
Blocked @0,1 = \User cannot sign in\
Recover @0,2 = /Recover using a phone number/

Recover __> Blocked
Blocked __> Login
Recover __> Login [zigzag] via right : A non-user might gain access
```

The first two connections point up through the tree. The conflict follows
the right side and points back to the goal.

### Reference diagrams

Each example shows the result of compiling the linked Drizzle file. Run
`lmdred sample/de-morgan.zz de-morgan` for the De Morgan
diagram, or substitute another example path and output stem.

#### Goal notation and obstacles

The heavy outline distinguishes a leaf, the trapezoid records a descriptive
statement, and the mirrored risk shape points to a goal through an obstacle
bar. A cloud holds the explanatory note. [Drizzle source](../../sample/notation.zz).

![Goal and risk notation with a cloud, trapezoid, hexagon, and crossbar](../../docs/images/notation.png)

#### Requirement refinement

Independent connections let the refined requirement and leaf requirements
carry their own markers and arrowheads. [Drizzle source](../../sample/requirements.zz).

![Bottom-up refinement tree with three thick-outlined leaves](../../docs/images/requirements.png)

#### Rail goal conflicts

Branches join several refinements at one junction. The zigzags connect
conflicting goals across the tree. [Drizzle source](../../sample/rail-conflicts.zz).

![Rail goal diagram with shared circle junctions and two zigzag conflicts](../../docs/images/rail-conflicts.png)

#### Authentication risk

Mirrored shapes show the problems that prevent sign-in. The outer zigzag
shows a possible conflict introduced by a recovery option. [Drizzle source](../../sample/auth-risk.zz).

![Authentication risk tree with a zigzag route around its right side](../../docs/images/auth-risk.png)

#### De Morgan goal and risk diagram

The source explicitly declares the goal and its negation, the shared
refinement branch, and the three dashed obstructions. [Drizzle source](../../sample/de-morgan.zz).

![Goal and negated goal with a shared branch and dashed crossbar relationships](../../docs/images/de-morgan.png)

#### Agent monitoring and control

The hexagonal agent receives an attribute from the first object and
controls an attribute of the second. [Drizzle source](../../sample/object-agent.zz).

![An input object connects to an agent, which then connects to an output object](../../docs/images/object-agent.png)

#### Train control

Solid and dashed lines, separate arrows for measurements and commands,
an action ellipse, and a responsibility note appear in one diagram.
[Drizzle source](../../sample.zz).

![Train control requirements with agents, objects, an action, and a responsibility note](../../output.png)

### Editor and diagnostics

The VS Code extension source in `drizzle/` highlights `.zz`, `.drizzle`,
and `.rd` files. To try the current grammar, open that folder in VS Code and
press **F5** to start the Extension Development Host. Open your diagram
source in the host window. [VS Code extension documentation](https://code.visualstudio.com/api/get-started/your-first-extension).

| Message or symptom | How to resolve it |
| --- | --- |
| `unknown node 'X'` | Declare `X` with the same capitalization or correct the reference. |
| `duplicate ID` | Give each declared node a unique ID. |
| `cell @… is already occupied` | Assign one of the nodes to another grid cell. |
| `invalid Drizzle syntax` | Use the file, line, and column in the message to find the delimiter, operator, or misplaced option. |
| `diagram is too large for PNG output` | Bring sparse grid positions closer or split the model into diagrams. |
| `no system fonts found` | Install a sans-serif font and rerun the compiler. |
| `link.exe not found` | Install the MSVC build tools or run the GNU command above. |

## Live preview in VS Code

The [Lamda RED Preview companion extension](../../vscode-preview/) runs
`lmdred` when you save a Drizzle file and shows the PNG in the editor group
beside your source. It also saves editable mxGraph XML and three PNG sizes
in `lmdred-out/` beside the source. Use **Export diagram** in the preview to
export the current editor contents before saving. The preview uses the
existing Drizzle extension
for syntax highlighting. The companion extension guide covers setup, export
filenames, the manual preview command, and development and packaging.
[Release the preview alongside a compiler update](../../vscode-preview/README.md#release-alongside-a-compiler-update).

## Installation

The commands below install compiler release `lmdred-v1.2.0`.

The installers select x64 or ARM64 for your system, download the archive
and its SHA-256 checksum, verify it, and install `lmdred` in your user
account. Run the installer again to upgrade or repair an installation.

### Linux and macOS

Download and run the shell installer using curl.

```sh
curl -fL https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.sh -o install-lmdred.sh
sh install-lmdred.sh
```

You can also download it using wget.

```sh
wget https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.sh -O install-lmdred.sh
sh install-lmdred.sh
```

The default directory is `$HOME/.local/bin`. The installer updates
`~/.profile` and startup files for the current Bash, Zsh, or Fish shell.
Open a new terminal after installation. In Bash, Zsh, or sh, you can also
enable the command immediately in your current terminal.

```sh
export PATH="$HOME/.local/bin:$PATH"
lmdred --version
```

Choose a version or an install directory with the following options.

```sh
sh install-lmdred.sh --version 1.2.0 --bin-dir "$HOME/.local/bin"
sh install-lmdred.sh --no-modify-path
```

`LMDRED_VERSION` and `LMDRED_INSTALL_DIR` provide environment variable
defaults. Explicit command-line options take precedence. For another shell,
add the installation directory to its startup configuration.

### Windows

Download and run the installer from PowerShell.

```powershell
$installer = Join-Path $env:TEMP 'install-lmdred.ps1'
Invoke-WebRequest -Uri 'https://github.com/Asncodes-80/lamda_red/releases/download/lmdred-v1.2.0/install.ps1' -OutFile $installer
powershell -NoProfile -ExecutionPolicy Bypass -File $installer
```

Open a new terminal and run `lmdred --version`. The installer puts the
executable in `%LOCALAPPDATA%\Programs\lmdred\bin` and puts that directory
first in your user PATH. This gives it priority over a copy installed
through Cargo. If a machine-wide copy takes precedence, the installer
reports its path. Use `Get-Command lmdred -All` to check which copy your
terminal selects.

Choose a different location or manage PATH yourself using these options.

```powershell
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -Version 1.2.0 -InstallDirectory "$env:LOCALAPPDATA\Programs\lmdred\bin"
powershell -NoProfile -ExecutionPolicy Bypass -File $installer -NoModifyPath
```

`LMDRED_VERSION` and `LMDRED_INSTALL_DIR` also set the defaults on Windows.

### Download an archive directly

Extract the archive for your system and put the executable in a directory
included in your PATH.

| System | Processor | Release archive |
| --- | --- | --- |
| Windows | x64 | `lmdred-v1.2.0-x86_64-pc-windows-msvc.zip` |
| Windows | ARM64 | `lmdred-v1.2.0-aarch64-pc-windows-msvc.zip` |
| macOS | Intel | `lmdred-v1.2.0-x86_64-apple-darwin.tar.gz` |
| macOS | Apple Silicon | `lmdred-v1.2.0-aarch64-apple-darwin.tar.gz` |
| Linux | x64 | `lmdred-v1.2.0-x86_64-unknown-linux-musl.tar.gz` |
| Linux | ARM64 | `lmdred-v1.2.0-aarch64-unknown-linux-musl.tar.gz` |

Linux release binaries use musl to support glibc and musl distributions.
Windows release builds link the C runtime statically. Rendering uses
system fonts. On a minimal Linux server or container, install a sans-serif
font such as DejaVu Sans.

### Install from source

With Rust and the platform build tools installed, run this from the
repository root.

```sh
cargo install --path . --locked
```

Cargo installs `lmdred` in `~/.cargo/bin`, or `%USERPROFILE%\.cargo\bin` on
Windows. Add that directory to PATH if needed. You can also install
directly from the GitHub source.

```sh
cargo install --git https://github.com/Asncodes-80/lamda_red --locked --bin lmdred
```

For the Windows GNU toolchain, use the following command.

```powershell
cargo +stable-x86_64-pc-windows-gnu install --path . --locked
```

## Source and development

The compiler reads a UTF-8 source file, parses it into nodes and connections,
measures its labels, computes the layout and routes, and exports the diagram.

| Location | Responsibility |
| --- | --- |
| [src/main.rs](../../src/main.rs) | CLI arguments, file paths, errors, and the compilation pipeline |
| [src/lamda_red_grammar.pest](../../src/lamda_red_grammar.pest) | Drizzle grammar |
| [src/parsing.rs](../../src/parsing.rs) | Diagram model, declarations, references, and syntax diagnostics |
| [src/labels.rs](../../src/labels.rs) | Font discovery, text shaping, wrapping, and truncation |
| [src/layout.rs](../../src/layout.rs) | Shape geometry, grid placement, and canvas bounds |
| [src/routing.rs](../../src/routing.rs) | Connections, branches, detours, and connection label positions |
| [src/export.rs](../../src/export.rs) | PNG and mxGraph XML generation |
| [sample.zz](../../sample.zz) and [sample/](../../sample/) | Train control and other reference diagrams |
| [tests/test_cli_install.py](../../tests/test_cli_install.py) | CLI and release installer integration checks |
| [scripts/](../../scripts/) | Installers and release archive creation |
| [.github/workflows/release-cli.yml](../../.github/workflows/release-cli.yml) | Builds, checks, and releases for six platform targets |
| [drizzle/](../../drizzle/) | VS Code syntax highlighting extension |

### Set up the source

Install a recent stable Rust toolchain, Cargo, and the platform's native
build tools. Python 3 is used for CLI and installer integration checks.
The release packager runs in Windows PowerShell 5.1 or PowerShell 7.
PNG rendering discovers fonts installed on the host.
[Rust installation](https://www.rust-lang.org/tools/install).

```sh
git clone https://github.com/Asncodes-80/lamda_red.git
cd lamda_red
cargo build --locked
```

Windows MSVC builds require the Visual Studio C++ build tools. On macOS,
install the Xcode command-line tools. On Linux, install a C compiler and
linker. Minimal Linux environments also need a font such as DejaVu Sans.

For Windows with the GNU toolchain, use the following commands. You can
insert `+stable-x86_64-pc-windows-gnu` immediately after `cargo` in the other
build, run, test, and install commands in this README.

```powershell
rustup toolchain install stable-x86_64-pc-windows-gnu --profile minimal --component clippy,rustfmt
cargo +stable-x86_64-pc-windows-gnu build --locked
```

Keep `Cargo.lock` checked in and use `--locked` to use the committed
dependency versions.

### Run and debug

Cargo's development build includes debug information. Run a reference
diagram through the compiler with the following command.

```sh
cargo run --locked -- sample/requirements.zz target/requirements
cargo run --locked -- --help
cargo run --locked -- --version
```

The first command writes `target/requirements.png` and
`target/requirements.xml`. Running `cargo run --locked` by itself reads
`sample.zz` and writes `output.png` and `output.xml` in the repository root.

To debug in VS Code, open the repository root, install CodeLLDB, and add the
following configuration to `.vscode/launch.json`. It builds `lmdred` with
Cargo and opens the sample with paths relative to the repository.
[CodeLLDB configuration reference](https://github.com/vadimcn/codelldb/blob/master/MANUAL.md#cargo-support).

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "name": "Debug lmdred",
      "type": "lldb",
      "request": "launch",
      "cargo": {
        "args": ["build", "--locked", "--bin", "lmdred"],
        "filter": { "name": "lmdred", "kind": "bin" }
      },
      "args": ["sample/requirements.zz", "target/debug-diagram"],
      "cwd": "${workspaceFolder}",
      "env": { "RUST_BACKTRACE": "1" },
      "sourceLanguages": ["rust"]
    }
  ]
}
```

Set a breakpoint in `run` in `src/main.rs`, then press **F5**. Step into
`parsing::parse` to inspect the diagram model, `layout::arrange` to inspect
positions and routes, and `export::png` or `export::xml` to inspect output.
When selecting the GNU toolchain explicitly in the debug configuration,
change the Cargo arguments to
`["+stable-x86_64-pc-windows-gnu", "build", "--locked", "--bin", "lmdred"]`.

### Test and check

Run the Rust tests, formatter, and linter from the repository root.

```sh
cargo test --locked
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
```

The Rust tests cover parsing and references, markers and branches, backward
compatibility, obstacle routing, and text wrapping and truncation.

To check the CLI and installers, build a release binary and pass its path
to the integration check.

```sh
cargo build --locked --release
python tests/test_cli_install.py --binary target/release/lmdred
```

On Windows, use `target/release/lmdred.exe`. For an explicit Rust target,
use `target/<target>/release/lmdred` or `lmdred.exe`. The integration check
renders from outside the repository, exercises paths with spaces and output
defaults, and serves a release archive over local HTTP. It installs into a
temporary directory, reinstalls, and checks that a corrupt checksum is
rejected while preserving the installed executable.

### Build and package

```sh
cargo build --locked
cargo build --locked --release
```

Development binaries are written to `target/debug/` and optimized binaries
to `target/release/`. The executable is `lmdred` on Linux and macOS and
`lmdred.exe` on Windows.

On Windows, build the target and then package it using Windows PowerShell
5.1. Use `pwsh` in place of `powershell.exe` if you use PowerShell 7.

```powershell
$compilerTarget = ((rustc -vV | Select-String '^host: ').Line -split ': ', 2)[1]
cargo build --locked --release --target $compilerTarget
powershell.exe -NoProfile -ExecutionPolicy Bypass -File .\scripts\package-release.ps1 -Target $compilerTarget
```

The commands use the active Rust toolchain's target for both steps. For an
x64 Windows toolchain, this is `x86_64-pc-windows-msvc` or
`x86_64-pc-windows-gnu`. The executable is in `target/<target>/release/`.
The archive and its `.sha256` file are written to `target/dist/`. Each
archive includes the executable, license, README, and `sample.zz`.
Unix targets use `.tar.gz` archives.

### Publish a compiler release

The release workflow builds and tests native binaries for x64 and ARM64 on
Windows, macOS, and Linux. It uploads platform archives, their SHA-256
checksums, both installer scripts, and a combined `SHA256SUMS` file.

1. Run the **Version Bump** workflow from the repository's **Actions** tab
   ([`.github/workflows/version-bump.yml`](../../.github/workflows/version-bump.yml))
   and select `patch`, `minor`, or `major`. It opens a pull request that bumps
   `Cargo.toml`, syncs `Cargo.lock`, updates `CHANGELOG.md`, updates the
   default version in `scripts/install.sh` and `scripts/install.ps1`, and
   updates the versioned download URLs and installation commands in this README
   and its [localized copies](../). Repository settings must allow GitHub
   Actions to create pull requests.
2. Review and merge the version-bump pull request into `main`. Tagging is
   intentionally left disabled in the bump workflow so the release tag is
   created only after the PR lands on `main`.
3. After the PR is merged, pull `main` and create and push the
   `lmdred-v<version>` tag pointing to the merged release commit (matching
   `Cargo.toml`):

```sh
git switch main
git pull origin main
git tag lmdred-v1.2.0
git push origin lmdred-v1.2.0
```

Pushing that tag triggers
[`.github/workflows/release-cli.yml`](../../.github/workflows/release-cli.yml),
which verifies the tag against `Cargo.toml` and both installers and creates the
compiler release after all builds pass. Publishing a GitHub release manually
with the same compiler tag also starts the workflow and attaches the assets to
that release. The workflow's **Run workflow** button builds downloadable Actions
artifacts for the selected branch.

If you prepare a release manually instead of using the **Version Bump**
workflow, update `Cargo.toml`, `Cargo.lock`, `CHANGELOG.md`, the default version
in both installers, and the versioned installation commands in the READMEs
before merging and pushing the matching tag. Compiler tags use `lmdred-v`. The
VS Code extension uses `drizzle-` tags and its own version in
`drizzle/package.json`.

Verify that Lamda RED Preview works with the new compiler before deciding
whether to publish a companion update.
[Lamda RED Preview release steps](../../vscode-preview/README.md#release-alongside-a-compiler-update).

## Roadmap

- Create a state machine diagram for the Lamda RED Pest grammar.
- Build a web service with a React app that requests rendered PNGs from a Rust server.

## License

This project is licensed under the [GNU GPLV3](../../LICENSE).
