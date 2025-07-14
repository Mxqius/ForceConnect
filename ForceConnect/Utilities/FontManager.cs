using System;
using System.Drawing;
using System.Drawing.Text;
using System.IO;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace ForceConnect.Utilities
{
    /// <summary>
    /// Manages font installation and loading with optimized memory management.
    /// </summary>
    public class FontManager : IDisposable
    {
        [DllImport("gdi32.dll", EntryPoint = "AddFontResourceW", SetLastError = true)]
        public static extern int AddFontResource([In][MarshalAs(UnmanagedType.LPWStr)]
                                         string lpFileName);
        [DllImport("gdi32.dll", EntryPoint = "RemoveFontResourceW", SetLastError = true)]
        public static extern int RemoveFontResource([In][MarshalAs(UnmanagedType.LPWStr)]
                                            string lpFileName);

        private static readonly string _fontDirectory = Path.Combine(Application.StartupPath, "Fonts");
        private static PrivateFontCollection _fontCollection = new PrivateFontCollection();
        private static bool _fontsInstalled = false;
        private static readonly object _lockObject = new object();
        private bool _disposed = false;

        /// <summary>
        /// Installs fonts with optimized memory management and error handling.
        /// </summary>
        /// <returns>True if fonts were successfully installed, false otherwise.</returns>
        public static bool InstallFonts()
        {
            lock (_lockObject)
            {
                if (_fontsInstalled)
                    return _fontCollection.Families.Length > 0;

                try
                {
                    if (!Directory.Exists(_fontDirectory))
                        return false;

                    var fontFiles = Directory.GetFiles(_fontDirectory, "*.ttf");
                    if (fontFiles.Length == 0)
                        return false;

                    foreach (string fontFile in fontFiles)
                    {
                        InstallFont(fontFile);
                    }

                    _fontsInstalled = _fontCollection.Families.Length > 0;
                    return _fontsInstalled;
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Font installation failed: {ex.Message}");
                    return false;
                }
            }
        }

        /// <summary>
        /// Installs a single font with proper memory management.
        /// </summary>
        /// <param name="fontFile">Path to the font file.</param>
        private static void InstallFont(string fontFile)
        {
            IntPtr fontPtr = IntPtr.Zero;
            try
            {
                var fontData = File.ReadAllBytes(fontFile);
                fontPtr = Marshal.AllocCoTaskMem(fontData.Length);
                Marshal.Copy(fontData, 0, fontPtr, fontData.Length);
                
                AddFontResource(fontFile);
                _fontCollection.AddMemoryFont(fontPtr, fontData.Length);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to install font {fontFile}: {ex.Message}");
            }
            finally
            {
                if (fontPtr != IntPtr.Zero)
                {
                    Marshal.FreeCoTaskMem(fontPtr);
                }
            }
        }

        /// <summary>
        /// Loads fonts for controls with optimized memory usage.
        /// </summary>
        /// <param name="controls">Array of controls to apply fonts to.</param>
        /// <param name="indexFont">Font family index.</param>
        /// <param name="fontSize">Font size.</param>
        public static void LoadControlsFont(Control[] controls, byte indexFont = 0, float fontSize = 12f)
        {
            if (controls == null || controls.Length == 0)
                return;

            try
            {
                if (_fontCollection.Families.Length <= indexFont)
                    return;

                using var installedFont = new Font(_fontCollection.Families[indexFont], fontSize);
                foreach (var control in controls)
                {
                    if (control != null && !control.IsDisposed)
                    {
                        control.Font = installedFont;
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to load control fonts: {ex.Message}");
            }
        }

        /// <summary>
        /// Disposes of the font collection to free memory.
        /// </summary>
        public void Dispose()
        {
            if (!_disposed)
            {
                _fontCollection?.Dispose();
                _disposed = true;
            }
        }
    }

}
