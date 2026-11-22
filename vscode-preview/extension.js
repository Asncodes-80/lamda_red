const { spawn } = require("node:child_process");
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const vscode = require("vscode");

const EXPORT_SUFFIXES = [".xml", "-compact.png", "-document.png", "-high.png"];

function activate(context) {
  let preview;
  const saves = new Map();
  let saveQueue = Promise.resolve();
  const output = vscode.window.createOutputChannel("Lamda RED");

  function buildDiagram(document, onSave = false) {
    const uri = document.uri.toString();
    const fileName = path.basename(document.fileName);
    const activeUri = vscode.window.activeTextEditor?.document.uri.toString();
    const show = !onSave || uri === activeUri || uri === preview?.uri;
    const build = { onSave, controller: new AbortController() };
    const { signal } = build.controller;
    const config = vscode.workspace.getConfiguration("lamdaRed", document.uri);
    const executable = config.get("compilerPath").trim();
    const source = document.getText();

    if (show) showPreview(uri, fileName, build);
    if (!onSave) return render();

    saves.get(uri)?.controller.abort();
    saves.set(uri, build);
    saveQueue = saveQueue.then(render);
    return saveQueue;

    async function render() {
      try {
        signal.throwIfAborted();
        const png = await compile(executable, document.fileName, source, onSave, signal);
        if (signal.aborted || preview?.build !== build) return;
        preview.image = png.toString("base64");
        preview.panel.webview.html = previewHtml(fileName, preview.image);
      } catch (error) {
        if (signal.aborted) return;
        output.appendLine(`${document.fileName}\n${error.message}\n`);
        if (preview?.build === build) {
          preview.panel.webview.html = previewHtml(fileName, preview.image, undefined, error.message);
        }
      } finally {
        if (saves.get(uri) === build) saves.delete(uri);
      }
    }
  }

  function showPreview(uri, fileName, build) {
    if (!preview) {
      const panel = vscode.window.createWebviewPanel(
        "lamdaRed.preview",
        "Lamda RED Preview",
        { viewColumn: vscode.ViewColumn.Beside, preserveFocus: true },
        { localResourceRoots: [] }
      );
      preview = { panel };
      panel.onDidDispose(() => {
        if (preview.build && !preview.build.onSave) preview.build.controller.abort();
        preview = undefined;
      });
    } else if (!build.onSave || preview.uri !== uri) {
      preview.panel.reveal(preview.panel.viewColumn, true);
    }

    if (preview.build && !preview.build.onSave) preview.build.controller.abort();
    if (preview.uri !== uri) {
      preview.uri = uri;
      preview.image = undefined;
    }
    preview.build = build;
    preview.panel.title = `Lamda RED: ${fileName}`;
    preview.panel.webview.html = previewHtml(fileName, preview.image, "Compiling…");
  }

  context.subscriptions.push(
    vscode.commands.registerCommand("lamdaRed.showPreview", () => {
      const document = vscode.window.activeTextEditor?.document;
      if (!isDrizzleFile(document)) {
        return vscode.window.showInformationMessage("Open a Drizzle file to preview it.");
      }
      return buildDiagram(document);
    }),
    vscode.workspace.onDidSaveTextDocument(document => {
      if (!isDrizzleFile(document)) return;
      if (vscode.workspace.getConfiguration("lamdaRed", document.uri).get("previewOnSave")) {
        void buildDiagram(document, true);
      }
    }),
    {
      dispose() {
        for (const build of saves.values()) build.controller.abort();
        preview?.panel.dispose();
        output.dispose();
      }
    }
  );
}

function isDrizzleFile(document) {
  return document?.languageId === "drizzle" && document.uri.scheme === "file";
}

