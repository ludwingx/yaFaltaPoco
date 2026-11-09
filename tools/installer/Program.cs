using System;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using System.Threading;
using System.Windows.Forms;
using Microsoft.Win32;

namespace YaFaltaPoco.Installer
{
    static class Program
    {
        [STAThread]
        static void Main(string[] args)
        {
            if (args != null && args.Length > 0 && (args[0].Equals("/silent", StringComparison.OrdinalIgnoreCase) || args[0].Equals("/s", StringComparison.OrdinalIgnoreCase)))
            {
                InstallerForm form = new InstallerForm();
                form.RunSilentInstallation();
                return;
            }

            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            Application.Run(new InstallerForm());
        }
    }

    public class ModernProgressBar : Control
    {
        private int _value = 0;
        public int Value
        {
            get { return _value; }
            set
            {
                _value = Math.Max(0, Math.Min(100, value));
                Invalidate();
            }
        }

        public ModernProgressBar()
        {
            SetStyle(ControlStyles.UserPaint | ControlStyles.AllPaintingInWmPaint | ControlStyles.OptimizedDoubleBuffer, true);
            Height = 10;
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            Graphics g = e.Graphics;
            g.SmoothingMode = SmoothingMode.AntiAlias;

            // Fondo del riel
            using (SolidBrush bgBrush = new SolidBrush(Color.FromArgb(28, 28, 34)))
            {
                g.FillRectangle(bgBrush, 0, 0, Width, Height);
            }

            // Borde sutil
            using (Pen borderPen = new Pen(Color.FromArgb(42, 42, 50)))
            {
                g.DrawRectangle(borderPen, 0, 0, Width - 1, Height - 1);
            }

            // Barra de progreso (Azul eléctrico vibrante)
            if (_value > 0)
            {
                int fillWidth = (int)((Width - 2) * (_value / 100.0));
                if (fillWidth > 0)
                {
                    using (LinearGradientBrush fillBrush = new LinearGradientBrush(
                        new Rectangle(1, 1, fillWidth, Height - 2),
                        Color.FromArgb(10, 132, 255),
                        Color.FromArgb(56, 189, 248),
                        LinearGradientMode.Horizontal))
                    {
                        g.FillRectangle(fillBrush, 1, 1, fillWidth, Height - 2);
                    }
                }
            }
        }
    }

    public class InstallerForm : Form
    {
        private PictureBox picLogo;
        private Label lblTitle;
        private Label lblSubtitle;
        private Label[] lblStepIcons;
        private Label[] lblStepTexts;
        private ModernProgressBar progressBar;
        private Label lblStatus;
        private Label lblPercent;
        private CheckBox chkDesktop;
        private CheckBox chkLaunch;
        private Label lblDestPath;
        private Button btnCancel;
        private Button btnInstall;

        private bool isFinished = false;
        private string installDir;
        private string exePath;

        private readonly string[] stepTitles = new string[]
        {
            "Preparar directorio de instalación",
            "Desempaquetar componentes de YaFaltaPoco",
            "Instalar binario y WebView2 Loader",
            "Crear acceso directo en el Escritorio",
            "Registrar en el Menú Inicio de Windows"
        };

        public InstallerForm()
        {
            InitializeComponent();
            LoadEmbeddedAssets();
        }

