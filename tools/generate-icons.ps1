param(
    [string]$SourceImage = "$PSScriptRoot\app-icon-source.png"
)

$ErrorActionPreference = "Stop"
$workspaceRoot = Resolve-Path "$PSScriptRoot\.."
Set-Location $workspaceRoot

if (-not (Test-Path $SourceImage)) {
    throw "No se encontró la imagen de origen en: $SourceImage"
}

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "   Generador de Iconos de Alta Resolución para YaFaltaPoco  " -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

# Definición de C# para generar DIBs puros (100% compatibles con Win32 y WinForms) y PNG en ICO
$csharpSource = @"
using System;
using System.IO;
using System.Drawing;
using System.Drawing.Imaging;
using System.Drawing.Drawing2D;

public class IconGenerator {
    public static byte[] CreateDIB(Bitmap bmp) {
        int width = bmp.Width;
        int height = bmp.Height;
        int andStride = ((width + 31) / 32) * 4;
        int andSize = andStride * height;
        int pixelDataSize = width * height * 4;
        int totalSize = 40 + pixelDataSize + andSize;

        byte[] data = new byte[totalSize];
        using (MemoryStream ms = new MemoryStream(data))
        using (BinaryWriter bw = new BinaryWriter(ms)) {
            // BITMAPINFOHEADER (40 bytes)
            bw.Write((uint)40);
            bw.Write((int)width);
            bw.Write((int)(height * 2)); // XOR + AND height
            bw.Write((ushort)1);          // Planes
            bw.Write((ushort)32);         // BitCount
            bw.Write((uint)0);            // Compression (BI_RGB)
            bw.Write((uint)(pixelDataSize + andSize));
            bw.Write((int)0);             // XPelsPerMeter
            bw.Write((int)0);             // YPelsPerMeter
            bw.Write((uint)0);            // ClrUsed
            bw.Write((uint)0);            // ClrImportant

            // BGRA 32bpp data from bottom to top scanlines
            for (int y = height - 1; y >= 0; y--) {
                for (int x = 0; x < width; x++) {
                    Color c = bmp.GetPixel(x, y);
                    bw.Write(c.B);
                    bw.Write(c.G);
                    bw.Write(c.R);
                    bw.Write(c.A);
                }
            }

            // AND mask (1bpp monochrome mask, all 0s for full alpha usage)
            byte[] andMask = new byte[andSize];
            bw.Write(andMask);
        }
        return data;
    }

    public static byte[] CreatePng(Bitmap bmp) {
        using (MemoryStream ms = new MemoryStream()) {
            bmp.Save(ms, ImageFormat.Png);
            return ms.ToArray();
        }
    }

    public static Bitmap RenderSquare(Bitmap src, int canvasSize, float paddingRatio = 0.03f) {
        Bitmap target = new Bitmap(canvasSize, canvasSize, PixelFormat.Format32bppArgb);
        using (Graphics g = Graphics.FromImage(target)) {
            g.InterpolationMode = InterpolationMode.HighQualityBicubic;
            g.SmoothingMode = SmoothingMode.HighQuality;
            g.PixelOffsetMode = PixelOffsetMode.HighQuality;
            g.CompositingQuality = CompositingQuality.HighQuality;
            g.Clear(Color.Transparent);

            int padding = (int)Math.Round(canvasSize * paddingRatio);
            int drawSize = canvasSize - (padding * 2);
            if (drawSize < 1) drawSize = canvasSize;
            int drawX = (canvasSize - drawSize) / 2;
            int drawY = (canvasSize - drawSize) / 2;

            g.DrawImage(src, new Rectangle(drawX, drawY, drawSize, drawSize));
        }
        return target;
    }

    public static byte[] GenerateMultiResIco(Bitmap croppedSrc) {
        // Resoluciones completas de Windows: 16, 24, 32, 48, 64, 128, 256
        int[] sizes = new int[] { 16, 24, 32, 48, 64, 128, 256 };
        byte[][] buffers = new byte[sizes.Length][];

        for (int i = 0; i < sizes.Length; i++) {
            int sz = sizes[i];
            // Para tamaños chicos (16, 24) usamos padding 0 para máxima legibilidad; para mayores un sutil 2.5%
            float pad = sz <= 24 ? 0f : 0.025f;
            using (Bitmap frame = RenderSquare(croppedSrc, sz, pad)) {
                if (sz <= 128) {
                    buffers[i] = CreateDIB(frame);
                } else {
                    buffers[i] = CreatePng(frame);
                }
            }
        }

        using (MemoryStream ms = new MemoryStream())
        using (BinaryWriter bw = new BinaryWriter(ms)) {
            // ICONDIR
            bw.Write((ushort)0); // Reserved
            bw.Write((ushort)1); // Type: 1 = Icon
            bw.Write((ushort)sizes.Length);

            int offset = 6 + (sizes.Length * 16);
            for (int i = 0; i < sizes.Length; i++) {
                int sz = sizes[i];
                bw.Write((byte)(sz == 256 ? 0 : sz)); // Width
                bw.Write((byte)(sz == 256 ? 0 : sz)); // Height
                bw.Write((byte)0);                    // ColorCount
                bw.Write((byte)0);                    // Reserved
                bw.Write((ushort)1);                  // Planes
                bw.Write((ushort)32);                 // BitCount
                bw.Write((uint)buffers[i].Length);    // BytesInRes
                bw.Write((uint)offset);               // ImageOffset
                offset += buffers[i].Length;
            }

            for (int i = 0; i < sizes.Length; i++) {
                bw.Write(buffers[i]);
            }
            return ms.ToArray();
        }
    }
}
"@

