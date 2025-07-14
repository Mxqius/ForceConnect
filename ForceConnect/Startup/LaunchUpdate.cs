using Newtonsoft.Json;
using System;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Windows.Forms;

namespace ForceConnect.Startup
{
    internal class LaunchUpdate
    {
        /// <summary>
        /// Gets the latest version from GitHub releases.
        /// </summary>
        /// <param name="repositoryOwner">The repository owner.</param>
        /// <param name="repositoryName">The repository name.</param>
        /// <returns>The latest version string, or "ERROR" if failed.</returns>
        /// <exception cref="ArgumentNullException">Thrown when parameters are null or empty.</exception>
        public static async Task<string> GetLatestVersionFromGitHubAsync(string repositoryOwner, string repositoryName)
        {
            if (string.IsNullOrWhiteSpace(repositoryOwner))
                throw new ArgumentNullException(nameof(repositoryOwner), "Repository owner cannot be null or empty.");
            
            if (string.IsNullOrWhiteSpace(repositoryName))
                throw new ArgumentNullException(nameof(repositoryName), "Repository name cannot be null or empty.");

            try
            {
                var apiUrl = $"https://api.github.com/repos/{repositoryOwner}/{repositoryName}/releases/latest";
                ServicePointManager.SecurityProtocol = SecurityProtocolType.Tls12;
                
                using var request = (HttpWebRequest)WebRequest.Create(apiUrl);
                request.UserAgent = "Mozilla/5.0";
                request.Timeout = 10000; // 10 seconds timeout

                using var response = (HttpWebResponse)await request.GetResponseAsync();
                using var stream = response.GetResponseStream();
                using var reader = new StreamReader(stream);
                var responseJson = await reader.ReadToEndAsync();

                dynamic release = JsonConvert.DeserializeObject(responseJson);
                string name = release.name;
                var versionParts = name.Split(' ');
                
                return versionParts.Length > 1 ? versionParts[1] : "ERROR";
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get latest version: {ex.Message}");
                return "ERROR";
            }
        }
        /// <summary>
        /// Gets the download URL for the latest version from GitHub releases.
        /// </summary>
        /// <param name="repositoryOwner">The repository owner.</param>
        /// <param name="repositoryName">The repository name.</param>
        /// <returns>The download URL, or "ERROR" if failed.</returns>
        /// <exception cref="ArgumentNullException">Thrown when parameters are null or empty.</exception>
        public static async Task<string> GetLatestVersionDownloadUrlAsync(string repositoryOwner, string repositoryName)
        {
            if (string.IsNullOrWhiteSpace(repositoryOwner))
                throw new ArgumentNullException(nameof(repositoryOwner), "Repository owner cannot be null or empty.");
            
            if (string.IsNullOrWhiteSpace(repositoryName))
                throw new ArgumentNullException(nameof(repositoryName), "Repository name cannot be null or empty.");

            try
            {
                var apiUrl = $"https://api.github.com/repos/{repositoryOwner}/{repositoryName}/releases/latest";
                ServicePointManager.SecurityProtocol = SecurityProtocolType.Tls12;
                
                using var request = (HttpWebRequest)WebRequest.Create(apiUrl);
                request.UserAgent = "Mozilla/5.0";
                request.Timeout = 10000; // 10 seconds timeout

                using var response = (HttpWebResponse)await request.GetResponseAsync();
                using var stream = response.GetResponseStream();
                using var reader = new StreamReader(stream);
                var responseJson = await reader.ReadToEndAsync();

                dynamic release = JsonConvert.DeserializeObject(responseJson);
                string downloadUrl = release.assets[0].browser_download_url;
                return downloadUrl;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get download URL: {ex.Message}");
                return "ERROR";
            }
        }
        public static bool IsUpdateAvailable(string latestVersion, string currentVersion)
        {
            try
            {
                Version latest = new Version(latestVersion);
                Version current = new Version(currentVersion);

                return latest > current;
            }
            catch
            {
                return false;
            }
        }
        /// <summary>
        /// Downloads and installs an update from the specified URL.
        /// </summary>
        /// <param name="downloadUrl">The URL to download the update from.</param>
        /// <param name="savePath">The path where to save the downloaded file.</param>
        /// <exception cref="ArgumentNullException">Thrown when parameters are null or empty.</exception>
        public static async Task DownloadUpdateAsync(string downloadUrl, string savePath)
        {
            if (string.IsNullOrWhiteSpace(downloadUrl))
                throw new ArgumentNullException(nameof(downloadUrl), "Download URL cannot be null or empty.");
            
            if (string.IsNullOrWhiteSpace(savePath))
                throw new ArgumentNullException(nameof(savePath), "Save path cannot be null or empty.");

            try
            {
                ServicePointManager.SecurityProtocol = SecurityProtocolType.Tls12;
                using var client = new WebClient();
                await client.DownloadFileTaskAsync(downloadUrl, savePath);

                // Open and install the downloaded file
                var installProcess = Process.Start(savePath);
                if (installProcess != null)
                {
                    await installProcess.WaitForExitAsync();
                    // Remove the installation file
                    if (File.Exists(savePath))
                    {
                        File.Delete(savePath);
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to download update: {ex.Message}");
                MessageBox.Show($"Failed to download update: {ex.Message}", "Update Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                throw;
            }
        }
        public static Version getVersionApplication()
        {
            return Version.Parse(Application.ProductVersion);
        }
    }
}