        private void InitializeComponent()
        {
            Text = "Ya Falta Poco — Instalador";
            ClientSize = new Size(680, 440);
            StartPosition = FormStartPosition.CenterScreen;
            FormBorderStyle = FormBorderStyle.FixedDialog;
            MaximizeBox = false;
            BackColor = Color.FromArgb(14, 14, 17);
            ForeColor = Color.White;
            Font = new Font("Segoe UI", 9.5f, FontStyle.Regular);

            installDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Programs", "YaFaltaPoco");
            exePath = Path.Combine(installDir, "yafaltapoco.exe");

            // Panel Izquierdo (GIF animado de marca sobre fondo negro puro)
            Panel pnlLeft = new Panel
            {
                Dock = DockStyle.Left,
                Width = 220,
                BackColor = Color.FromArgb(8, 8, 10)
            };
            pnlLeft.Paint += (s, e) =>
            {
                using (Pen borderPen = new Pen(Color.FromArgb(28, 28, 34)))
                {
                    e.Graphics.DrawLine(borderPen, pnlLeft.Width - 1, 0, pnlLeft.Width - 1, pnlLeft.Height);
                }
            };

            picLogo = new PictureBox
            {
                Location = new Point(15, 60),
                Size = new Size(190, 190),
                SizeMode = PictureBoxSizeMode.Zoom,
                BackColor = Color.Transparent
            };
            pnlLeft.Controls.Add(picLogo);

            Label lblBrandName = new Label
            {
                Text = "Ya Falta Poco",
                Location = new Point(10, 260),
                Size = new Size(200, 26),
                TextAlign = ContentAlignment.MiddleCenter,
                Font = new Font("Segoe UI", 12f, FontStyle.Bold),
                ForeColor = Color.White
            };
            pnlLeft.Controls.Add(lblBrandName);

            Label lblBrandTag = new Label
            {
                Text = "v1.0.0 • Agenda & Temporizador",
                Location = new Point(10, 288),
                Size = new Size(200, 20),
                TextAlign = ContentAlignment.MiddleCenter,
                Font = new Font("Segoe UI", 8.5f),
                ForeColor = Color.FromArgb(120, 120, 130)
            };
            pnlLeft.Controls.Add(lblBrandTag);

            Controls.Add(pnlLeft);

            // Panel Derecho (Contenido del Instalador)
            Panel pnlRight = new Panel
            {
                Dock = DockStyle.Fill,
                BackColor = Color.FromArgb(14, 14, 17),
                Padding = new Padding(26, 20, 26, 20)
            };
            Controls.Add(pnlRight);

            lblTitle = new Label
            {
                Text = "Instalación de YaFaltaPoco",
                Location = new Point(24, 18),
                AutoSize = true,
                Font = new Font("Segoe UI", 15f, FontStyle.Bold),
                ForeColor = Color.White
            };
            pnlRight.Controls.Add(lblTitle);

            lblSubtitle = new Label
            {
                Text = "Configura el entorno de escritorio en tu equipo.",
                Location = new Point(25, 48),
                AutoSize = true,
                Font = new Font("Segoe UI", 9f),
                ForeColor = Color.FromArgb(142, 142, 147)
            };
            pnlRight.Controls.Add(lblSubtitle);

            // Pasos de instalación
            int startY = 80;
            lblStepIcons = new Label[stepTitles.Length];
            lblStepTexts = new Label[stepTitles.Length];

            for (int i = 0; i < stepTitles.Length; i++)
            {
                lblStepIcons[i] = new Label
                {
                    Text = "○",
                    Location = new Point(26, startY + (i * 24)),
                    Size = new Size(20, 20),
                    Font = new Font("Segoe UI", 9.5f, FontStyle.Bold),
                    ForeColor = Color.FromArgb(70, 70, 80),
                    TextAlign = ContentAlignment.MiddleCenter
                };
                pnlRight.Controls.Add(lblStepIcons[i]);

                lblStepTexts[i] = new Label
                {
                    Text = stepTitles[i],
                    Location = new Point(48, startY + (i * 24) + 1),
                    AutoSize = true,
                    Font = new Font("Segoe UI", 9.2f),
                    ForeColor = Color.FromArgb(120, 120, 130)
                };
                pnlRight.Controls.Add(lblStepTexts[i]);
            }

            // Barra de progreso moderna
            progressBar = new ModernProgressBar
            {
                Location = new Point(26, 215),
                Size = new Size(405, 8),
                Value = 0
            };
            pnlRight.Controls.Add(progressBar);

            lblStatus = new Label
            {
                Text = "Listo para instalar.",
                Location = new Point(25, 230),
                Size = new Size(330, 20),
                Font = new Font("Segoe UI", 8.5f),
                ForeColor = Color.FromArgb(142, 142, 147)
            };
            pnlRight.Controls.Add(lblStatus);

            lblPercent = new Label
            {
                Text = "0%",
                Location = new Point(370, 230),
                Size = new Size(60, 20),
                Font = new Font("JetBrains Mono", 8.5f, FontStyle.Bold),
                ForeColor = Color.FromArgb(142, 142, 147),
                TextAlign = ContentAlignment.TopRight
            };
            pnlRight.Controls.Add(lblPercent);

            // Opciones de instalación (Checkboxes oscuros)
            chkDesktop = new CheckBox
            {
                Text = "Crear acceso directo en el Escritorio",
                Checked = true,
                Location = new Point(28, 260),
                AutoSize = true,
                Font = new Font("Segoe UI", 9f),
                ForeColor = Color.FromArgb(210, 210, 215)
            };
            pnlRight.Controls.Add(chkDesktop);

            chkLaunch = new CheckBox
            {
                Text = "Iniciar YaFaltaPoco al terminar",
                Checked = true,
                Location = new Point(28, 286),
                AutoSize = true,
                Font = new Font("Segoe UI", 9f),
                ForeColor = Color.FromArgb(210, 210, 215)
            };
            pnlRight.Controls.Add(chkLaunch);

            // Ruta de instalación destino
            lblDestPath = new Label
            {
                Text = "Destino: " + installDir,
                Location = new Point(26, 335),
                Size = new Size(405, 18),
                Font = new Font("Segoe UI", 8f),
                ForeColor = Color.FromArgb(90, 90, 100),
                AutoEllipsis = true
            };
            pnlRight.Controls.Add(lblDestPath);

            // Botones de acción inferiores
            btnCancel = new Button
            {
                Text = "Cancelar",
                Location = new Point(220, 360),
                Size = new Size(100, 32),
                FlatStyle = FlatStyle.Flat,
                BackColor = Color.FromArgb(26, 26, 32),
                ForeColor = Color.FromArgb(170, 170, 180),
                Cursor = Cursors.Hand
            };
            btnCancel.FlatAppearance.BorderSize = 1;
            btnCancel.FlatAppearance.BorderColor = Color.FromArgb(42, 42, 50);
            btnCancel.Click += (s, e) => Close();
            pnlRight.Controls.Add(btnCancel);

            btnInstall = new Button
            {
                Text = "Instalar",
                Location = new Point(330, 360),
                Size = new Size(100, 32),
                FlatStyle = FlatStyle.Flat,
                BackColor = Color.FromArgb(10, 132, 255),
                ForeColor = Color.White,
                Font = new Font("Segoe UI", 9.5f, FontStyle.Bold),
                Cursor = Cursors.Hand
            };
            btnInstall.FlatAppearance.BorderSize = 0;
            btnInstall.Click += BtnInstall_Click;
            pnlRight.Controls.Add(btnInstall);
        }

