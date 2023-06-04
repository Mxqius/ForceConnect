using Microsoft.Win32;
using System.Reflection;

namespace ForceConnect.Launch
{
    internal class LaunchProgram
    {
        public static bool IsAppSetToRunAtStartup()
        {
            string appName = Assembly.GetEntryAssembly().GetName().Name;
            RegistryKey rk = Registry.CurrentUser.OpenSubKey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", true);
            return rk.GetValue(appName) != null;
        }
        public static void RemoveAppFromStartup()
        {
            string appName = Assembly.GetEntryAssembly().GetName().Name;
            RegistryKey rk = Registry.CurrentUser.OpenSubKey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", true);
            rk.DeleteValue(appName, false);
        }

        public static void AddAppToStartup()
        {
            string appName = Assembly.GetEntryAssembly().GetName().Name;
            string appPath = Assembly.GetEntryAssembly().Location;
            RegistryKey rk = Registry.CurrentUser.OpenSubKey("SOFTWARE\\Microsoft\\Windows\\CurrentVersion\\Run", true);
            rk.SetValue(appName, appPath);
        }

    }
}
