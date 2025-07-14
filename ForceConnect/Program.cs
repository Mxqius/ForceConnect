using System;
using System.Threading.Tasks;
using System.Windows.Forms;
using System.Runtime.InteropServices;
using ForceConnect.Utilities;

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
            try
            {
                // Set the DPI awareness for the application
                SetProcessDPIAware();

                // Initialize localization system
                LocalizationManager.Initialize();

                // Install fonts in a separate thread
                Task.Run(() => 
                {
                    FontManager.InstallFonts();
                    PersianFontManager.InstallPersianFonts();
                });

                Application.EnableVisualStyles();
                Application.SetCompatibleTextRenderingDefault(false);

                Application.Run(new ForceConnect.Forms.frm_main());
            }
            catch (Exception ex)
            {
                // Log the exception details for debugging
                System.Diagnostics.Debug.WriteLine($"Application startup error: {ex.Message}");
                
                try
                {
                    new ForceConnect.Forms.frm_messageBox()
                    {
                        MessageText = LocalizationManager.GetString("failedToExecuteCommand", "An error occurred during program execution") + $": {ex.Message}",
                        MessageCaption = LocalizationManager.GetString("error", "Error"),
                        MessageButtons = ForceConnect.Forms.frm_messageBox.Buttons.OK,
                        MessageIcon = ForceConnect.Forms.frm_messageBox.Icon.Error
                    }.ShowMessage();
                }
                catch
                {
                    // Fallback to console if message box fails
                    Console.WriteLine($"Critical error: {ex.Message}");
                }
            }
        }
    }
}