        private Stream logoStream;
        private Stream iconStream;

        private void LoadEmbeddedAssets()
        {
            try
            {
                Assembly asm = Assembly.GetExecutingAssembly();
                logoStream = asm.GetManifestResourceStream("LogoPayload");
                if (logoStream != null)
                {
                    picLogo.Image = Image.FromStream(logoStream);
                }

                iconStream = asm.GetManifestResourceStream("IconPayload");
                if (iconStream != null)
                {
                    Icon = new Icon(iconStream);
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine("Error al cargar recursos embebidos: " + ex.ToString());
            }
        }

        private void BtnInstall_Click(object sender, EventArgs e)
        {
            if (isFinished)
            {
                if (chkLaunch.Checked && File.Exists(exePath))
                {
                    try
                    {
                        System.Diagnostics.Process.Start(new System.Diagnostics.ProcessStartInfo
                        {
                            FileName = exePath,
                            WorkingDirectory = installDir
                        });
                    }
                    catch (Exception ex)
                    {
                        MessageBox.Show("No se pudo iniciar la aplicación: " + ex.Message, "Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    }
                }
                Close();
                return;
            }

            btnInstall.Enabled = false;
            btnCancel.Enabled = false;
            chkDesktop.Enabled = false;
            chkLaunch.Enabled = false;

            Thread worker = new Thread(RunInstallation)
            {
                IsBackground = true
            };
            worker.Start();
        }

        private void SetStepState(int stepIndex, int state)
        {
            // state: 0 = pending (○), 1 = active (►), 2 = completed (✓)
            if (InvokeRequired)
            {
                Invoke(new Action(() => SetStepState(stepIndex, state)));
                return;
            }

            if (state == 1)
            {
                lblStepIcons[stepIndex].Text = "►";
                lblStepIcons[stepIndex].ForeColor = Color.FromArgb(56, 189, 248); // Cyan vibrante
                lblStepTexts[stepIndex].ForeColor = Color.White;
            }
            else if (state == 2)
            {
                lblStepIcons[stepIndex].Text = "✓";
                lblStepIcons[stepIndex].ForeColor = Color.FromArgb(48, 209, 88); // Verde esmeralda
                lblStepTexts[stepIndex].ForeColor = Color.FromArgb(220, 220, 225);
            }
        }

        private void UpdateProgress(int percent, string statusText)
        {
            if (InvokeRequired)
            {
                Invoke(new Action(() => UpdateProgress(percent, statusText)));
                return;
            }

            progressBar.Value = percent;
            lblPercent.Text = percent + "%";
            lblStatus.Text = statusText;
        }

        public void RunSilentInstallation()
        {
            try
            {
                Assembly asm = Assembly.GetExecutingAssembly();
                if (!Directory.Exists(installDir))
                {
                    Directory.CreateDirectory(installDir);
                }

                byte[] exeBytes = ExtractGzipResource(asm, "AppPayload");
                byte[] dllBytes = ExtractGzipResource(asm, "LoaderPayload");
                byte[] icoBytes = ExtractRawResource(asm, "IconPayload");

                File.WriteAllBytes(exePath, exeBytes);
                string dllPath = Path.Combine(installDir, "WebView2Loader.dll");
                File.WriteAllBytes(dllPath, dllBytes);
                string icoPath = Path.Combine(installDir, "icon.ico");
                if (icoBytes != null && icoBytes.Length > 0)
                {
                    File.WriteAllBytes(icoPath, icoBytes);
                }

                CreateUninstaller();

                string desktopPath = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
                string desktopLink = Path.Combine(desktopPath, "YaFaltaPoco.lnk");
                CreateShortcut(desktopLink, exePath, installDir, icoPath, "Ya Falta Poco — Widget de productividad");

                string startMenuPrograms = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.StartMenu), "Programs", "YaFaltaPoco");
                if (!Directory.Exists(startMenuPrograms))
                {
                    Directory.CreateDirectory(startMenuPrograms);
                }
                string startLink = Path.Combine(startMenuPrograms, "YaFaltaPoco.lnk");
                CreateShortcut(startLink, exePath, installDir, icoPath, "Ya Falta Poco — Widget de productividad");
            }
            catch (Exception ex)
            {
                Console.WriteLine("Error en instalación silenciosa: " + ex.Message);
            }
        }

        private void RunInstallation()
        {
            try
            {
                Assembly asm = Assembly.GetExecutingAssembly();

                // Paso 1: Preparar directorio
                SetStepState(0, 1);
                UpdateProgress(10, "Creando carpeta de instalación...");
                Thread.Sleep(250);

                if (!Directory.Exists(installDir))
                {
                    Directory.CreateDirectory(installDir);
                }
                SetStepState(0, 2);

                // Paso 2: Desempaquetar componentes
                SetStepState(1, 1);
                UpdateProgress(30, "Descomprimiendo binarios de YaFaltaPoco...");
                Thread.Sleep(300);

                byte[] exeBytes = ExtractGzipResource(asm, "AppPayload");
                byte[] dllBytes = ExtractGzipResource(asm, "LoaderPayload");
                byte[] icoBytes = ExtractRawResource(asm, "IconPayload");
                SetStepState(1, 2);

                // Paso 3: Instalar archivos principales
                SetStepState(2, 1);
                UpdateProgress(60, "Instalando yafaltapoco.exe y WebView2Loader.dll...");
                Thread.Sleep(350);

                // Escribir ejecutable principal
                File.WriteAllBytes(exePath, exeBytes);

                // Escribir WebView2Loader.dll
                string dllPath = Path.Combine(installDir, "WebView2Loader.dll");
                File.WriteAllBytes(dllPath, dllBytes);

                // Escribir icono
                string icoPath = Path.Combine(installDir, "icon.ico");
                if (icoBytes != null && icoBytes.Length > 0)
                {
                    File.WriteAllBytes(icoPath, icoBytes);
                }

                // Escribir desinstalador por script
                CreateUninstaller();
                SetStepState(2, 2);

                // Paso 4: Crear acceso directo en el Escritorio
                SetStepState(3, 1);
                UpdateProgress(80, "Creando acceso directo en el Escritorio...");
                Thread.Sleep(250);

                if (chkDesktop.Checked)
                {
                    string desktopPath = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
                    string desktopLink = Path.Combine(desktopPath, "YaFaltaPoco.lnk");
                    CreateShortcut(desktopLink, exePath, installDir, icoPath, "Ya Falta Poco — Widget de productividad");
                }
                SetStepState(3, 2);

                // Paso 5: Registrar en el Menú Inicio
                SetStepState(4, 1);
                UpdateProgress(95, "Registrando accesos en el Menú Inicio...");
                Thread.Sleep(250);

                string startMenuPrograms = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.StartMenu), "Programs", "YaFaltaPoco");
                if (!Directory.Exists(startMenuPrograms))
                {
                    Directory.CreateDirectory(startMenuPrograms);
                }

                string startLink = Path.Combine(startMenuPrograms, "YaFaltaPoco.lnk");
                CreateShortcut(startLink, exePath, installDir, icoPath, "Ya Falta Poco — Widget de productividad");

                string uninstLink = Path.Combine(startMenuPrograms, "Desinstalar YaFaltaPoco.lnk");
                CreateShortcut(uninstLink, Path.Combine(installDir, "desinstalar.cmd"), installDir, null, "Desinstalar Ya Falta Poco");

                SetStepState(4, 2);

                // Finalización
                UpdateProgress(100, "¡Instalación completada con éxito!");

                Invoke(new Action(() =>
                {
                    isFinished = true;
                    btnInstall.Enabled = true;
                    btnInstall.Text = "Iniciar";
                    btnInstall.BackColor = Color.FromArgb(48, 209, 88); // Verde esmeralda
                    btnCancel.Enabled = true;
                    btnCancel.Text = "Cerrar";
                }));
            }
            catch (Exception ex)
            {
                Invoke(new Action(() =>
                {
                    MessageBox.Show("Ocurrió un error durante la instalación:\n\n" + ex.Message, "Error de instalación", MessageBoxButtons.OK, MessageBoxIcon.Error);
                    btnInstall.Enabled = true;
                    btnCancel.Enabled = true;
                }));
            }
        }

