using Microsoft.Win32;
using System.Reflection;

namespace ForceConnect.Startup
{
    internal class RegistryApplication
    {
        private static readonly string _appName = Assembly.GetEntryAssembly().GetName().Name;
        // Save data in the Registry     
        public static void SaveData(string key, string value)
        {
            RegistryKey registryKey = Registry.CurrentUser.CreateSubKey($"Software\\{_appName}");
            registryKey.SetValue(key, value);
            registryKey.Close();
        }

        // Retrieve data from the Registry
        public static string RetrieveData(string key)
        {
            string value = null;
            RegistryKey registryKey = Registry.CurrentUser.OpenSubKey($"Software\\{_appName}");
            if (registryKey != null)
            {
                value = registryKey.GetValue(key) as string;
                registryKey.Close();
            }
            return value;
        }

    }
}
