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
    invalid = work / "invalid.zz"
    invalid.write_text("Missing __> Absent\n")
    run([binary, invalid, "failed"], cwd=work, succeeds=False)
    assert not (work / "failed.png").exists()
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
            else:
                for profile in [home / ".profile", home / ".bashrc"]:
                    assert profile.read_text().count("# Lamda RED compiler") == 1
                resolved = run(
                    ["sh", "-c", '. "$1"; command -v lmdred', "test", str(home / ".profile")],
                    cwd=work, env=env,
                ).stdout.strip()
                assert Path(resolved) == installed
                # Check startup configuration for the other supported interactive shells.
                for shell, files in [
                    ("/bin/zsh", [home / "zsh/.zshrc", home / "zsh/.zprofile"]),
                    ("/usr/bin/fish", [home / ".config/fish/conf.d/lmdred.fish"]),
                ]:
                    env["SHELL"] = shell
                    run(command, cwd=work, env=env)
                    assert all(path.is_file() for path in files)
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
    print("CLI rendering, paths, installer downloads, reinstalls, and checksum rejection passed.")


if __name__ == "__main__":
    main()