        private byte[] ExtractGzipResource(Assembly asm, string resourceName)
        {
            using (Stream s = asm.GetManifestResourceStream(resourceName))
            {
                if (s == null) throw new Exception("Recurso no encontrado: " + resourceName);
                using (GZipStream gz = new GZipStream(s, CompressionMode.Decompress))
                using (MemoryStream ms = new MemoryStream())
                {
                    byte[] buffer = new byte[81920];
                    int read;
                    while ((read = gz.Read(buffer, 0, buffer.Length)) > 0)
                    {
                        ms.Write(buffer, 0, read);
                    }
                    return ms.ToArray();
                }
            }
        }

        private byte[] ExtractRawResource(Assembly asm, string resourceName)
        {
            using (Stream s = asm.GetManifestResourceStream(resourceName))
            {
                if (s == null) return new byte[0];
                using (MemoryStream ms = new MemoryStream())
                {
                    s.CopyTo(ms);
                    return ms.ToArray();
                }
            }
        }

        private void CreateShortcut(string shortcutPath, string targetPath, string workDir, string iconLocation, string description)
        {
            try
            {
                Type shellType = Type.GetTypeFromProgID("WScript.Shell");
                if (shellType == null) return;
                object shell = Activator.CreateInstance(shellType);
                object shortcut = shellType.InvokeMember("CreateShortcut", BindingFlags.InvokeMethod, null, shell, new object[] { shortcutPath });
                if (shortcut == null) return;
                Type scType = shortcut.GetType();
                scType.InvokeMember("TargetPath", BindingFlags.SetProperty, null, shortcut, new object[] { targetPath });
                scType.InvokeMember("WorkingDirectory", BindingFlags.SetProperty, null, shortcut, new object[] { workDir });
                if (!string.IsNullOrEmpty(iconLocation) && File.Exists(iconLocation))
                {
                    scType.InvokeMember("IconLocation", BindingFlags.SetProperty, null, shortcut, new object[] { iconLocation + ",0" });
                }
                scType.InvokeMember("Description", BindingFlags.SetProperty, null, shortcut, new object[] { description });
                scType.InvokeMember("Save", BindingFlags.InvokeMethod, null, shortcut, null);
            }
            catch (Exception ex)
            {
                Console.WriteLine("No se pudo crear acceso directo: " + ex.Message);
            }
        }

