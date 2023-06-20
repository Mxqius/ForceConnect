using System;
using System.Drawing;
using System.Drawing.Text;
using System.IO;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace ForceConnect.Utility
{
    public class FontManager
    {
        [DllImport("gdi32.dll", EntryPoint = "AddFontResourceW", SetLastError = true)]
        public static extern int AddFontResource([In][MarshalAs(UnmanagedType.LPWStr)]
                                         string lpFileName);
        [DllImport("gdi32.dll", EntryPoint = "RemoveFontResourceW", SetLastError = true)]
        public static extern int RemoveFontResource([In][MarshalAs(UnmanagedType.LPWStr)]
                                            string lpFileName);

        private static string _fontDirectory = Application.StartupPath + "\\Fonts";
        private static PrivateFontCollection _fontCollection = new PrivateFontCollection();

        public static bool InstallFonts()
        {
            if (Directory.Exists(_fontDirectory))
            {
                string[] fontFiles = Directory.GetFiles(_fontDirectory, "*.ttf");

                if (fontFiles.Length < 1)
                    return false;

                foreach (string fontFile in fontFiles)
                {
                    InstallFont(fontFile);
                }

                if (_fontCollection.Families.Length > 0)
                    return true;
            }

            return false;
        }

        private static void InstallFont(string fontFile)
        {
            byte[] fontData = File.ReadAllBytes(fontFile);
            IntPtr fontPtr = Marshal.AllocCoTaskMem(fontData.Length);
            Marshal.Copy(fontData, 0, fontPtr, fontData.Length);
            AddFontResource(fontFile);
            _fontCollection.AddMemoryFont(fontPtr, fontData.Length);
            Marshal.FreeCoTaskMem(fontPtr);
        }
        public static void LoadControlsFont(Control[] controls, byte indexFont = 0, float fontSize = 12f)
        {
            Font installedFont = new Font(_fontCollection.Families[indexFont], fontSize);
            foreach (Control control in controls)
                control.Font = installedFont;
        }
    }

}
