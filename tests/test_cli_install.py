"""Exercise the installed CLI and release installers using a local HTTP server."""

import argparse
import functools
import hashlib
import http.server
import os
from pathlib import Path
import platform
import re
import shutil
import struct
import subprocess
import tarfile
import tempfile
import threading
import xml.etree.ElementTree as ET
import zipfile


REPO = Path(__file__).resolve().parents[1]
VERSION = re.search(r'^version = "([^"]+)"', (REPO / "Cargo.toml").read_text(), re.M)[1]


def run(command, *, cwd, env=None, succeeds=True):
    result = subprocess.run(command, cwd=cwd, env=env, capture_output=True, text=True)
    if succeeds and result.returncode != 0:
        raise AssertionError(f"{command}\n{result.stdout}\n{result.stderr}")
    if not succeeds and result.returncode == 0:
        raise AssertionError(f"Expected a failure: {command}")
    return result


def check_cli(binary, work):
    assert run([binary, "--version"], cwd=work).stdout.strip() == f"lmdred {VERSION}"
    assert "Usage: lmdred" in run([binary, "--help"], cwd=work).stdout
    assert "Unknown option" in run([binary, "--unknown"], cwd=work, succeeds=False).stderr
    run([binary, "a", "b", "c"], cwd=work, succeeds=False)
    source = work / "diagram with spaces.zz"
    shutil.copyfile(REPO / "sample/requirements.zz", source)
    for stem, arguments in [
        ("my-new-red", [source.name, "my-new-red"]),
        ("version.1", [source.name, "version.1"]),
        ("output", [source.name]),
        ("--diagram", ["--", source.name, "--diagram"]),
    ]:
        run([binary, *arguments], cwd=work)
        png = (work / f"{stem}.png").read_bytes()
        assert png[:8] == b"\x89PNG\r\n\x1a\n"
        assert all(struct.unpack(">II", png[16:24]))
        assert ET.parse(work / f"{stem}.xml").getroot().tag == "mxGraphModel"
    shutil.copyfile(source, work / "sample.zz")
    run([binary], cwd=work)

    run([binary, "--export-set", source, "exports"], cwd=work)
    compact, document, high = [
        (work / f"exports-{variant}.png").read_bytes()
        for variant in ("compact", "document", "high")
    ]
    assert compact == (work / "output.png").read_bytes()
    width, height = struct.unpack(">II", compact[16:24])
    assert struct.unpack(">II", document[16:24]) == (2 * width, 2 * height)
    assert struct.unpack(">II", high[16:24]) == (4 * width, 4 * height)
    assert len(compact) < len(document) < len(high)
    assert (work / "exports.xml").read_bytes() == (work / "output.xml").read_bytes()
    assert not (work / "exports.png").exists()

    invalid = work / "invalid.zz"
    invalid.write_text("Missing __> Absent\n")
    run([binary, invalid, "failed"], cwd=work, succeeds=False)
    assert not (work / "failed.png").exists()
    run([binary, "--export-set", invalid, "exports"], cwd=work, succeeds=False)
    assert (work / "exports-high.png").read_bytes() == high
    run([binary, "missing.zz"], cwd=work, succeeds=False)
    # Protect a source whose extension happens to be an output extension.
    protected = work / "source.png"
    shutil.copyfile(source, protected)
    before = protected.read_bytes()
    run([binary, protected, "source"], cwd=work, succeeds=False)
    assert protected.read_bytes() == before


class QuietHandler(http.server.SimpleHTTPRequestHandler):
    def log_message(self, *args):
        pass


def check_windows_path_priority(shell, env, destination, work):
    # Load the PATH helper without updating the account's registry in a test.
    script = r"""
$ErrorActionPreference = 'Stop'
$tokens = $null
$errors = $null
$installer = [Management.Automation.Language.Parser]::ParseFile(
    $env:LMDRED_TEST_INSTALLER, [ref]$tokens, [ref]$errors
)
if ($errors) { throw $errors[0].Message }
$helper = $installer.EndBlock.Statements | Where-Object { $_.Name -eq 'Get-LmdredPathFirst' }
if (-not $helper) { throw 'PATH helper not found.' }
. ([ScriptBlock]::Create($helper.Extent.Text))
$install = $env:LMDRED_TEST_INSTALL_DIR
$previous = $env:LMDRED_TEST_LEGACY_DIR
$original = "$previous;C:\Other;" + '"' + '%LMDRED_TEST_INSTALL_DIR%\' + '"' + ";$install"
$expected = "$install;$previous;C:\Other"
$actual = Get-LmdredPathFirst -Path $original -Directory $install
if ($actual -cne $expected) { throw "Unexpected PATH order: $actual" }
if ((Get-LmdredPathFirst -Path $actual -Directory $install) -cne $expected) {
    throw 'Reinstall changed the already prioritized PATH.'
}
$env:Path = "$actual;$env:Path"
$command = (Get-Command lmdred.exe).Source
if ($command -ne (Join-Path $install 'lmdred.exe')) {
    throw "PATH resolved to the wrong lmdred: $command"
}
& $command --version
exit $LASTEXITCODE
"""
    legacy = work / "older cargo bin"
    legacy.mkdir(exist_ok=True)
    (legacy / "lmdred.exe").write_bytes(b"older installation")
    env = {
        **env,
        "LMDRED_TEST_INSTALLER": str(REPO / "scripts/install.ps1"),
        "LMDRED_TEST_INSTALL_DIR": str(destination),
        "LMDRED_TEST_LEGACY_DIR": str(legacy),
    }
    result = run([shell, "-NoProfile", "-Command", script], cwd=work, env=env)
    assert result.stdout.strip() == f"lmdred {VERSION}"


