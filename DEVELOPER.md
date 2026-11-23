# Developing Lamda RED

Lamda RED compiles Drizzle source files into diagrams. The `lmdred` CLI, the
Drizzle syntax extension, and the Lamda RED Preview extension live in this
repository. See the [Drizzle syntax guide](./README.md#drizzle-syntax) for the
language and the [contribution guide](./CONTRIBUTING.md) for opening a pull request.

## Set up the source

Install [stable Rust and Cargo](https://www.rust-lang.org/tools/install), Git,
and the native build tools for your system. Use the Visual Studio C++ build
tools for Windows MSVC, the Xcode command-line tools for macOS, or a C compiler
and linker for Linux. PNG rendering uses system fonts. On a minimal Linux system,
install a font such as DejaVu Sans.

```sh
git clone https://github.com/Asncodes-80/lamda_red.git
cd lamda_red
rustup component add rustfmt clippy
cargo build --locked
cargo run --locked -- sample/object-agent.zz target/object-agent
```

The sample command writes `target/object-agent.png` and `target/object-agent.xml`.
Use an output directory that already exists. To use the GNU toolchain on Windows,
follow the [Windows GNU setup](./README.md#set-up-the-source) and pass the same
toolchain selector to the Cargo commands below.

## Find your way around the code

| Path | Responsibility |
| --- | --- |
| [`src/main.rs`](./src/main.rs) | Connects CLI arguments, file paths, diagnostics, and the compilation pipeline |
| [`src/lamda_red_grammar.pest`](./src/lamda_red_grammar.pest) and [`src/parsing.rs`](./src/parsing.rs) | Define the grammar and turn source text into nodes and connections |
| [`src/labels.rs`](./src/labels.rs) | Discovers system fonts and shapes, measures, and wraps text |
| [`src/layout.rs`](./src/layout.rs) and [`src/routing.rs`](./src/routing.rs) | Position shapes and route connections and their labels |
| [`src/export.rs`](./src/export.rs) | Writes PNG and editable mxGraph XML output |
| [`sample/`](./sample/) | Holds reference diagrams for language features |
| [`drizzle/`](./drizzle/) | Provides Drizzle syntax highlighting in VS Code |
| [`vscode-preview/`](./vscode-preview/) | Runs `lmdred` from VS Code and shows the result beside the editor |
| [`scripts/`](./scripts/) and [`release-cli.yml`](./.github/workflows/release-cli.yml) | Build, test, package, and publish compiler releases |
| [`tests/test_cli_install.py`](./tests/test_cli_install.py) | Exercises the compiled CLI and installers |

## Build and check changes

Run these commands from the repository root.

```sh
cargo fmt --check
cargo clippy --locked --all-targets -- -D warnings
cargo test --locked
```

For changes to the CLI or the installers, use Python 3 to check a release build.

```sh
cargo build --locked --release
python tests/test_cli_install.py --binary target/release/lmdred
```

On Windows, pass `target/release/lmdred.exe`. When you build with `--target`, use
the executable under `target/<target>/release/`. The integration check serves
test release assets over local HTTP and installs into a temporary directory.
See the [packaging](./README.md#build-and-package) and
[release instructions](./README.md#publish-a-compiler-release) for publishing.

## Debug the compiler and preview

- Reproduce compiler behavior with a small `.zz` file. For a syntax or reference
  error, start at the reported file, line, and column. To step through the
  compiler in VS Code, use the [CodeLLDB setup](./README.md#run-and-debug).
- Open `drizzle/` in desktop VS Code and press **F5** to try syntax changes in an
  Extension Development Host. Open `vscode-preview/` instead to load both
  extensions and test rendering and export.
- To test a local compiler in the preview, set `lamdaRed.compilerPath` in the
  development host to the executable's absolute path. Save a sample to check live
  rendering, then use **Export diagram** to check the PNG and XML outputs.
  See the [extension development guide](./vscode-preview/README.md#develop-and-package).

## Environment variables

A local compiler build needs no project-specific environment variables.

| Variable | Use |
| --- | --- |
| `RUST_BACKTRACE=1` | Show a Rust stack trace if the compiler panics |
| `LMDRED_VERSION` | Set the default release version for the installers |
| `LMDRED_INSTALL_DIR` | Set the default binary install directory |
| `LMDRED_RELEASE_BASE_URL` | Point the installers at a test release server |

Explicit installer arguments take precedence over version and directory defaults.
See the [installation guide](./README.md#installation) for platform examples.
