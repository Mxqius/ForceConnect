using System;
using System.Drawing.Text;
using System.IO;
using System.Runtime.InteropServices;
using System.Windows.Forms;

namespace ForceConnect.Utility
{
    public class FontManager
    {
        private static string _fontDirectory = Application.StartupPath + "\\Fonts";
        private static PrivateFontCollection _fontCollection = new PrivateFontCollection();

        public static bool InstallFonts()
        {
            if (Directory.Exists(_fontDirectory))
            {
                string[] fontFiles = Directory.GetFiles(_fontDirectory, "*.ttf");

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
            _fontCollection.AddMemoryFont(fontPtr, fontData.Length);
            Marshal.FreeCoTaskMem(fontPtr);
        }
    }

}
