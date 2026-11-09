# Script automatizado para compilar el instalador nativo setup.exe de YaFaltaPoco
$ErrorActionPreference = "Stop"

$workspaceRoot = Resolve-Path "$PSScriptRoot\..\.."
Set-Location $workspaceRoot

Write-Host "==========================================" -ForegroundColor Cyan
Write-Host "  Compilando setup.exe para YaFaltaPoco   " -ForegroundColor Cyan
Write-Host "==========================================" -ForegroundColor Cyan

$exeSource = "$workspaceRoot\yafaltapoco.exe"
$dllSource = "$workspaceRoot\WebView2Loader.dll"
$gifSource = "$workspaceRoot\ui\bg-logo.gif"
$icoSource = "$workspaceRoot\src-tauri\icons\icon.ico"
$outputSetup = "$workspaceRoot\setup.exe"

# Validar existencia de archivos fuente
if (-not (Test-Path $exeSource)) {
    throw "No se encontró $exeSource. Asegúrate de haber compilado la aplicación primero."
}
if (-not (Test-Path $dllSource)) {
    if (Test-Path "$workspaceRoot\src-tauri\target\release\WebView2Loader.dll") {
        Copy-Item "$workspaceRoot\src-tauri\target\release\WebView2Loader.dll" -Destination $dllSource -Force
    } else {
        throw "No se encontró $dllSource."
    }
}
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

Compress-FileGzip $exeSource $appGz
Compress-FileGzip $dllSource $loaderGz

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

$setupSize = [math]::Round((Get-Item $outputSetup).Length / 1MB, 2)
Write-Host "[3/3] ¡Instalador generado con éxito!" -ForegroundColor Green
Write-Host "  -> Ubicación: $outputSetup" -ForegroundColor White
Write-Host "  -> Tamaño final: $setupSize MB" -ForegroundColor White
Write-Host "==========================================" -ForegroundColor Cyan