        private void CreateUninstaller()
        {
            try
            {
                string uninstallerCmd = Path.Combine(installDir, "desinstalar.cmd");
                string cmdContent = "@echo off\r\n" +
                    "chcp 65001 >nul\r\n" +
                    "echo Cerrando YaFaltaPoco...\r\n" +
                    "taskkill /f /im yafaltapoco.exe >nul 2>&1\r\n" +
                    "ping 127.0.0.1 -n 2 >nul\r\n" +
                    "echo Eliminando accesos directos...\r\n" +
                    "del \"%USERPROFILE%\\Desktop\\YaFaltaPoco.lnk\" >nul 2>&1\r\n" +
                    "rmdir /s /q \"%APPDATA%\\Microsoft\\Windows\\Start Menu\\Programs\\YaFaltaPoco\" >nul 2>&1\r\n" +
                    "echo Eliminando archivos...\r\n" +
                    "del \"%~dp0yafaltapoco.exe\" >nul 2>&1\r\n" +
                    "del \"%~dp0WebView2Loader.dll\" >nul 2>&1\r\n" +
                    "del \"%~dp0icon.ico\" >nul 2>&1\r\n" +
                    "reg delete \"HKCU\\Software\\Microsoft\\Windows\\CurrentVersion\\Uninstall\\YaFaltaPoco\" /f >nul 2>&1\r\n" +
                    "echo ¡YaFaltaPoco ha sido desinstalado correctamente!\r\n" +
                    "ping 127.0.0.1 -n 2 >nul\r\n" +
                    "cd \"%TEMP%\"\r\n" +
                    "rmdir /s /q \"%~dp0\" >nul 2>&1\r\n" +
                    "exit\r\n";

                File.WriteAllText(uninstallerCmd, cmdContent);

                // Registrar en Windows Add/Remove Programs
                using (RegistryKey key = Registry.CurrentUser.CreateSubKey(@"Software\Microsoft\Windows\CurrentVersion\Uninstall\YaFaltaPoco"))
                {
                    if (key != null)
                    {
                        key.SetValue("DisplayName", "Ya Falta Poco");
                        key.SetValue("DisplayVersion", "1.0.0");
                        key.SetValue("Publisher", "ludwingx");
                        key.SetValue("DisplayIcon", Path.Combine(installDir, "yafaltapoco.exe"));
                        key.SetValue("UninstallString", "\"" + uninstallerCmd + "\"");
                        key.SetValue("InstallLocation", installDir);
                        key.SetValue("NoModify", 1, RegistryValueKind.DWord);
                        key.SetValue("NoRepair", 1, RegistryValueKind.DWord);
                    }
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine("Error al crear desinstalador: " + ex.Message);
            }
        }
    }
}
