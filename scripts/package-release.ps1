param(
    [Parameter(Mandatory = $true)]
    [string] $Target,
    [string] $OutputDirectory = 'target/dist'
)

$ErrorActionPreference = 'Stop'
$repoRoot = Split-Path -Parent $PSScriptRoot
$manifest = Get-Content -LiteralPath (Join-Path $repoRoot 'Cargo.toml') -Raw
if ($manifest -notmatch '(?m)^version = "([^"]+)"') {
    throw 'Cannot read the compiler version from Cargo.toml.'
}
$version = $Matches[1]
$windows = $Target -match '-windows-'
if ($Target -notmatch '^(x86_64|aarch64)-(pc-windows-msvc|pc-windows-gnu|unknown-linux-musl|apple-darwin)$') {
    throw "Unsupported release target: $Target"
}
$executable = if ($windows) { 'lmdred.exe' } else { 'lmdred' }
$binary = Join-Path $repoRoot "target/$Target/release/$executable"
if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
    throw "Build the binary first with cargo build --locked --release --target $Target"
}
$output = [IO.Path]::GetFullPath($OutputDirectory)
$null = New-Item -ItemType Directory -Force -Path $output
$stage = Join-Path ([IO.Path]::GetTempPath()) ("lmdred-package-" + [guid]::NewGuid())
$null = New-Item -ItemType Directory -Path $stage
try {
    Copy-Item -LiteralPath $binary -Destination (Join-Path $stage $executable)
    foreach ($file in @('LICENSE', 'README.md', 'sample.zz')) {
        Copy-Item -LiteralPath (Join-Path $repoRoot $file) -Destination $stage
    }
    $name = "lmdred-v$version-$Target"
    if ($windows) {
        $archive = Join-Path $output "$name.zip"
        Compress-Archive -Path (Join-Path $stage '*') -DestinationPath $archive -Force
    } else {
        $archive = Join-Path $output "$name.tar.gz"
        & tar -czf $archive -C $stage $executable LICENSE README.md sample.zz
        if ($LASTEXITCODE -ne 0) { throw 'Archive creation failed.' }
    }
    $hash = (Get-FileHash -LiteralPath $archive -Algorithm SHA256).Hash.ToLowerInvariant()
    "$hash  $([IO.Path]::GetFileName($archive))" |
        Set-Content -LiteralPath "$archive.sha256" -Encoding ascii
    Write-Output $archive
} finally {
    $resolvedStage = [IO.Path]::GetFullPath($stage)
    $tempRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd('\', '/') +
        [IO.Path]::DirectorySeparatorChar
    if (-not $resolvedStage.StartsWith($tempRoot, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Staging directory is outside the expected root: $resolvedStage"
    }
    Remove-Item -LiteralPath $resolvedStage -Recurse -Force
}
