using System;
using System.Collections.Generic;
using System.Linq;
using System.IO;
using System.Text;
using System.Threading.Tasks;
using System.Reflection;

namespace ForceConnect.Services
{
    internal class Service
    {
        private static readonly string pathServices = Environment.CurrentDirectory + "\\Services\\services.txt";
        private static readonly string directory = Environment.CurrentDirectory + "\\Services";
        public static void AddService(string name, string address1, string address2 = "")
        {
            if (!Directory.Exists(directory))
                Directory.CreateDirectory(directory);
            using (StreamWriter writer = new StreamWriter(pathServices))
            {
                writer.WriteLine($"{name} {address1} {address2}");
            }
        }
        public static string[] GetServices()
        {

            string[] results = File.ReadAllLines(pathServices);
            return results;
        }
    }
}
