# ⏳✨ Ya Falta Poco

<div align="center">

> *"Porque la mejor parte de la jornada laboral... es saber exactamente cuánto falta para terminarla."* 😎☕

[![Rust 2021](https://img.shields.io/badge/Rust-2021-orange?style=for-the-badge&logo=rust)](https://www.rust-lang.org/)
[![Tauri v2](https://img.shields.io/badge/Tauri-v2.0-blue?style=for-the-badge&logo=tauri)](https://tauri.app/)
[![Windows 10 / 11](https://img.shields.io/badge/Windows-10%20%7C%2011-0078D6?style=for-the-badge&logo=windows)](https://microsoft.com/windows)
[![Dark Mode](https://img.shields.io/badge/Aesthetic-Dark%20Mode-18181b?style=for-the-badge&logo=apple)](https://github.com/ludwingx/yaFaltaPoco)
[![License MIT](https://img.shields.io/badge/License-MIT-34d399?style=for-the-badge)](LICENSE)

</div>

---

### 👋 ¡Hola! Te presentamos tu nuevo compañero de escritorio

¿Alguna vez has estado a mitad de la tarde sintiendo que las horas avanzan en cámara lenta? **Ya Falta Poco** es un widget de escritorio flotante, minimalista y en modo oscuro profundo que transforma tu jornada de trabajo o estudio en un viaje visual fluido, estético y muy motivador.

Inspirado en la elegancia visual de **Linear** y las interfaces de **Apple**, combina una línea temporal continua conectada de Sol a Luna, un rayo láser rojo que avanza minuto a minuto y un temporizador que te dice con precisión suiza cuánto falta para tu próxima pausa o para el fin de la jornada.

```text
  ┌──────────────────────────────────────────────────────────────┐
  │  ☀️ 08:00   ┌──────────────────────────┐                     │
  │     │       │ 💻 OtherBrain I          │ Turno de Mañana     │
  │     09:00   │    08:00 – 12:00 (4h)    │                     │
  │     10:00   └──────────────────────────┘                     │
  │  ───🔴 11:34 ───────────────────────────── ¡Láser en Vivo!   │
  │     │       ┌──────────────────────────┐                     │
  │     12:00   │ 🍽️ Almuerzo              │ ¡A recargar energía!│
  │     │       └──────────────────────────┘                     │
  │     13:00   ┌──────────────────────────┐                     │
  │     14:00   │ 💻 OtherBrain II         │ Turno de Tarde      │
  │     │       │    13:00 – 20:00 (7h)    │                     │
  │     ...     └──────────────────────────┘                     │
  │  🌙 20:00 ─── Fin de Jornada ───────────── Domingo Free 🌴   │
  └──────────────────────────────────────────────────────────────┘
```

---

### 🌟 ¿Qué lo hace tan peculiar y genial?

#### 🔴 1. El Láser Rojo en Tiempo Real
Nada de aburridas barras estáticas. Un haz láser rojo neón (`#ff453a`) corta visualmente la cápsula de tu actividad actual y avanza milimétricamente en tiempo real para que veas tu progreso de un vistazo.

#### 🎨 2. Tu Propio GIF & Fondo Camaleónico (¡Nuevo!)
¿Quieres a tu personaje de anime favorito, un gatito programador o el logo de tu empresa?
- **Haz clic directamente en el logo** para abrir el personalizador visual.
- Sube cualquier **GIF o imagen** local (o pega una URL directa).
- **Autodetección mágica de color:** La app analiza los píxeles perimetrales del GIF y adapta el color de fondo del widget de forma automática.
- **Difuminado de bordes (*Fade*):** Un sutil viñeteado radial fusiona tu GIF con el fondo negro para que se vea como si fuera parte nativa de la interfaz.

#### ⏱️ 3. Temporizador con *"Faltan:"*
Un contador regresivo minimalista que te mantiene al tanto de los segundos que restan para que suene la campana. Sin distracciones, sin números gigantes molestos.

#### 🌴 4. Domingo Free Oficial
Porque los domingos están hechos para descansar: la app detecta automáticamente el domingo y silencia las alertas de trabajo, mostrándote el mensaje: *"¡Domingo Free! 🎉"* con temática de relax.

#### 🔔 5. Alarma Sonora con Control Táctil
- **Clic izquierdo** en la campana (`🔔`): Prueba instantánea del sonido.
- **Clic derecho** en la campana: Silencia o reactiva las notificaciones sonoras.

#### 📐 6. Redimensionamiento Responsivo
Arrastra la esquina inferior derecha o los bordes. Todas las fuentes, cápsulas y espacios se recalculan con precisión matemática mediante CSS dinámico.

---

### 📦 ¿Cómo empezar en 1 minuto?

#### Opción A: Instalador de un Clic (`setup.exe` — ¡Recomendado!)
1. Descarga o abre [`setup.exe`](./setup.exe) (pesa solo **4.9 MB**).
2. Verás una ventana moderna en modo oscuro con el logo animado y el paso a paso interactivo.
3. Haz clic en **Instalar**. En 2 segundos creará tu acceso directo en el **Escritorio** y en el **Menú Inicio**.
4. ¡Listo! Ábrelo desde tu escritorio como cualquier programa normal.

#### Opción B: Modo Portable (Sin instalar)
Simplemente ejecuta [`yafaltapoco.exe`](./yafaltapoco.exe) manteniendo `WebView2Loader.dll` en la misma carpeta.

---

### 🎮 Atajos y Gestos

| Gesto / Clic | ¿Qué hace? |
|---|---|
| **Arrastrar desde arriba** | Mueve el widget libremente por cualquier parte de tu pantalla. |
| **Arrastrar esquina SE** | Cambia el tamaño del widget con escalado responsivo suave. |
| **Clic en el GIF / Logo** | 🎨 Abre el personalizador para elegir tu propio GIF y color de fondo. |
| **Clic en `Horario` / `Temporizador`** | Alterna entre agenda diaria por bloques o temporizador directo a una hora meta. |
| **Clic en `[ ➕ Crear turno ]`** | Abre el formulario modal para programar una nueva actividad. |
| **Clic sobre una tarjeta** | Edita el nombre, horario o color de ese turno. |
| **Clic en `🔔` (izquierdo)** | Prueba la campana sonora. |
| **Clic en `🔔` (derecho)** | Silencia o activa el sonido. |
| **Clic en `✕`** | Cierra la aplicación limpiamente. |

---

### 🗂️ Estructura del Proyecto

Organizado bajo principios de **Clean Code** y separación de responsabilidades:

```text
yaFaltaPoco/
├── 📦 setup.exe                # Instalador nativo oficial (Modo oscuro + GIF + Accesos)
├── 🚀 yafaltapoco.exe          # Ejecutable release portable
├── ⚙️ WebView2Loader.dll       # Runtime loader para Windows (GNU toolchain)
│
├── 🛠️ tools/installer/         # Código fuente del instalador nativo en C# WinForms
│   ├── Program.cs             # Lógica de instalación, pasos animados y accesos directos
│   └── build-installer.ps1    # Script automatizado de empaquetado y compresión GZip
│
├── 🦀 src-tauri/               # Backend en Rust (Tauri v2)
│   ├── src/main.rs            # Ventana flotante sin bordes, drag IPC y comandos nativos
│   ├── src/schedule.rs        # Lógica de bloques horarios, colores y duración
│   ├── src/timer.rs           # Temporizador regresivo y hora meta
│   ├── src/config.rs          # Guardado automático en yafaltapoco.json
│   └── Cargo.toml             # Dependencias de Rust
│
├── 🎨 ui/                      # Frontend Web ultra ligero (HTML5 + CSS3 + Vanilla JS)
│   ├── index.html             # Interfaz, maquetación del riel, láser e iconos Lucide
│   ├── bg-logo.gif            # Logotipo animado por defecto
│   └── alarm.mp3              # Campana de fin de turno
│
├── 📦 legacy/                  # Prototipo inicial en egui/eframe (archivado históricamente)
└── 📖 README.md                # Esta guía
```

---

### 💻 Para Desarrolladores y Curiosos

#### 1. Previsualización instantánea en el navegador (sin compilar nada):
Abre directamente [`ui/index.html`](./ui/index.html) con tu navegador favorito (Chrome, Edge, Firefox). El archivo incluye un simulador (*mock*) automático de Tauri para que puedas ajustar estilos y probar animaciones al instante.

#### 2. Compilar el binario release de Rust:
```powershell
cd src-tauri
cargo build --release
```

#### 3. Recompilar el instalador `setup.exe`:
```powershell
powershell -ExecutionPolicy Bypass -File tools\installer\build-installer.ps1
```

---

### 💡 Preguntas Frecuentes

<details>
<summary><b>¿Dónde se guardan mis turnos y configuraciones?</b></summary>
Se guardan de forma local en <code>yafaltapoco.json</code> en el directorio de la aplicación, y tus preferencias de tema personalizado en el almacenamiento local. Nada sale de tu máquina: privacidad total y 100% offline.
</details>

<details>
<summary><b>¿Consume muchos recursos en segundo plano?</b></summary>
Casi cero. Gracias a Tauri v2 y Rust, el consumo de memoria RAM es mínimo (menos de 25-35 MB) y el uso de CPU es prácticamente 0% mientras esperas que avance el tiempo.
</details>

<details>
<summary><b>¿Puedo usar cualquier imagen o GIF animado?</b></summary>
¡Sí! Puedes usar archivos <code>.gif</code>, <code>.png</code>, <code>.jpg</code> o <code>.webp</code> tanto locales desde tu disco duro como pegando un enlace web.
</details>

---

<div align="center">

Hecho con ☕, Rust y mucho cariño por **ludwingx**.  
*¡Que la jornada se te haga cortita!* ⏳🚀

</div>