def check_installer(binary, work):
    windows = os.name == "nt"
    if windows:
        processor = os.environ.get("PROCESSOR_ARCHITEW6432") or os.environ["PROCESSOR_ARCHITECTURE"]
        arch = {"AMD64": "x86_64", "ARM64": "aarch64"}[processor.upper()]
        target = f"{arch}-pc-windows-msvc"
        executable = "lmdred.exe"
        extension = ".zip"
    else:
        arch = {"x86_64": "x86_64", "arm64": "aarch64", "aarch64": "aarch64"}[platform.machine()]
        target = f"{arch}-" + ("apple-darwin" if platform.system() == "Darwin" else "unknown-linux-musl")
        executable = "lmdred"
        extension = ".tar.gz"

    assets = work / "releases" / f"lmdred-v{VERSION}"
    assets.mkdir(parents=True)
    archive = assets / f"lmdred-v{VERSION}-{target}{extension}"
    if windows:
        with zipfile.ZipFile(archive, "w", zipfile.ZIP_DEFLATED) as package:
            package.write(binary, executable)
    else:
        with tarfile.open(archive, "w:gz") as package:
            package.add(binary, arcname=executable)
    digest = hashlib.sha256(archive.read_bytes()).hexdigest()
    checksum = Path(f"{archive}.sha256")
    checksum.write_text(f"{digest}  {archive.name}\n", encoding="ascii")

    handler = functools.partial(QuietHandler, directory=str(work / "releases"))
    with http.server.ThreadingHTTPServer(("127.0.0.1", 0), handler) as server:
        thread = threading.Thread(target=server.serve_forever, daemon=True)
        thread.start()
        try:
            env = os.environ.copy()
            env["LMDRED_RELEASE_BASE_URL"] = f"http://127.0.0.1:{server.server_port}"
            home = work / "user's home"
            home.mkdir()
            destination = home / "local bin"
            if not windows:
                # macOS temporary paths can alias /private/var through /var.
                # Exercise an alias on every Unix runner to cover that case.
                alias = work / "home-link"
                alias.symlink_to(home, target_is_directory=True)
                destination = alias / "local bin"
            env["HOME"] = str(home)
            env["SHELL"] = "/bin/bash"
            env["ZDOTDIR"] = str(home / "zsh")
            env["XDG_CONFIG_HOME"] = str(home / ".config")
            if windows:
                # Keep automated tests out of the account's persistent registry PATH.
                command = [
                    "pwsh", "-NoProfile", "-File", str(REPO / "scripts/install.ps1"),
                    "-InstallDirectory", str(destination), "-Version", VERSION, "-NoModifyPath",
                ]
            else:
                command = [
                    "sh", str(REPO / "scripts/install.sh"),
                    "--bin-dir", str(destination), "--version", VERSION,
                ]
            run(command, cwd=work, env=env)
            installed = destination / executable
            assert installed.read_bytes() == binary.read_bytes()
            run(command, cwd=work, env=env)  # Reinstalling replaces the file consistently.
            if windows:
                run(["powershell", *command[1:]], cwd=work, env=env)
                for shell in ("pwsh", "powershell"):
                    check_windows_path_priority(shell, env, destination, work)
            else:
                legacy = work / "older cargo bin"
                legacy.mkdir()
                legacy_binary = legacy / "lmdred"
                legacy_binary.write_text("#!/bin/sh\nprintf 'lmdred 0.0.0\\n'\n")
                legacy_binary.chmod(0o755)
                env["PATH"] = os.pathsep.join([str(legacy), str(installed.parent), env["PATH"]])
                for profile in [home / ".profile", home / ".bashrc"]:
                    assert profile.read_text().count("# Lamda RED compiler") == 1
                resolved = run(
                    ["sh", "-c", '. "$1"; command -v lmdred', "test", str(home / ".profile")],
                    cwd=work, env=env,
                ).stdout.strip()
                assert installed.samefile(resolved), (
                    f"PATH resolved to {resolved}, expected the installed file {installed}"
                )
                # Check startup configuration for the other supported interactive shells.
                for shell, files in [
                    ("/bin/zsh", [home / "zsh/.zshrc", home / "zsh/.zprofile"]),
                    ("/usr/bin/fish", [home / ".config/fish/conf.d/lmdred.fish"]),
                ]:
                    env["SHELL"] = shell
                    run(command, cwd=work, env=env)
                    assert all(path.is_file() for path in files)
                if shutil.which("fish"):
                    fish_command = [
                        "fish", "--no-config", "-c",
                        'source "$argv[1]"; command -s lmdred',
                        str(home / ".config/fish/conf.d/lmdred.fish"),
                    ]
                    resolved = run(fish_command, cwd=work, env=env).stdout.strip()
                    assert installed.samefile(resolved)
            check_cli(str(installed), work)
            # A corrupt download must leave an existing installation usable.
            checksum.write_text("0" * 64 + f"  {archive.name}\n", encoding="ascii")
            rejected = run(command, cwd=work, env=env, succeeds=False)
            assert "SHA-256 verification failed" in rejected.stderr
            assert installed.read_bytes() == binary.read_bytes()
            run([str(installed), "--version"], cwd=work)
        finally:
            server.shutdown()
            thread.join()


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--binary", type=Path, required=True)
    args = parser.parse_args()
    binary = args.binary.resolve(strict=True)
    with tempfile.TemporaryDirectory(prefix="lmdred-cli-") as temporary:
        work = Path(temporary)
        check_cli(str(binary), work)
        check_installer(binary, work)
    print("CLI rendering, installer downloads, reinstalls, PATH priority, and checksum rejection passed.")


if __name__ == "__main__":
    main()
