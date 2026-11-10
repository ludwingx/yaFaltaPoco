using System;
using System.Drawing;
using System.Drawing.Drawing2D;
using System.IO;
using System.IO.Compression;
using System.Reflection;
using System.Runtime.InteropServices;
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

    // Dibujado de rectángulos con bordes redondeados
    internal static class GraphicsExtensions
    {
        public static GraphicsPath GetRoundedPath(Rectangle rect, int radius)
        {
            GraphicsPath path = new GraphicsPath();
            if (radius <= 0)
            {
                path.AddRectangle(rect);
                return path;
            }

            int d = radius * 2;
            Rectangle arc = new Rectangle(rect.X, rect.Y, d, d);

            path.AddArc(arc, 180, 90);
            arc.X = rect.Right - d;
            path.AddArc(arc, 270, 90);
            arc.Y = rect.Bottom - d;
            path.AddArc(arc, 0, 90);
            arc.X = rect.Left;
            path.AddArc(arc, 90, 90);
            path.CloseFigure();
            return path;
        }

        public static void FillRoundedRectangle(this Graphics g, Brush brush, Rectangle rect, int radius)
        {
            using (GraphicsPath path = GetRoundedPath(rect, radius))
            {
                g.FillPath(brush, path);
            }
        }

        public static void DrawRoundedRectangle(this Graphics g, Pen pen, Rectangle rect, int radius)
        {
            using (GraphicsPath path = GetRoundedPath(rect, radius))
            {
                g.DrawPath(pen, path);
            }
        }
    }

    // Botón moderno personalizado con hover y colores dark
    public class ModernButton : Button
    {
        public bool IsPrimary { get; set; }
        public bool IsSuccess { get; set; }
        private bool isHovered = false;
        private bool isPressed = false;

        public ModernButton()
        {
            SetStyle(ControlStyles.UserPaint | ControlStyles.AllPaintingInWmPaint | ControlStyles.OptimizedDoubleBuffer, true);
            Cursor = Cursors.Hand;
            Font = new Font("Segoe UI", 9.5f, FontStyle.Bold);
            Height = 36;
            Width = 120;
        }

        protected override void OnMouseEnter(EventArgs e)
        {
            base.OnMouseEnter(e);
            isHovered = true;
            Invalidate();
        }

        protected override void OnMouseLeave(EventArgs e)
        {
            base.OnMouseLeave(e);
            isHovered = false;
            isPressed = false;
            Invalidate();
        }

        protected override void OnMouseDown(MouseEventArgs mevent)
        {
            base.OnMouseDown(mevent);
            if (mevent.Button == MouseButtons.Left)
            {
                isPressed = true;
                Invalidate();
            }
        }

        protected override void OnMouseUp(MouseEventArgs mevent)
        {
            base.OnMouseUp(mevent);
            isPressed = false;
            Invalidate();
        }

        protected override void OnPaint(PaintEventArgs pevent)
        {
            Graphics g = pevent.Graphics;
            g.SmoothingMode = SmoothingMode.AntiAlias;

            Rectangle rect = new Rectangle(0, 0, Width - 1, Height - 1);

            if (!Enabled)
            {
                using (SolidBrush b = new SolidBrush(Color.FromArgb(24, 24, 30)))
                    g.FillRoundedRectangle(b, rect, 6);
                using (Pen p = new Pen(Color.FromArgb(38, 38, 46)))
                    g.DrawRoundedRectangle(p, rect, 6);
                TextRenderer.DrawText(g, Text, Font, rect, Color.FromArgb(90, 90, 100), TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
                return;
            }

            if (IsSuccess)
            {
                Color bg = isPressed ? Color.FromArgb(36, 175, 72) : (isHovered ? Color.FromArgb(52, 219, 98) : Color.FromArgb(48, 209, 88));
                using (SolidBrush b = new SolidBrush(bg))
                    g.FillRoundedRectangle(b, rect, 6);
                TextRenderer.DrawText(g, Text, Font, rect, Color.White, TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
                return;
            }

            if (IsPrimary)
            {
                Color c1 = isPressed ? Color.FromArgb(0, 102, 204) : (isHovered ? Color.FromArgb(24, 144, 255) : Color.FromArgb(10, 132, 255));
                Color c2 = isPressed ? Color.FromArgb(0, 85, 179) : (isHovered ? Color.FromArgb(0, 120, 235) : Color.FromArgb(0, 113, 227));
                using (LinearGradientBrush lgb = new LinearGradientBrush(rect, c1, c2, LinearGradientMode.Vertical))
                {
                    g.FillRoundedRectangle(lgb, rect, 6);
                }
                using (Pen p = new Pen(Color.FromArgb(56, 189, 248), 1))
                {
                    if (isHovered)
                        g.DrawRoundedRectangle(p, rect, 6);
                }
                TextRenderer.DrawText(g, Text, Font, rect, Color.White, TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
            }
            else
            {
                Color bg = isPressed ? Color.FromArgb(22, 22, 28) : (isHovered ? Color.FromArgb(36, 36, 46) : Color.FromArgb(26, 26, 34));
                Color border = isHovered ? Color.FromArgb(70, 70, 85) : Color.FromArgb(45, 45, 56);
                using (SolidBrush b = new SolidBrush(bg))
                    g.FillRoundedRectangle(b, rect, 6);
                using (Pen p = new Pen(border))
                    g.DrawRoundedRectangle(p, rect, 6);
                Color textCol = isHovered ? Color.White : Color.FromArgb(180, 180, 190);
                TextRenderer.DrawText(g, Text, Font, rect, textCol, TextFormatFlags.HorizontalCenter | TextFormatFlags.VerticalCenter);
            }
        }
    }

    // Checkbox dark moderno personalizado
    public class ModernCheckBox : Control
    {
        private bool _checked = true;
        public bool Checked
        {
            get { return _checked; }
            set
            {
                if (_checked != value)
                {
                    _checked = value;
                    Invalidate();
                    if (CheckedChanged != null)
                    {
                        CheckedChanged(this, EventArgs.Empty);
                    }
                }
            }
        }

        public event EventHandler CheckedChanged;
        private bool isHovered = false;

        public ModernCheckBox()
        {
            SetStyle(ControlStyles.UserPaint | ControlStyles.AllPaintingInWmPaint | ControlStyles.OptimizedDoubleBuffer, true);
            Cursor = Cursors.Hand;
            Font = new Font("Segoe UI", 9.25f);
            ForeColor = Color.FromArgb(220, 220, 225);
            Height = 26;
            Width = 320;
        }

        protected override void OnMouseEnter(EventArgs e)
        {
            base.OnMouseEnter(e);
            isHovered = true;
            Invalidate();
        }

        protected override void OnMouseLeave(EventArgs e)
        {
            base.OnMouseLeave(e);
            isHovered = false;
            Invalidate();
        }

        protected override void OnClick(EventArgs e)
        {
            Checked = !Checked;
            base.OnClick(e);
        }

        protected override void OnPaint(PaintEventArgs pevent)
        {
            Graphics g = pevent.Graphics;
            g.SmoothingMode = SmoothingMode.AntiAlias;

            // Caja del checkbox
            int boxSize = 17;
            int boxY = (Height - boxSize) / 2;
            Rectangle boxRect = new Rectangle(1, boxY, boxSize, boxSize);

            if (_checked)
            {
                using (SolidBrush fill = new SolidBrush(Color.FromArgb(10, 132, 255)))
                {
                    g.FillRoundedRectangle(fill, boxRect, 4);
                }
                using (Pen checkPen = new Pen(Color.White, 2.2f))
                {
                    checkPen.StartCap = LineCap.Round;
                    checkPen.EndCap = LineCap.Round;
                    g.DrawLines(checkPen, new Point[] {
                        new Point(boxRect.Left + 4, boxRect.Top + 9),
                        new Point(boxRect.Left + 7, boxRect.Top + 12),
                        new Point(boxRect.Left + 13, boxRect.Top + 5)
                    });
                }
            }
            else
            {
                using (SolidBrush bg = new SolidBrush(Color.FromArgb(24, 24, 30)))
                {
                    g.FillRoundedRectangle(bg, boxRect, 4);
                }
                Color borderColor = isHovered ? Color.FromArgb(90, 90, 110) : Color.FromArgb(50, 50, 62);
                using (Pen border = new Pen(borderColor, 1.2f))
                {
                    g.DrawRoundedRectangle(border, boxRect, 4);
                }
            }

            // Texto
            Rectangle textRect = new Rectangle(boxSize + 10, 0, Width - (boxSize + 10), Height);
            Color textCol = isHovered ? Color.White : ForeColor;
            TextRenderer.DrawText(g, Text, Font, textRect, textCol, TextFormatFlags.Left | TextFormatFlags.VerticalCenter);
        }
    }

    // Barra de progreso con gradiente y bordes redondeados
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
            Height = 8;
        }

        protected override void OnPaint(PaintEventArgs e)
        {
            Graphics g = e.Graphics;
            g.SmoothingMode = SmoothingMode.AntiAlias;

            Rectangle rect = new Rectangle(0, 0, Width - 1, Height - 1);

            // Fondo del riel
            using (SolidBrush bgBrush = new SolidBrush(Color.FromArgb(24, 24, 30)))
            {
                g.FillRoundedRectangle(bgBrush, rect, 4);
            }
            using (Pen borderPen = new Pen(Color.FromArgb(38, 38, 48)))
            {
                g.DrawRoundedRectangle(borderPen, rect, 4);
            }

            // Progreso
            if (_value > 0)
            {
                int fillWidth = (int)((Width - 2) * (_value / 100.0));
                if (fillWidth > 4)
                {
                    Rectangle fillRect = new Rectangle(1, 1, fillWidth, Height - 2);
                    using (LinearGradientBrush fillBrush = new LinearGradientBrush(
                        fillRect,
                        Color.FromArgb(10, 132, 255),
                        Color.FromArgb(34, 211, 238),
                        LinearGradientMode.Horizontal))
                    {
                        g.FillRoundedRectangle(fillBrush, fillRect, 3);
                    }
                }
            }
        }
    }

    public class InstallerForm : Form
    {
        [DllImport("dwmapi.dll", PreserveSig = true)]
        private static extern int DwmSetWindowAttribute(IntPtr hwnd, int attr, ref int attrValue, int attrSize);

        private Label lblAppTitle;
        private Label lblVersionBadge;
        private Label lblSubtitle;
        private ModernProgressBar progressBar;
        private Label lblStatus;
        private Label lblPercent;
        private ModernCheckBox chkDesktop;
        private ModernCheckBox chkLaunch;
        private Label lblDestDir;
        private ModernButton btnCancel;
        private ModernButton btnInstall;

        private bool isFinished = false;
        private string installDir;
        private string exePath;
        private Icon appIcon;

        public InstallerForm()
        {
            InitializeComponent();
            ApplyNativeDarkTheme();
            LoadEmbeddedAssets();
        }

        private void ApplyNativeDarkTheme()
        {
            try
            {
                // DWMWA_USE_IMMERSIVE_DARK_MODE = 20 (Windows 11 / 10 20H1+), 19 (Windows 10 anteriores)
                int darkMode = 1;
                DwmSetWindowAttribute(Handle, 20, ref darkMode, sizeof(int));
                DwmSetWindowAttribute(Handle, 19, ref darkMode, sizeof(int));

                // DWMWA_WINDOW_CORNER_PREFERENCE = 33 (Windows 11 esquinas redondeadas)
                int cornerRound = 2; // DWMWCP_ROUND
                DwmSetWindowAttribute(Handle, 33, ref cornerRound, sizeof(int));
            }
            catch { }
        }

        private void InitializeComponent()
        {
            Text = "Instalador de YaFaltaPoco";
            ClientSize = new Size(580, 420);
            StartPosition = FormStartPosition.CenterScreen;
            FormBorderStyle = FormBorderStyle.FixedDialog;
            MaximizeBox = false;
            BackColor = Color.FromArgb(14, 14, 18);
            ForeColor = Color.White;
            Font = new Font("Segoe UI", 9.5f, FontStyle.Regular);

            installDir = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.LocalApplicationData), "Programs", "YaFaltaPoco");
            exePath = Path.Combine(installDir, "yafaltapoco.exe");

            // Header Hero (Icono + Título + Versión + Subtítulo)
            Panel pnlHeader = new Panel
            {
                Location = new Point(0, 0),
                Size = new Size(580, 96),
                BackColor = Color.FromArgb(17, 17, 23)
            };
            pnlHeader.Paint += (s, e) =>
            {
                using (Pen p = new Pen(Color.FromArgb(32, 32, 42)))
                {
                    e.Graphics.DrawLine(p, 0, pnlHeader.Height - 1, pnlHeader.Width, pnlHeader.Height - 1);
                }

                // Insignia con el Icono de la App
                Rectangle badgeRect = new Rectangle(28, 20, 56, 56);
                using (SolidBrush b = new SolidBrush(Color.FromArgb(25, 25, 34)))
                    e.Graphics.FillRoundedRectangle(b, badgeRect, 10);
                using (Pen pen = new Pen(Color.FromArgb(45, 45, 60), 1.2f))
                    e.Graphics.DrawRoundedRectangle(pen, badgeRect, 10);

                if (appIcon != null)
                {
                    e.Graphics.DrawIcon(appIcon, new Rectangle(badgeRect.X + 12, badgeRect.Y + 12, 32, 32));
                }
            };
            Controls.Add(pnlHeader);

            lblAppTitle = new Label
            {
                Text = "Ya Falta Poco",
                Location = new Point(98, 22),
                AutoSize = true,
                Font = new Font("Segoe UI", 16f, FontStyle.Bold),
                ForeColor = Color.White
            };
            pnlHeader.Controls.Add(lblAppTitle);

            lblVersionBadge = new Label
            {
                Text = "v1.0.0",
                Location = new Point(255, 29),
                Size = new Size(54, 20),
                TextAlign = ContentAlignment.MiddleCenter,
                Font = new Font("Segoe UI", 8.25f, FontStyle.Bold),
                ForeColor = Color.FromArgb(34, 211, 238),
                BackColor = Color.FromArgb(20, 36, 46)
            };
            lblVersionBadge.Paint += (s, e) =>
            {
                using (Pen p = new Pen(Color.FromArgb(34, 211, 238, 100)))
                    e.Graphics.DrawRoundedRectangle(p, new Rectangle(0, 0, lblVersionBadge.Width - 1, lblVersionBadge.Height - 1), 3);
            };
            pnlHeader.Controls.Add(lblVersionBadge);

            lblSubtitle = new Label
            {
                Text = "Compañero visual de productividad y cuenta regresiva para escritorio.",
                Location = new Point(100, 54),
                AutoSize = true,
                Font = new Font("Segoe UI", 9f),
                ForeColor = Color.FromArgb(150, 150, 160)
            };
            pnlHeader.Controls.Add(lblSubtitle);

            // Tarjeta de Opciones y Directorio (Centro)
            Panel pnlCard = new Panel
            {
                Location = new Point(28, 114),
                Size = new Size(524, 156),
                BackColor = Color.FromArgb(20, 20, 26)
            };
            pnlCard.Paint += (s, e) =>
            {
                e.Graphics.SmoothingMode = SmoothingMode.AntiAlias;
                Rectangle r = new Rectangle(0, 0, pnlCard.Width - 1, pnlCard.Height - 1);
                using (Pen p = new Pen(Color.FromArgb(36, 36, 48)))
                    e.Graphics.DrawRoundedRectangle(p, r, 8);
            };
            Controls.Add(pnlCard);

            Label lblDestTag = new Label
            {
                Text = "DIRECTORIO DE INSTALACIÓN",
                Location = new Point(18, 16),
                AutoSize = true,
                Font = new Font("Segoe UI", 7.5f, FontStyle.Bold),
                ForeColor = Color.FromArgb(100, 100, 115)
            };
            pnlCard.Controls.Add(lblDestTag);

            lblDestDir = new Label
            {
                Text = installDir,
                Location = new Point(18, 36),
                Size = new Size(488, 26),
                Font = new Font("Segoe UI", 8.5f),
                ForeColor = Color.FromArgb(200, 200, 210),
                BackColor = Color.FromArgb(15, 15, 20),
                TextAlign = ContentAlignment.MiddleLeft,
                Padding = new Padding(8, 0, 8, 0),
                AutoEllipsis = true
            };
            lblDestDir.Paint += (s, e) =>
            {
                using (Pen p = new Pen(Color.FromArgb(32, 32, 42)))
                    e.Graphics.DrawRoundedRectangle(p, new Rectangle(0, 0, lblDestDir.Width - 1, lblDestDir.Height - 1), 4);
            };
            pnlCard.Controls.Add(lblDestDir);

            chkDesktop = new ModernCheckBox
            {
                Text = "Crear acceso directo en el Escritorio",
                Checked = true,
                Location = new Point(18, 80),
                Size = new Size(350, 24)
            };
            pnlCard.Controls.Add(chkDesktop);

            chkLaunch = new ModernCheckBox
            {
                Text = "Iniciar YaFaltaPoco al terminar la instalación",
                Checked = true,
                Location = new Point(18, 114),
                Size = new Size(350, 24)
            };
            pnlCard.Controls.Add(chkLaunch);

            // Barra de progreso y estado
            progressBar = new ModernProgressBar
            {
                Location = new Point(28, 292),
                Size = new Size(524, 8),
                Value = 0
            };
            Controls.Add(progressBar);

            lblStatus = new Label
            {
                Text = "Listo para instalar en este equipo.",
                Location = new Point(28, 308),
                Size = new Size(440, 20),
                Font = new Font("Segoe UI", 8.5f),
                ForeColor = Color.FromArgb(150, 150, 162)
            };
            Controls.Add(lblStatus);

            lblPercent = new Label
            {
                Text = "0%",
                Location = new Point(480, 308),
                Size = new Size(72, 20),
                Font = new Font("Segoe UI", 8.5f, FontStyle.Bold),
                ForeColor = Color.FromArgb(150, 150, 162),
                TextAlign = ContentAlignment.TopRight
            };
            Controls.Add(lblPercent);

            // Barra Inferior de Acciones
            Panel pnlFooter = new Panel
            {
                Location = new Point(0, 344),
                Size = new Size(580, 76),
                BackColor = Color.FromArgb(17, 17, 23)
            };
            pnlFooter.Paint += (s, e) =>
            {
                using (Pen p = new Pen(Color.FromArgb(32, 32, 42)))
                    e.Graphics.DrawLine(p, 0, 0, pnlFooter.Width, 0);
            };
            Controls.Add(pnlFooter);

            Label lblInfoNote = new Label
            {
                Text = "Instalación local y segura • No requiere permisos de administrador.",
                Location = new Point(28, 24),
                Size = new Size(270, 32),
                Font = new Font("Segoe UI", 7.5f),
                ForeColor = Color.FromArgb(90, 90, 105)
            };
            pnlFooter.Controls.Add(lblInfoNote);

            btnCancel = new ModernButton
            {
                Text = "Cancelar",
                Location = new Point(310, 18),
                Size = new Size(105, 36),
                IsPrimary = false
            };
            btnCancel.Click += (s, e) => Close();
            pnlFooter.Controls.Add(btnCancel);

            btnInstall = new ModernButton
            {
                Text = "Instalar",
                Location = new Point(425, 18),
                Size = new Size(125, 36),
                IsPrimary = true
            };
            btnInstall.Click += BtnInstall_Click;
            pnlFooter.Controls.Add(btnInstall);
        }

        private void LoadEmbeddedAssets()
        {
            try
            {
                Assembly asm = Assembly.GetExecutingAssembly();
                Stream iconStream = asm.GetManifestResourceStream("IconPayload");
                if (iconStream != null)
                {
                    appIcon = new Icon(iconStream);
                    Icon = appIcon;
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine("Error al cargar icono embebido: " + ex.Message);
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

                UpdateProgress(10, "Preparando directorio de instalación...");
                Thread.Sleep(200);

                if (!Directory.Exists(installDir))
                {
                    Directory.CreateDirectory(installDir);
                }

                UpdateProgress(35, "Descomprimiendo binarios de YaFaltaPoco...");
                Thread.Sleep(250);

                byte[] exeBytes = ExtractGzipResource(asm, "AppPayload");
                byte[] dllBytes = ExtractGzipResource(asm, "LoaderPayload");
                byte[] icoBytes = ExtractRawResource(asm, "IconPayload");

                UpdateProgress(65, "Instalando ejecutable y dependencias del sistema...");
                Thread.Sleep(250);

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

                // Escribir desinstalador
                CreateUninstaller();

                UpdateProgress(85, "Configurando accesos directos...");
                Thread.Sleep(200);

                if (chkDesktop.Checked)
                {
                    string desktopPath = Environment.GetFolderPath(Environment.SpecialFolder.DesktopDirectory);
                    string desktopLink = Path.Combine(desktopPath, "YaFaltaPoco.lnk");
                    CreateShortcut(desktopLink, exePath, installDir, icoPath, "Ya Falta Poco — Widget de productividad");
                }

                UpdateProgress(95, "Registrando en el Menú Inicio de Windows...");
                Thread.Sleep(200);

                string startMenuPrograms = Path.Combine(Environment.GetFolderPath(Environment.SpecialFolder.StartMenu), "Programs", "YaFaltaPoco");
                if (!Directory.Exists(startMenuPrograms))
                {
                    Directory.CreateDirectory(startMenuPrograms);
                }

                string startLink = Path.Combine(startMenuPrograms, "YaFaltaPoco.lnk");
                CreateShortcut(startLink, exePath, installDir, icoPath, "Ya Falta Poco — Widget de productividad");

                string uninstLink = Path.Combine(startMenuPrograms, "Desinstalar YaFaltaPoco.lnk");
                CreateShortcut(uninstLink, Path.Combine(installDir, "desinstalar.cmd"), installDir, null, "Desinstalar Ya Falta Poco");

                UpdateProgress(100, "¡Instalación completada con éxito!");

                Invoke(new Action(() =>
                {
                    isFinished = true;
                    btnInstall.Enabled = true;
                    btnInstall.IsPrimary = false;
                    btnInstall.IsSuccess = true;
                    btnInstall.Text = "Iniciar App";
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
