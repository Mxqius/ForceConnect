using System;
using System.IO;
using System.Text;

namespace ForceConnect.Services
{
    internal class Service
    {
        private static readonly string pathServices = Environment.CurrentDirectory + "\\Services\\services.txt";
        private static readonly string directory = Environment.CurrentDirectory + "\\Services";
            /// <summary>
    /// Generates a unique identifier of the specified length.
    /// </summary>
    /// <param name="length">The length of the unique identifier.</param>
    /// <returns>A unique identifier string.</returns>
    /// <exception cref="ArgumentOutOfRangeException">Thrown when length is less than or equal to 0.</exception>
    public static string GenerateUniqueID(int length)
    {
        if (length <= 0)
            throw new ArgumentOutOfRangeException(nameof(length), "Length must be greater than 0.");

        const string chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
        var random = new Random();
        var sb = new StringBuilder(length);

        for (int i = 0; i < length; i++)
        {
            sb.Append(chars[random.Next(chars.Length)]);
        }

        return sb.ToString();
    }
        /// <summary>
        /// Adds a new service to the services file.
        /// </summary>
        /// <param name="name">The name of the service.</param>
        /// <param name="address1">The primary DNS address.</param>
        /// <param name="address2">The secondary DNS address (optional).</param>
        /// <exception cref="ArgumentNullException">Thrown when name or address1 is null or empty.</exception>
        public static void AddService(string name, string address1, string address2 = "")
        {
            if (string.IsNullOrWhiteSpace(name))
                throw new ArgumentNullException(nameof(name), "Service name cannot be null or empty.");
            
            if (string.IsNullOrWhiteSpace(address1))
                throw new ArgumentNullException(nameof(address1), "Primary DNS address cannot be null or empty.");

            try
            {
                string[] services = GetServices();
                if (!Directory.Exists(directory))
                    Directory.CreateDirectory(directory);

                using (var writer = new StreamWriter(pathServices))
                {
                    if (services != null)
                    {
                        foreach (string service in services)
                        {
                            string[] serviceParts = service.Split(' ');
                            if (serviceParts.Length >= 4)
                            {
                                writer.WriteLine($"{serviceParts[0]} {serviceParts[1]} {serviceParts[2]} {serviceParts[3]}");
                            }
                        }
                    }
                    writer.WriteLine($"{GenerateUniqueID(6)} {name} {address1} {address2}");
                }
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to add service '{name}': {ex.Message}", ex);
            }
        }
        /// <summary>
        /// Gets all services from the services file.
        /// </summary>
        /// <returns>Array of service strings, or null if file doesn't exist.</returns>
        public static string[] GetServices()
        {
            if (File.Exists(pathServices))
            {
                return File.ReadAllLines(pathServices);
            }
            return null;
        }

        /// <summary>
        /// Deletes a service by its unique ID.
        /// </summary>
        /// <param name="idToDelete">The unique ID of the service to delete.</param>
        /// <returns>True if the service was found and deleted, false otherwise.</returns>
        /// <exception cref="ArgumentNullException">Thrown when idToDelete is null or empty.</exception>
        public static bool DeleteService(string idToDelete)
        {
            if (string.IsNullOrWhiteSpace(idToDelete))
                throw new ArgumentNullException(nameof(idToDelete), "Service ID cannot be null or empty.");

            try
            {
                if (!File.Exists(pathServices))
                    return false;

                string[] lines = File.ReadAllLines(pathServices);
                var newContent = new StringBuilder();
                bool found = false;

                foreach (string line in lines)
                {
                    string[] parts = line.Split(' ');
                    if (parts.Length >= 1 && parts[0] == idToDelete)
                    {
                        found = true;
                        continue; // Skip this line
                    }
                    newContent.AppendLine(line);
                }

                if (found)
                {
                    File.WriteAllText(pathServices, newContent.ToString());
                    return true;
                }
                
                return false; // ID not found in the file
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to delete service '{idToDelete}': {ex.Message}", ex);
            }
        }
    }
}
