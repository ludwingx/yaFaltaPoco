# ⏳ Ya Falta Poco

> **Widget flotante de escritorio para productividad, agenda visual por turnos y temporizador regresivo minimalista en modo oscuro.**

![Rust](https://img.shields.io/badge/Rust-2021-orange?logo=rust)
![Tauri](https://img.shields.io/badge/Tauri-v2.0-blue?logo=tauri)
![Platform](https://img.shields.io/badge/Platform-Windows%2010%20%7C%2011-0078D6?logo=windows)
![Icons](https://img.shields.io/badge/Icons-Lucide%20React-F56565?logo=react)
![License](https://img.shields.io/badge/License-MIT-green)

---

## 📖 Descripción

**Ya Falta Poco** es una aplicación de escritorio diseñada para mantenerte enfocado durante tu jornada de trabajo o estudio. Presenta un diseño moderno inspirado en interfaces de alta gama (*Dark Mode* de Apple y Linear), combinando un riel temporal continuo con cápsulas de colores, un marcador láser rojo en tiempo real y una cuenta regresiva que te recuerda cuánto falta para culminar tu actividad actual o tu jornada laboral.

---

## ✨ Características Principales

- **🧭 Línea Temporal Interactiva:**
  - Riel vertical continuo que conecta desde el Sol (`Sun`) hasta la Luna (`Moon`) con alineación matemática precisa.
  - Cápsulas verticales para cada actividad con iconos vectoriales **Lucide** (`Laptop`, `Utensils`, `Coffee`, `Dumbbell`, `ShowerHead`, `Mail`, `Video`, etc.).
  - Colores distintivos: azul eléctrico, ámbar/naranja, púrpura y coral según el tipo de bloque.
  - Casillas de verificación (`Check`) con estilo iOS para marcar turnos concluidos.

- **🔴 Rayo Láser en Tiempo Real:**
  - Una línea roja horizontal (`#ff453a`) con la hora actual que corta visualmente la cápsula y la tarjeta del turno activo.

- **⏱️ Temporizador Regresivo con "Faltan:":**
  - Muestra el tiempo restante exacto (`HH:MM:SS`) y el nombre de la tarea en curso.
  - Cuenta con la etiqueta **"Faltan:"** en la parte superior izquierda.

- **🔄 Dos Modos de Operación:**
  - **Modo Horario:** Evalúa tus bloques configurados de lunes a sábado (ej. trabajo, almuerzo, estudio).
  - **Modo Temporizador:** Permite fijar una hora meta directa (ej. `20:00`) con un solo clic.

- **🔔 Notificación Sonora Personalizada:**
  - Emite una alerta auditiva al concluir cada turno para que no pierdas la noción del tiempo.
  - Botón interactivo para probar la campana con clic izquierdo o silenciarla con clic derecho.

- **🌴 Domingo Free:**
  - Los domingos se desactivan automáticamente las alertas de trabajo, mostrando el estado de descanso (*"¡Domingo Free! 🎉"*).

- **📐 Redimensionamiento y Escalado Responsivo:**
  - Factor de escala CSS dinámico (`--s`) que ajusta tipografías, tarjetas y espacios al arrastrar los bordes o la esquina inferior derecha.

- **🖼️ Logo Animado con Zoom:**
  - Integración sutil y prominente del logo animado (GIF) en fondo negro puro sin bordes circulares molestos.

- **⚡ 100% Offline:**
  - Todos los iconos SVG de Lucide, estilos, tipografías y audios están embebidos dentro de la aplicación. No requiere conexión a Internet.

---

## 🚀 Descarga e Instalación

Puedes usar **Ya Falta Poco** de dos maneras:

1. **Instalador Oficial (`setup.exe` - Recomendado):**
   - Ejecuta directamente [`setup.exe`](./setup.exe) en la raíz del proyecto.
   - Presenta un diseño moderno en **modo oscuro** (`#0e0e11`) con el logotipo animado a la izquierda, paso a paso interactivo y barra de progreso.
   - Instala la aplicación en `%LOCALAPPDATA%\Programs\YaFaltaPoco`, copia todas las librerías necesarias (`WebView2Loader.dll`) y crea automáticamente el **acceso directo en el Escritorio** y en el **Menú Inicio**.
   - Incluye desinstalador integrado accesible desde el menú de inicio y desde la configuración de aplicaciones de Windows.

2. **Portable (Uso directo sin instalar):**
   - Ejecuta [`yafaltapoco.exe`](./yafaltapoco.exe) manteniendo `WebView2Loader.dll` en la misma carpeta.

---

## ⌨️ Controles y Atajos

| Acción | Control |
|---|---|
| **Mover la ventana** | Haz clic y arrastra desde la barra superior o cualquier espacio libre. |
| **Cambiar tamaño** | Arrastra la manija de la esquina inferior derecha o los bordes laterales. |
| **Alternar Modo** | Haz clic en la píldora superior (`Horario` / `Temporizador`). |
| **Ajustar Hora Meta** | En Modo Temporizador, haz clic en el texto inferior del contador. |
| **Probar Alarma** | Clic izquierdo en el botón de la campana (`🔔`). |
| **Silenciar Alarma** | Clic derecho en la campana (`🔔` / `🔇`). |
| **Crear Turno** | Clic en el botón `[ ➕ Crear turno ]`. |
| **Editar Turno** | Clic sobre la tarjeta del turno correspondiente. |
| **Cerrar** | Botón `✕` en la esquina superior derecha. |

---

## 📁 Estructura del Proyecto

El proyecto sigue una arquitectura limpia (*Clean Architecture*) separando claramente el backend nativo en Rust, la interfaz visual web, el instalador nativo y los archivos históricos:

```text
yaFaltaPoco/
├── setup.exe                   # 📦 Instalador oficial moderno (Modo oscuro + GIF + Acceso directo)
├── yafaltapoco.exe             # 🚀 Ejecutable release portable
├── WebView2Loader.dll          # ⚙️ Loader necesario para ejecución portable en Windows (GNU toolchain)
│
├── tools/                      # 🛠️ Herramientas y utilidades del proyecto
│   └── installer/              # Código fuente y script de compilación de setup.exe (C# WinForms)
│       ├── Program.cs          # UI oscura, reproductor de GIF, pasos de instalación y accesos directos
│       └── build-installer.ps1 # Automatización de empaquetado y compresión
│
├── src-tauri/                  # 🦀 Backend nativo (Rust + Tauri v2)
│   ├── src/
│   │   ├── main.rs             # Punto de entrada, ventana flotante y comandos IPC
│   │   ├── schedule.rs         # Lógica de bloques horarios, colores y turnos
│   │   ├── timer.rs            # Temporizador de cuenta regresiva hacia hora meta
│   │   ├── config.rs           # Gestión de persistencia y archivo yafaltapoco.json
│   │   └── alarm.rs            # Estado y control de sonido
│   ├── icons/                  # Icono de la aplicación (.ico)
│   ├── Cargo.toml              # Dependencias de Rust (Tauri 2, Serde, Chrono)
│   └── tauri.conf.json         # Configuración de ventana, permisos y empaquetado
│
├── ui/                         # 🎨 Frontend Web (HTML5, CSS3, Vanilla JS)
│   ├── index.html              # Interfaz completa, maquetación responsiva e iconos Lucide SVG
│   ├── bg-logo.gif             # Logotipo animado del workspace
│   └── alarm.mp3               # Audio de campana para fin de turno
│
├── legacy/                     # 📦 Archivo histórico
│   └── egui-prototype/         # Prototipo inicial en egui/eframe (archivado)
│
├── .gitignore                  # Exclusiones de Git (builds, temporales y configs)
└── README.md                   # Esta documentación
```

---

## 💻 Desarrollo y Compilación

### Requisitos previos:
- [Rust](https://rustup.rs/) (versión 1.75 o superior).
- Windows 10/11 con soporte para WebView2 (incluido de forma nativa en el sistema).
- .NET Framework (incluido de forma nativa en Windows 10/11 para compilar el instalador con `csc.exe`).

### Compilar el instalador `setup.exe`:
```powershell
powershell -ExecutionPolicy Bypass -File tools\installer\build-installer.ps1
```

### Previsualización instantánea en navegador (sin compilar):
Puedes abrir directamente [`ui/index.html`](./ui/index.html) en Chrome, Edge o Firefox. La aplicación incluye un sistema de *mock* automático que simula los comandos de Tauri, permitiendo ajustar estilos y diseño de forma inmediata.

### Compilar el binario Release de Rust:
```powershell
# Desde el directorio src-tauri:
cd src-tauri
cargo build --release
```
El ejecutable resultante se ubicará en `src-tauri/target/release/yafaltapoco.exe`.

### Generar instaladores oficiales (MSI y NSIS):
```powershell
npx @tauri-apps/cli build
```
Los instaladores se generarán en la carpeta `src-tauri/target/release/bundle/`.

---

## ⚙️ Persistencia de Datos

La configuración del usuario (turnos personalizados, colores, hora meta y estado del sonido) se guarda automáticamente en formato JSON en el archivo local `yafaltapoco.json`:

```json
{
  "mode": "Schedule",
  "blocks": [
    {
      "id": "block-1",
      "title": "OtherBrain I",
      "start_hour": 8,
      "start_min": 0,
      "end_hour": 12,
      "end_min": 0,
      "color": { "r": 10, "g": 132, "b": 255 }
    }
  ],
  "timer": {
    "target_hour": 20,
    "target_min": 0
  },
  "sound_enabled": true
}
```

---

## 📄 Licencia

Este proyecto está bajo la Licencia **MIT**. Consulta el código fuente para más detalles.
