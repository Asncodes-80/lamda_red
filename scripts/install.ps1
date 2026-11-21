param(
    [string] $Version = $(if ($env:LMDRED_VERSION) { $env:LMDRED_VERSION } else { '1.1.0' }),
    [string] $InstallDirectory = $(if ($env:LMDRED_INSTALL_DIR) {
        $env:LMDRED_INSTALL_DIR
    } else {
        Join-Path $env:LOCALAPPDATA 'Programs\lmdred\bin'
    }),
    [switch] $NoModifyPath
)

$ErrorActionPreference = 'Stop'

function Get-LmdredPathFirst {
    param([string] $Path, [string] $Directory)

    $normalizedDirectory = $Directory.Replace('/', '\').TrimEnd('\')
    $remaining = foreach ($entry in ($Path -split ';')) {
        if ($entry) {
            $expanded = [Environment]::ExpandEnvironmentVariables($entry.Trim().Trim('"'))
            if ($expanded.Replace('/', '\').TrimEnd('\') -ne $normalizedDirectory) {
                $entry
            }
        }
    }
    (@($Directory) + @($remaining) | Where-Object { $_ }) -join ';'
}

if ($env:OS -ne 'Windows_NT') {
    throw 'Use install.sh on Linux and macOS.'
}
$Version = $Version -replace '^lmdred-v', ''
if ($Version -notmatch '^\d+\.\d+\.\d+$') {
    throw 'Use a numeric release version such as 1.1.0.'
}
$processor = if ($env:PROCESSOR_ARCHITEW6432) {
    $env:PROCESSOR_ARCHITEW6432
} else {
    $env:PROCESSOR_ARCHITECTURE
}
$architecture = switch ($processor) {
    'AMD64' { 'x86_64' }
    'ARM64' { 'aarch64' }
    default { throw "Unsupported processor: $processor. Supported architectures are x64 and ARM64." }
}
$name = "lmdred-v$Version-$architecture-pc-windows-msvc.zip"
$baseUrl = if ($env:LMDRED_RELEASE_BASE_URL) {
    $env:LMDRED_RELEASE_BASE_URL.TrimEnd('/')
} else {
    'https://github.com/Asncodes-80/lamda_red/releases/download'
}
$url = "$baseUrl/lmdred-v$Version"
$temporary = Join-Path ([IO.Path]::GetTempPath()) ("lmdred-install-" + [guid]::NewGuid())
$null = New-Item -ItemType Directory -Path $temporary
$staged = $null
try {
    [Net.ServicePointManager]::SecurityProtocol = [Net.ServicePointManager]::SecurityProtocol -bor
        [Net.SecurityProtocolType]::Tls12
    Write-Host "Downloading lmdred $Version for Windows $architecture..."
    $archive = Join-Path $temporary $name
    try {
        Invoke-WebRequest -UseBasicParsing -Uri "$url/$name" -OutFile $archive
        $checksum = (Invoke-WebRequest -UseBasicParsing -Uri "$url/$name.sha256").Content
    } catch {
        throw "Could not download compiler release lmdred-v$Version from $url. $($_.Exception.Message)"
    }
    if ($checksum -is [byte[]]) {
        $checksum = [Text.Encoding]::UTF8.GetString($checksum)
    }
    if ($checksum -notmatch '^\s*([a-fA-F0-9]{64})(?:\s|$)') {
        throw 'Invalid release checksum.'
    }
    $expected = $Matches[1]
    $sha256 = [Security.Cryptography.SHA256]::Create()
    $stream = [IO.File]::OpenRead($archive)
    try {
        $actual = [BitConverter]::ToString($sha256.ComputeHash($stream)).Replace('-', '')
    } finally {
        $stream.Dispose()
        $sha256.Dispose()
    }
    if ($actual -ne $expected) {
        throw 'SHA-256 verification failed.'
    }
    $unpacked = Join-Path $temporary 'unpacked'
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    [IO.Compression.ZipFile]::ExtractToDirectory($archive, $unpacked)
    $binary = Join-Path $unpacked 'lmdred.exe'
    if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
        throw 'The release archive has no lmdred.exe.'
    }
    & $binary --version
    if ($LASTEXITCODE -ne 0) { throw 'The downloaded compiler could not run.' }
    $install = [IO.Path]::GetFullPath($InstallDirectory)
    $null = New-Item -ItemType Directory -Force -Path $install
    $destination = Join-Path $install 'lmdred.exe'
    $staged = Join-Path $install ('.lmdred-' + [guid]::NewGuid() + '.exe')
    Copy-Item -LiteralPath $binary -Destination $staged
    if (Test-Path -LiteralPath $destination) {
        [IO.File]::Replace($staged, $destination, [NullString]::Value)
    } else {
        [IO.File]::Move($staged, $destination)
    }
    if (-not $NoModifyPath) {
        $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
        $preferredPath = Get-LmdredPathFirst -Path $userPath -Directory $install
        if ($preferredPath -cne $userPath) {
            [Environment]::SetEnvironmentVariable('Path', $preferredPath, 'User')
        }
        $env:Path = Get-LmdredPathFirst -Path $env:Path -Directory $install
        Write-Host 'Installed lmdred and put its directory first in your user PATH.'
        Write-Host 'Open a new terminal to pick up the updated PATH.'

        # Windows searches the machine PATH before the user PATH in a new session.
        foreach ($entry in ([Environment]::GetEnvironmentVariable('Path', 'Machine') -split ';')) {
            $directory = [Environment]::ExpandEnvironmentVariables($entry.Trim().Trim('"'))
            if (-not [IO.Path]::IsPathRooted($directory)) { continue }
            $candidate = Join-Path $directory 'lmdred.exe' -ErrorAction SilentlyContinue
            if ($candidate -eq $destination) { break }
            if ([IO.File]::Exists($candidate)) {
                Write-Warning "The system PATH may still select $candidate. Run '$destination' directly or update the system installation."
                break
            }
        }
    } else {
        Write-Host "Installed $destination. Add $install to PATH to call lmdred by name."
    }
    Write-Host 'Run: lmdred sample.zz my-new-red'
} finally {
    if ($staged -and (Test-Path -LiteralPath $staged)) {
        Remove-Item -LiteralPath $staged -Force
    }
    $resolvedTemp = [IO.Path]::GetFullPath($temporary)
    $tempRoot = [IO.Path]::GetFullPath([IO.Path]::GetTempPath()).TrimEnd('\', '/') +
        [IO.Path]::DirectorySeparatorChar
    if (-not $resolvedTemp.StartsWith($tempRoot, [StringComparison]::OrdinalIgnoreCase)) {
        throw "Temporary directory is outside the expected root: $resolvedTemp"
    }
    Remove-Item -LiteralPath $resolvedTemp -Recurse -Force
}
