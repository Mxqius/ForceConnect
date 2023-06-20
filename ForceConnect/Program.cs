using System;
using System.Collections.Generic;
using System.Linq;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Runtime.InteropServices;
using ForceConnect.Utility;

namespace ForceConnect
{
    internal static class Program
    {
        [DllImport("User32.dll")]
        public static extern bool SetProcessDPIAware();
        /// <summary>
        /// The main entry point for the application.
        /// </summary>
        [STAThread]
        static void Main()
        {
            // Set the DPI awareness for the application
            SetProcessDPIAware();
            FontManager.InstallFonts();
            Application.EnableVisualStyles();
            Application.SetCompatibleTextRenderingDefault(false);
            try
            {
                Application.Run(new frm_main());
            }
            catch (Exception ex)
            {
                new frm_messageBox()
                {
                    MessageText = "An error occurred during program execution",
                    MessageCaption = "Error",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Error
                }.ShowMessage();
            }
        }
    }
}
