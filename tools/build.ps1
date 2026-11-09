# Script de compilacion y empaquetado integral para YaFaltaPoco
param(
    [switch]$SkipRustBuild = $false
)

$ErrorActionPreference = "Stop"
$workspaceRoot = Resolve-Path "$PSScriptRoot\.."
Set-Location $workspaceRoot

Write-Host "============================================================" -ForegroundColor Magenta
Write-Host "      YaFaltaPoco - Proceso de Build y Distribucion         " -ForegroundColor Magenta
Write-Host "============================================================" -ForegroundColor Magenta

# 1. Compilar backend de Rust en release (si no se omite)
if (-not $SkipRustBuild) {
    Write-Host "`n[Paso 1/2] Compilando backend en Rust (Release)..." -ForegroundColor Cyan
    & cargo build --release --manifest-path "$workspaceRoot\src-tauri\Cargo.toml"
    if ($LASTEXITCODE -ne 0) {
        throw "Fallo la compilacion de Rust."
    }
} else {
    Write-Host "`n[Paso 1/2] Omitiendo compilacion de Rust (-SkipRustBuild activo)." -ForegroundColor DarkGray
}

# 2. Generar carpeta dist/ con portable/ y setup.exe
Write-Host "`n[Paso 2/2] Generando artefactos en dist/ y empaquetando setup.exe..." -ForegroundColor Cyan
& powershell -ExecutionPolicy Bypass -File "$workspaceRoot\tools\installer\build-installer.ps1"

Write-Host "`nProceso completado con exito. Revisa la carpeta 'dist/'." -ForegroundColor Green
