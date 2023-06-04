using System;
using System.IO;

namespace ForceConnect.Services
{
    internal class Service
    {
        private static readonly string pathServices = Environment.CurrentDirectory + "\\Services\\services.txt";
        private static readonly string directory = Environment.CurrentDirectory + "\\Services";
        public static void AddService(string name, string address1, string address2 = "")
        {
            string[] services = GetServices();
            if (!Directory.Exists(directory))
                Directory.CreateDirectory(directory);

            using (StreamWriter writer = new StreamWriter(pathServices))
            {
                if (services != null)
                    foreach (string service in services)
                    {
                        string[] serviceParts = service.Split(' ');
                        writer.WriteLine($"{serviceParts[0]} {serviceParts[1]} {serviceParts[2]}");
                    }
                writer.WriteLine($"{name} {address1} {address2}");
            }
        }
        public static string[] GetServices()
        {
            if (File.Exists(pathServices))
            {
                string[] results = File.ReadAllLines(pathServices);
                return results;
            }
            else
                return null;
        }
    }
}
