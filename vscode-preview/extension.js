const { spawn } = require("node:child_process");
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const vscode = require("vscode");

function activate(context) {
  let preview;

  function showPreview(document, reveal = false) {
    const uri = document.uri.toString();
    const fileName = path.basename(document.fileName);

    if (!preview) {
      const panel = vscode.window.createWebviewPanel(
        "lamdaRed.preview",
        "Lamda RED Preview",
        { viewColumn: vscode.ViewColumn.Beside, preserveFocus: true },
        { localResourceRoots: [] }
      );
      preview = { panel };
      panel.onDidDispose(() => {
        preview.build?.abort();
        preview = undefined;
      });
    } else if (reveal || preview.uri !== uri) {
      preview.panel.reveal(preview.panel.viewColumn, true);
    }

    const current = preview;
    current.build?.abort();
    if (current.uri !== uri) {
      current.uri = uri;
      current.image = undefined;
    }
    current.panel.title = `Lamda RED: ${fileName}`;
    current.build = new AbortController();
    const { signal } = current.build;
    const config = vscode.workspace.getConfiguration("lamdaRed", document.uri);
    const executable = config.get("compilerPath").trim();
    const source = document.getText();
    current.panel.webview.html = previewHtml(fileName, current.image, "Compiling…");

    return render();

    async function render() {
      try {
        const png = await compile(executable, fileName, source, signal);
        if (signal.aborted) return;
        current.image = png.toString("base64");
        current.panel.webview.html = previewHtml(fileName, current.image);
      } catch (error) {
        if (signal.aborted) return;
        current.panel.webview.html = previewHtml(
          fileName, current.image, undefined, error.message
        );
      }
    }
  }

  context.subscriptions.push(
    vscode.commands.registerCommand("lamdaRed.showPreview", () => {
      const document = vscode.window.activeTextEditor?.document;
      if (!isDrizzleFile(document)) {
        return vscode.window.showInformationMessage("Open a Drizzle file to preview it.");
      }
      return showPreview(document, true);
    }),
    vscode.workspace.onDidSaveTextDocument(document => {
      if (!isDrizzleFile(document)) return;
      const uri = document.uri.toString();
      const activeUri = vscode.window.activeTextEditor?.document.uri.toString();
      if (uri !== activeUri && uri !== preview?.uri) return;
      if (vscode.workspace.getConfiguration("lamdaRed", document.uri).get("previewOnSave")) {
        void showPreview(document);
      }
    }),
    { dispose: () => preview?.panel.dispose() }
  );
}

function isDrizzleFile(document) {
  return document?.languageId === "drizzle" && document.uri.scheme === "file";
}

async function compile(executable, fileName, source, signal) {
  if (executable && !path.isAbsolute(executable)) {
    throw new Error("Lamda RED: Compiler Path must be an absolute path to lmdred.");
  }

  const directory = await fs.mkdtemp(path.join(os.tmpdir(), "lamda-red-"));
  try {
    const input = path.join(directory, "source.zz");
    const output = path.join(directory, "diagram");
    await fs.writeFile(input, source, { signal });
    await runCompiler(executable || "lmdred", directory, signal);
    return await fs.readFile(`${output}.png`, { signal });
  } catch (error) {
    if (error.code === "ENOENT" && error.syscall?.startsWith("spawn")) {
      throw new Error("Install lmdred and restart VS Code, or set Lamda RED: Compiler Path.");
    }
    throw new Error(error.message.replace(/^source\.zz:/, () => `${fileName}:`));
  } finally {
    await fs.rm(directory, { recursive: true, force: true, maxRetries: 3 });
  }
}

function runCompiler(executable, directory, signal) {
  return new Promise((resolve, reject) => {
    signal.throwIfAborted();
    const child = spawn(executable, ["source.zz", "diagram"], {
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