Add-Type -TypeDefinition $csharpSource -ReferencedAssemblies System.Drawing

Write-Host "Cargando imagen base..." -ForegroundColor Yellow
$srcBitmap = [System.Drawing.Bitmap]::FromFile($SourceImage)

# Recorte exacto del círculo del icono (eliminando márgenes asimétricos)
# El círculo visual mide exactamente 396x396 desde (4, 4) hasta (399, 399)
$cropRect = New-Object System.Drawing.Rectangle(4, 4, 396, 396)
$croppedCircle = $srcBitmap.Clone($cropRect, $srcBitmap.PixelFormat)

# Directorios de destino
$tauriIconsDir = "$workspaceRoot\src-tauri\icons"
$uiDir = "$workspaceRoot\ui"
$portableDir = "$workspaceRoot\dist\portable"

if (-not (Test-Path $tauriIconsDir)) { New-Item -ItemType Directory -Path $tauriIconsDir | Out-Null }
if (-not (Test-Path $uiDir)) { New-Item -ItemType Directory -Path $uiDir | Out-Null }
if (-not (Test-Path $portableDir)) { New-Item -ItemType Directory -Path $portableDir | Out-Null }

Write-Host "Generando icon.ico multi-resolución (16x16 hasta 256x256)..." -ForegroundColor Yellow
$icoBytes = [IconGenerator]::GenerateMultiResIco($croppedCircle)

# Guardar icon.ico en src-tauri/icons, ui, y dist/portable
[System.IO.File]::WriteAllBytes("$tauriIconsDir\icon.ico", $icoBytes)
[System.IO.File]::WriteAllBytes("$uiDir\favicon.ico", $icoBytes)
[System.IO.File]::WriteAllBytes("$portableDir\icon.ico", $icoBytes)

Write-Host "  -> Creado src-tauri\icons\icon.ico ($([math]::Round($icoBytes.Length/1024, 1)) KB)" -ForegroundColor Green
Write-Host "  -> Creado ui\favicon.ico" -ForegroundColor Green
Write-Host "  -> Creado dist\portable\icon.ico" -ForegroundColor Green

# Generación de PNGs para Tauri y Windows App Store
$pngSpecs = @{
    "icon.png"             = 512
    "128x128@2x.png"       = 256
    "128x128.png"          = 128
    "64x64.png"            = 64
    "32x32.png"            = 32
    "Square310x310Logo.png" = 310
    "Square284x284Logo.png" = 284
    "Square150x150Logo.png" = 150
    "Square142x142Logo.png" = 142
    "Square107x107Logo.png" = 107
    "Square89x89Logo.png"  = 89
    "Square71x71Logo.png"  = 71
    "Square44x44Logo.png"  = 44
    "Square30x30Logo.png"  = 30
    "StoreLogo.png"        = 50
}

Write-Host "`nGenerando variantes PNG optimizadas..." -ForegroundColor Yellow
foreach ($entry in $pngSpecs.GetEnumerator()) {
    $filename = $entry.Key
    $size = $entry.Value
    $pad = if ($size -le 32) { [float]0.0 } else { [float]0.025 }
    $bmp = [IconGenerator]::RenderSquare($croppedCircle, $size, $pad)
    $destPath = "$tauriIconsDir\$filename"
    $bmp.Save($destPath, [System.Drawing.Imaging.ImageFormat]::Png)
    $bmp.Dispose()
    Write-Host "  -> $filename (${size}x${size})" -ForegroundColor DarkGray
}

# Copiar icon.png a ui/ para el frontend
Copy-Item "$tauriIconsDir\icon.png" -Destination "$uiDir\icon.png" -Force
Write-Host "  -> ui\icon.png (512x512)" -ForegroundColor Green

# Liberar recursos
$croppedCircle.Dispose()
$srcBitmap.Dispose()

Write-Host "`nTodos los iconos generados exitosamente." -ForegroundColor Green