async function compile(executable, sourcePath, source, onSave, signal) {
  if (executable && !path.isAbsolute(executable)) {
    throw new Error("Lamda RED: Compiler Path must be an absolute path to lmdred.");
  }

  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "lamda-red-"));
  try {
    await fs.writeFile(path.join(directory, "source.zz"), source, { signal });
    await runCompiler(executable || "lmdred", directory, onSave, signal);
    const previewPng = onSave ? "diagram-compact.png" : "diagram.png";
    const png = await fs.readFile(path.join(directory, previewPng), { signal });
    if (onSave) await saveExports(directory, sourcePath, signal);
    return png;
  } catch (error) {
    if (error.code === "ENOENT" && error.syscall?.startsWith("spawn")) {
      throw new Error("Install lmdred and restart VS Code, or set Lamda RED: Compiler Path.");
    }
    if (onSave && error.message.startsWith("Unknown option: --export-set")) {
      throw new Error("Update lmdred to a build that supports --export-set to export on save.");
    }
    throw new Error(error.message.replace(/^source\.zz:/, () => `${path.basename(sourcePath)}:`));
  } finally {
    await fs.rm(directory, { recursive: true, force: true, maxRetries: 3 });
  }
}

async function saveExports(compiled, sourcePath, signal) {
  signal.throwIfAborted();
  const destination = path.join(path.dirname(sourcePath), "lmdred-out");
  const fileName = path.basename(sourcePath);
  await fs.mkdir(destination, { recursive: true });
  // Stage in the destination filesystem so each replacement is a rename.
  const staging = await fs.mkdtemp(path.join(destination, ".export-"));
  try {
    for (const suffix of EXPORT_SUFFIXES) {
      const target = path.join(destination, fileName + suffix);
      const entry = await fs.lstat(target).catch(error => {
        if (error.code !== "ENOENT") throw error;
      });
      if (entry && !entry.isFile() && !entry.isSymbolicLink()) {
        throw new Error(`Cannot replace output: ${target}`);
      }
      await fs.copyFile(path.join(compiled, "diagram" + suffix), path.join(staging, suffix));
    }
    signal.throwIfAborted();
    for (const suffix of EXPORT_SUFFIXES) {
      await fs.rename(path.join(staging, suffix), path.join(destination, fileName + suffix));
    }
  } finally {
    await fs.rm(staging, { recursive: true, force: true, maxRetries: 3 });
  }
}

function runCompiler(executable, directory, onSave, signal) {
  return new Promise((resolve, reject) => {
    signal.throwIfAborted();
    const args = ["source.zz", "diagram"];
    if (onSave) args.unshift("--export-set");
    const child = spawn(executable, args, {
      cwd: directory,
      signal,
      stdio: ["ignore", "ignore", "pipe"],
      windowsHide: true
    });
    let failure;
    let stderr = "";
    child.on("error", error => { failure = error; });
    child.stderr.setEncoding("utf8");
    child.stderr.on("data", chunk => {
      stderr += chunk.slice(0, 16384 - stderr.length);
    });
    child.on("close", code => {
      if (failure) reject(failure);
      else if (code === 0) resolve();
      else reject(new Error(stderr.trim() || `lmdred exited with status ${code}.`));
    });
  });
}

function previewHtml(fileName, image, status, error) {
  const escape = value => value.replace(/[&<>"']/g, char => ({
    "&": "&amp;", "<": "&lt;", ">": "&gt;", '"': "&quot;", "'": "&#39;"
  })[char]);
  return `<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <meta http-equiv="Content-Security-Policy" content="default-src 'none'; img-src data:; style-src 'unsafe-inline'">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Lamda RED Preview</title>
  <style>
    body { padding: 1rem; }
    pre { white-space: pre-wrap; overflow-wrap: anywhere; color: var(--vscode-errorForeground); }
    img { display: block; max-width: 100%; height: auto; margin-top: 0.75rem; }
    input:checked ~ img { max-width: none; }
  </style>
</head>
<body>
  ${status ? `<p role="status">${status}</p>` : ""}
  ${error ? `<pre role="alert">${escape(error)}</pre>` : ""}
  ${error && image ? "<p>Showing the last successful render.</p>" : ""}
  ${image ? `<input id="actual-size" type="checkbox"><label for="actual-size">Actual size</label>
  <img alt="Diagram for ${escape(fileName)}" src="data:image/png;base64,${image}">` : ""}
</body>
</html>`;
}

module.exports = { activate };
