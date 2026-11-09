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
Nada de aburridas barras estáticas. Un haz láser brillante corta visualmente la cápsula de tu actividad actual y avanza milimétricamente en tiempo real con efecto neón para que veas tu progreso de un solo vistazo.

#### 🎨 2. Temas Visuales y Modo Camaleónico (¡Nuevo!)
Elige la estética que mejor combine con tu setup o tu estado de ánimo:
- **6 Temas Integrados:**
  - 🌌 **Midnight:** El clásico y elegante negro profundo con láser rojo.
  - ⚡ **Cyberpunk Neon:** Fondo azul obsidiana con láser cian electro y bordes glow.
  - 🍏 **Apple Grafito:** Paleta inspirada en macOS con láser verde esmeralda suave.
  - 🌲 **Esmeralda Forest:** Tonalidades orgánicas oscuras con láser verde menta.
  - 🌅 **Atardecer Solar:** Tonos cálidos café/rojizos con láser naranja crepuscular.
  - 🔮 **Nébula Púrpura:** Mística espacial con láser violeta fluorescente.
- **Selector libre de colores:** Personaliza el fondo y el color del láser como quieras.
- **Autodetección mágica de color:** La app analiza tu GIF o imagen y calibra automáticamente el color de fondo para una integración perfecta.
- **Difuminado de bordes (*Fade*):** Viñeteado radial graduable (0% - 100%) para difuminar cualquier imagen o GIF con el fondo.

#### 🌤️ 3. Clima en Vivo con Open-Meteo (¡Nuevo!)
¿Cómo está el día afuera de la oficina?
- **Píldora climática en la cabecera:** Temperatura actual e icono meteorológico dinámico (☀️, 🌤️, ⛅, 🌧️, ⛈️, ❄️, 🌙).
- **Sin necesidad de API Key:** Integrado con el servicio libre mundial de **Open-Meteo**.
- **Panel con datos meteorológicos:** Sensación térmica (*feels-like*), humedad relativa, velocidad del viento y precipitaciones.
- **Ubicación flexible:** Selector de ciudades populares (Santa Cruz de la Sierra, La Paz, Cochabamba, Buenos Aires, Santiago, Lima, Bogotá, CDMX, Madrid, Miami...), botón de detección automática por **GPS** o coordenadas manuales.
- **Unidades:** Alterna entre Celsius (°C) y Fahrenheit (°F) con un solo toque.

#### ⏱️ 4. Temporizador con *"Faltan:"* y Modo 12h/24h
Un contador regresivo minimalista que te mantiene al tanto de los segundos que restan para que suene la campana. Puedes alternar entre formato militar de 24h o formato de 12 horas con indicador AM/PM.

#### ⚙️ 5. Centro de Ajustes Integral
Un nuevo botón de configuración (`⚙️`) en la barra superior te da acceso directo a 4 pestañas:
1. **🎨 Temas:** Presets de colores y personalización del haz láser.
2. **🖼️ Logo / GIF:** Selector de imagen o GIF local/URL y viñeteado.
3. **🌤️ Clima:** Activar/desactivar, configuración de ubicación y pronóstico.
4. **⚙️ Ajustes:** Volumen de la alarma con botón de prueba, formato de reloj y texto personalizable para el temporizador.

#### 🌴 6. Domingo Free Oficial
Porque los domingos están hechos para descansar: la app detecta automáticamente el domingo y silencia las alertas de trabajo, mostrándote el mensaje: *"¡Domingo Free! 🎉"* con temática de relax.

#### 🔔 7. Alarma Sonora con Control Táctil y Volumen
- **Clic izquierdo** en la campana (`🔔`): Prueba instantánea del sonido.
- **Clic derecho** en la campana: Silencia o reactiva las notificaciones sonoras.
- **Slider de volumen en ajustes:** Gradúa el volumen desde 0% hasta 100%.

#### 📐 8. Redimensionamiento Responsivo
Arrastra la esquina inferior derecha o los bordes. Todas las fuentes, cápsulas y espacios se recalculan con precisión matemática mediante CSS dinámico.

---

### 📦 ¿Cómo empezar en 1 minuto?

Todos los archivos listos para el usuario final se encuentran organizados dentro de la carpeta [`dist/`](./dist/):

