using System;
using System.IO;
using System.Text;

namespace ForceConnect.Services
{
    internal class Service
    {
        private static readonly string pathServices = Environment.CurrentDirectory + "\\Services\\services.txt";
        private static readonly string directory = Environment.CurrentDirectory + "\\Services";
        public static string GenerateUniqueID(int length)
        {
            const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
            Random random = new Random();
            StringBuilder sb = new StringBuilder(length);

            for (int i = 0; i < length; i++)
            {
                sb.Append(chars[random.Next(chars.Length)]);
            }

            return sb.ToString();
        }
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
                        writer.WriteLine($"{serviceParts[0]} {serviceParts[1]} {serviceParts[2]} {serviceParts[3]}");
                    }
                writer.WriteLine($"{GenerateUniqueID(6)} {name} {address1} {address2}");
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

        public static bool DeleteService(string idToDelete)
        {
            try
            {
                // Read the content of the file into an array of lines.
                string[] lines = File.ReadAllLines(pathServices);

                // Create a StringBuilder to build the new content.
                StringBuilder newContent = new StringBuilder();

                bool found = false;

                // Iterate through the lines and skip the line with the specified ID.
                foreach (string line in lines)
                {
                    string[] parts = line.Split(' '); // Assuming lines are CSV or have some delimiter.
                    if (parts.Length >= 1 && parts[0] == idToDelete)
                    {
                        found = true;
                        continue; // Skip this line.
                    }
                    newContent.AppendLine(line);
                }

                if (found)
                {
                    // Write the modified content back to the file.
                    File.WriteAllText(pathServices, newContent.ToString());
                    return true;
                }
                else
                {
                    return false; // ID not found in the file.
                }
            }
            catch (Exception ex)
            {
                Console.WriteLine("Error: " + ex.Message);
                return false;
            }
        }
    }
}
