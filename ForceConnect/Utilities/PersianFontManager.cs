using System;
using System.Drawing;
using System.Drawing.Text;
using System.IO;
using System.Linq;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace ForceConnect.Utilities
{
    /// <summary>
    /// Manages Persian font installation and rendering for proper Persian text display.
    /// </summary>
    public static class PersianFontManager
    {
        [DllImport("gdi32.dll", EntryPoint = "AddFontResourceW", SetLastError = true)]
        public static extern int AddFontResource([In][MarshalAs(UnmanagedType.LPWStr)] string lpFileName);

        [DllImport("gdi32.dll", EntryPoint = "RemoveFontResourceW", SetLastError = true)]
        public static extern int RemoveFontResource([In][MarshalAs(UnmanagedType.LPWStr)] string lpFileName);

        private static readonly string[] _persianFontNames = 
        {
            "Tahoma",
            "Arial",
            "Segoe UI",
            "Microsoft Sans Serif",
            "Verdana"
        };

        private static readonly string _fontDirectory = Path.Combine(Application.StartupPath, "Fonts");
        private static bool _persianFontsInstalled = false;

        /// <summary>
        /// Installs Persian fonts for proper text rendering.
        /// </summary>
        /// <returns>True if fonts were successfully installed, false otherwise.</returns>
        public static bool InstallPersianFonts()
        {
            try
            {
                if (_persianFontsInstalled)
                    return true;

                // Check if any Persian fonts are available
                var availableFonts = GetAvailablePersianFonts();
                if (availableFonts.Length > 0)
                {
                    _persianFontsInstalled = true;
                    return true;
                }

                // Try to install fonts from Fonts directory
                if (Directory.Exists(_fontDirectory))
                {
                    var fontFiles = Directory.GetFiles(_fontDirectory, "*.ttf");
                    foreach (var fontFile in fontFiles)
                    {
                        try
                        {
                            AddFontResource(fontFile);
                        }
                        catch (Exception ex)
                        {
                            System.Diagnostics.Debug.WriteLine($"Failed to install font {fontFile}: {ex.Message}");
                        }
                    }
                }

                _persianFontsInstalled = true;
                return true;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to install Persian fonts: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Gets available Persian fonts on the system.
        /// </summary>
        /// <returns>Array of available Persian font names.</returns>
        public static string[] GetAvailablePersianFonts()
        {
            try
            {
                var availableFonts = new System.Collections.Generic.List<string>();
                
                foreach (var fontName in _persianFontNames)
                {
                    if (FontFamily.Families.Any(f => f.Name == fontName))
                    {
                        availableFonts.Add(fontName);
                    }
                }

                return availableFonts.ToArray();
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get available Persian fonts: {ex.Message}");
                return new string[0];
            }
        }

        /// <summary>
        /// Gets the best available Persian font for the system.
        /// </summary>
        /// <returns>The best available Persian font name, or "Tahoma" as fallback.</returns>
        public static string GetBestPersianFont()
        {
            var availableFonts = GetAvailablePersianFonts();
            return availableFonts.Length > 0 ? availableFonts[0] : "Tahoma";
        }

        /// <summary>
        /// Applies Persian font to a control with proper size and style.
        /// </summary>
        /// <param name="control">The control to apply font to.</param>
        /// <param name="fontSize">The font size (optional, uses control's current size if not specified).</param>
        public static void ApplyPersianFont(Control control, float? fontSize = null)
        {
            if (control == null) return;

            try
            {
                var fontName = GetBestPersianFont();
                var size = fontSize ?? control.Font.Size;
                var style = control.Font.Style;

                control.Font = new Font(fontName, size, style);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to apply Persian font to control: {ex.Message}");
            }
        }

        /// <summary>
        /// Applies Persian font to all controls in a container.
        /// </summary>
        /// <param name="container">The container control.</param>
        /// <param name="fontSize">The font size (optional).</param>
        public static void ApplyPersianFontToContainer(Control container, float? fontSize = null)
        {
            if (container == null) return;

            try
            {
                ApplyPersianFont(container, fontSize);

                foreach (Control childControl in container.Controls)
                {
                    ApplyPersianFontToContainer(childControl, fontSize);
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to apply Persian font to container: {ex.Message}");
            }
        }

        /// <summary>
        /// Creates a Persian font with specified parameters.
        /// </summary>
        /// <param name="fontSize">The font size.</param>
        /// <param name="fontStyle">The font style.</param>
        /// <returns>A Persian font instance.</returns>
        public static Font CreatePersianFont(float fontSize, FontStyle fontStyle = FontStyle.Regular)
        {
            try
            {
                var fontName = GetBestPersianFont();
                return new Font(fontName, fontSize, fontStyle);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to create Persian font: {ex.Message}");
                return new Font("Tahoma", fontSize, fontStyle);
            }
        }

        /// <summary>
        /// Checks if Persian fonts are properly installed and available.
        /// </summary>
        /// <returns>True if Persian fonts are available, false otherwise.</returns>
        public static bool ArePersianFontsAvailable()
        {
            return GetAvailablePersianFonts().Length > 0;
        }

        /// <summary>
        /// Gets font information for debugging purposes.
        /// </summary>
        /// <returns>String containing font information.</returns>
        public static string GetFontInformation()
        {
            try
            {
                var availableFonts = GetAvailablePersianFonts();
                var bestFont = GetBestPersianFont();
                
                return $"Available Persian fonts: {string.Join(", ", availableFonts)}\n" +
                       $"Best font: {bestFont}\n" +
                       $"Fonts installed: {_persianFontsInstalled}";
            }
            catch (Exception ex)
            {
                return $"Failed to get font information: {ex.Message}";
            }
        }
    }
} 