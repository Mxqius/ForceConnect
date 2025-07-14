using Microsoft.Win32;
using System.Reflection;

namespace ForceConnect.Startup
{
    internal class LaunchProgram
    {
        private static readonly string _appName = Assembly.GetEntryAssembly().GetName().Name;
        public static bool IsAppSetToRunAtStartup()
        {
            RegistryKey rk = Registry.CurrentUser.OpenSubKey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", true);
            return rk.GetValue(_appName) != null;
        }
        public static void RemoveAppFromStartup()
        {
            RegistryKey rk = Registry.CurrentUser.OpenSubKey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", true);
            rk.DeleteValue(_appName, false);
        }

        public static void AddAppToStartup()
        {
            string appPath = Assembly.GetEntryAssembly().Location;
            RegistryKey rk = Registry.CurrentUser.OpenSubKey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", true);
            rk.SetValue(_appName, appPath);
        }

    }
}
