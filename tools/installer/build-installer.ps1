# Script automatizado para compilar el instalador nativo setup.exe de YaFaltaPoco
$ErrorActionPreference = "Stop"

$workspaceRoot = Resolve-Path "$PSScriptRoot\..\.."
Set-Location $workspaceRoot

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Compilando setup.exe para YaFaltaPoco   " -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

# Directorios de destino para distribución limpia
$distDir = "$workspaceRoot\dist"
$portableDir = "$distDir\portable"
if (-not (Test-Path $distDir)) { New-Item -ItemType Directory -Path $distDir | Out-Null }
if (-not (Test-Path $portableDir)) { New-Item -ItemType Directory -Path $portableDir | Out-Null }

# Búsqueda inteligente de binarios compilados
$exeCandidates = @(
    "$workspaceRoot\src-tauri\target\release\yafaltapoco.exe",
    "$portableDir\yafaltapoco.exe",
    "$workspaceRoot\yafaltapoco.exe"
)
$exeSource = $null
foreach ($cand in $exeCandidates) {
    if (Test-Path $cand) {
        $exeSource = $cand
        break
    }
}
if (-not $exeSource) {
    throw "No se encontró yafaltapoco.exe. Compila primero con: cargo build --release --manifest-path src-tauri/Cargo.toml"
}

$dllCandidates = @(
    "$workspaceRoot\src-tauri\target\release\WebView2Loader.dll",
    "$workspaceRoot\src-tauri\target\release\build\*\out\x64\WebView2Loader.dll",
    "$portableDir\WebView2Loader.dll",
    "$workspaceRoot\WebView2Loader.dll"
)
$dllSource = $null
foreach ($cand in $dllCandidates) {
    $matched = Get-Item -Path $cand -ErrorAction SilentlyContinue | Select-Object -First 1
    if ($matched -and (Test-Path $matched.FullName)) {
        $dllSource = $matched.FullName
        break
    }
}
if (-not $dllSource) {
    throw "No se encontró WebView2Loader.dll en las carpetas de compilación ni en dist."
}

# Detener instancias en ejecucion para permitir sobreescritura limpia
Get-Process yafaltapoco -ErrorAction SilentlyContinue | Stop-Process -Force -ErrorAction SilentlyContinue
Start-Sleep -Milliseconds 400

# Sincronizar versión portable en dist/portable/
Copy-Item $exeSource -Destination "$portableDir\yafaltapoco.exe" -Force
Copy-Item $dllSource -Destination "$portableDir\WebView2Loader.dll" -Force

$gifSource = "$workspaceRoot\ui\bg-logo.gif"
$icoSource = "$workspaceRoot\src-tauri\icons\icon.ico"
$outputSetup = "$distDir\setup.exe"

if (-not (Test-Path $gifSource)) {
    throw "No se encontró $gifSource."
}
if (-not (Test-Path $icoSource)) {
    throw "No se encontró $icoSource."
}

# Crear directorio temporal de empaquetado
$tempBuild = "$workspaceRoot\tools\installer\obj"
if (Test-Path $tempBuild) {
    Remove-Item $tempBuild -Recurse -Force
}
New-Item -ItemType Directory -Path $tempBuild | Out-Null

Write-Host "[1/3] Comprimiendo binarios con GZip óptimo..." -ForegroundColor Yellow
$appGz = "$tempBuild\app.gz"
$loaderGz = "$tempBuild\loader.gz"

function Compress-FileGzip($sourcePath, $destPath) {
    $bytes = [System.IO.File]::ReadAllBytes($sourcePath)
    $fs = [System.IO.File]::Create($destPath)
    $gz = New-Object System.IO.Compression.GZipStream($fs, [System.IO.Compression.CompressionLevel]::Optimal)
    $gz.Write($bytes, 0, $bytes.Length)
    $gz.Close()
    $fs.Close()
}

Compress-FileGzip "$portableDir\yafaltapoco.exe" $appGz
Compress-FileGzip "$portableDir\WebView2Loader.dll" $loaderGz

$appGzSize = [math]::Round((Get-Item $appGz).Length / 1MB, 2)
Write-Host "  -> yafaltapoco.exe comprimido: $appGzSize MB" -ForegroundColor DarkGray

Write-Host "[2/3] Compilando instalador ejecutable con csc.exe..." -ForegroundColor Yellow
$cscPath = "C:\Windows\Microsoft.NET\Framework64\v4.0.30319\csc.exe"
if (-not (Test-Path $cscPath)) {
    $cscPath = "C:\Windows\Microsoft.NET\Framework\v4.0.30319\csc.exe"
}
if (-not (Test-Path $cscPath)) {
    throw "No se encontró el compilador csc.exe del .NET Framework."
}

$csSource = "$workspaceRoot\tools\installer\Program.cs"

$cscArgs = @(
    "/nologo",
    "/target:winexe",
    "/platform:x64",
    "/optimize+",
    "/win32icon:$icoSource",
    "/resource:$appGz,AppPayload",
    "/resource:$loaderGz,LoaderPayload",
    "/resource:$gifSource,LogoPayload",
    "/resource:$icoSource,IconPayload",
    "/out:$outputSetup",
    $csSource
)

& $cscPath $cscArgs
if ($LASTEXITCODE -ne 0) {
    throw "Falló la compilación de setup.exe con código de salida $LASTEXITCODE."
}

# Limpiar carpeta temporal obj
Remove-Item $tempBuild -Recurse -Force

# Generar LEEME.txt informativo en dist/
$leemeContent = @"
============================================================
           ⏳ YaFaltaPoco - Archivos de Distribución
============================================================

¿Cómo utilizar la aplicación?

1. Instalador Oficial (Recomendado):
   Ejecuta "setup.exe". 
   - Diseñado con asistente visual en modo oscuro y logo animado.
   - Crea automáticamente el acceso directo en el Escritorio y en el Menú Inicio.
   - Integra desinstalador limpio en el Panel de Configuración de Windows.

2. Versión Portable (Sin instalación):
   Ingresa a la carpeta "portable" y ejecuta directamente "yafaltapoco.exe".
   - Puedes llevarla en una memoria USB o usarla sin permisos de administrador.
   - Requiere mantener "WebView2Loader.dll" en la misma carpeta.

¡Que disfrutes tu jornada! 🚀
============================================================
"@
Set-Content -Path "$distDir\LEEME.txt" -Value $leemeContent -Encoding UTF8

$setupSize = [math]::Round((Get-Item $outputSetup).Length / 1MB, 2)
Write-Host "[3/3] ¡Instalador generado con éxito!" -ForegroundColor Green
Write-Host "  -> Instalador: $outputSetup ($setupSize MB)" -ForegroundColor White
Write-Host "  -> Versión Portable: $portableDir\" -ForegroundColor White
Write-Host "  -> Guía de usuario: $distDir\LEEME.txt" -ForegroundColor White
Write-Host "==========================================" -ForegroundColor Cyan