#### Opción A: Instalador de un Clic (`dist/setup.exe` — ¡Recomendado!)
1. Descarga o abre [`dist/setup.exe`](./dist/setup.exe) (pesa solo **4.9 MB**).
2. Verás una ventana moderna en modo oscuro con el logo animado y el paso a paso interactivo.
3. Haz clic en **Instalar**. En 2 segundos creará tu acceso directo en el **Escritorio** y en el **Menú Inicio**.
4. ¡Listo! Ábrelo desde tu escritorio como cualquier programa normal.

#### Opción B: Modo Portable (Sin instalar)
Ingresa a la carpeta [`dist/portable/`](./dist/portable/) y ejecuta directamente `yafaltapoco.exe` (mantiene `WebView2Loader.dll` a su lado para máxima portabilidad en memorias USB o equipos sin privilegios).

---

### 🎮 Atajos y Gestos

| Gesto / Clic | ¿Qué hace? |
|---|---|
| **Arrastrar desde arriba** | Mueve el widget libremente por cualquier parte de tu pantalla. |
| **Arrastrar esquina SE** | Cambia el tamaño del widget con escalado responsivo suave. |
| **Clic en `⚙️` (Configuración)** | Abre el panel integral con pestañas de Temas, GIF, Clima y Ajustes. |
| **Clic en la píldora de Clima (`🌤️ 27°C`)** | Abre directamente los detalles y ajustes del clima. |
| **Clic en el GIF / Logo** | Abre directamente el personalizador de GIF y fondo. |
| **Clic en `Horario` / `Temporizador`** | Alterna entre agenda diaria por bloques o temporizador directo a una hora meta. |
| **Clic en `[ ➕ Crear turno ]`** | Abre el formulario modal para programar una nueva actividad. |
| **Clic sobre una tarjeta** | Edita el nombre, horario o color de ese turno. |
| **Clic en `🔔` (izquierdo)** | Prueba la campana sonora. |
| **Clic en `🔔` (derecho)** | Silencia o activa el sonido. |
| **Clic en `✕`** | Cierra la aplicación limpiamente. |

---

### 🗂️ Estructura del Proyecto

Organizado bajo principios de **Clean Code** y estricta separación entre código fuente y distribución:

```text
yaFaltaPoco/
├── 📦 dist/                     # Carpeta de Distribución Pública (Releases)
│   ├── setup.exe               # Instalador nativo oficial todo-en-uno (4.9 MB)
│   ├── LEEME.txt               # Guía rápida para el usuario final
│   └── portable/               # Versión portable lista para usar sin instalación
│       ├── yafaltapoco.exe
│       └── WebView2Loader.dll
│
├── 🦀 src-tauri/               # Backend en Rust (Tauri v2)
│   ├── src/main.rs            # Ventana flotante sin bordes, drag IPC y comandos nativos
│   ├── src/schedule.rs        # Lógica de bloques horarios, colores y duración
│   ├── src/timer.rs           # Temporizador regresivo y hora meta
│   ├── src/config.rs          # Persistencia en yafaltapoco.json
│   └── Cargo.toml             # Dependencias de Rust
│
├── 🎨 ui/                      # Frontend Web ultra ligero (HTML5 + CSS3 + Vanilla JS)
│   ├── index.html             # Interfaz, riel de tiempo, 6 temas, clima Open-Meteo
│   ├── bg-logo.gif            # Logotipo animado por defecto
│   └── alarm.mp3              # Campana de fin de turno
│
├── 🛠️ tools/                   # Herramientas de build y automatización
│   ├── build.ps1              # Script maestro de compilación integral (Rust + Dist)
│   └── installer/             # Código fuente del instalador nativo en C# WinForms
│       ├── Program.cs         # Lógica visual del instalador y descompresión GZip
│       └── build-installer.ps1 # Empaquetador a dist/setup.exe
│
├── 📦 legacy/                  # Prototipo inicial en egui/eframe (archivado históricamente)
└── 📖 README.md                # Esta guía
```

---

### 💻 Para Desarrolladores y Curiosos

#### 1. Compilación Completa en Un Solo Comando:
```powershell
powershell -ExecutionPolicy Bypass -File tools/build.ps1
```
Este script compila el binario release de Rust con optimizaciones máximas, extrae las librerías a `dist/portable/` y genera el instalador oficial `dist/setup.exe`.

#### 2. Previsualización instantánea en el navegador (sin compilar nada):
Abre directamente [`ui/index.html`](./ui/index.html) con cualquier navegador (Chrome, Edge, Firefox). Incluye un simulador (*mock*) automático de Tauri para ajustar estilos y animaciones al instante.

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
