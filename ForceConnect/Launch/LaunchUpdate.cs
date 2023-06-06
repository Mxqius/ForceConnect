using Newtonsoft.Json;
using System;
using System.Diagnostics;
using System.IO;
using System.Net;
using System.Windows.Forms;

namespace ForceConnect.Launch
{
    internal class LaunchUpdate
    {
        public static string GetLatestVersionFromGitHub(string repositoryOwner, string repositoryName)
        {
            try
            {
                string apiUrl = $"https://api.github.com/repos/{repositoryOwner}/{repositoryName}/releases/latest";
                ServicePointManager.SecurityProtocol = SecurityProtocolType.Tls12;
                HttpWebRequest request = (HttpWebRequest)WebRequest.Create(apiUrl);
                request.UserAgent = "Mozilla/5.0";
                string responseJson;

                using (HttpWebResponse response = (HttpWebResponse)request.GetResponse())
                using (Stream stream = response.GetResponseStream())
                using (StreamReader reader = new StreamReader(stream))
                {
                    responseJson = reader.ReadToEnd();
                }

                dynamic release = JsonConvert.DeserializeObject(responseJson);
                string name = release.name;
                // Use name to get version of release
                string latestVersion = name.Split(' ')[1];
                return latestVersion;
            }
            catch
            {
                return "ERROR";
            }
        }
        public static string GetLastestVersionDownloadUrl(string repositoryOwner, string repositoryName)
        {
            try
            {
                string apiUrl = $"https://api.github.com/repos/{repositoryOwner}/{repositoryName}/releases/latest";
                ServicePointManager.SecurityProtocol = SecurityProtocolType.Tls12;
                HttpWebRequest request = (HttpWebRequest)WebRequest.Create(apiUrl);
                request.UserAgent = "Mozilla/5.0";
                string responseJson;

                using (HttpWebResponse response = (HttpWebResponse)request.GetResponse())
                using (Stream stream = response.GetResponseStream())
                using (StreamReader reader = new StreamReader(stream))
                {
                    responseJson = reader.ReadToEnd();
                }

                dynamic release = JsonConvert.DeserializeObject(responseJson);
                string download_url = release.assets[0].browser_download_url;
                return download_url;
            }
            catch
            {
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
        public static void DownloadUpdate(string downloadUrl, string savePath)
        {
            try
            {
                ServicePointManager.SecurityProtocol = SecurityProtocolType.Tls12;
                using (WebClient client = new WebClient())
                {
                    client.DownloadFile(downloadUrl, savePath);
                }
                // Open and install the downloaded file
                Process installProcess = Process.Start(savePath); // Assuming the downloaded file is an executable or installer

                // Wait for the installation process to complete
                installProcess.WaitForExit();
                // Remove the installation file
                File.Delete(savePath);
            }
            catch (IOException e)
            {
                MessageBox.Show("Error");
            }
        }
        public static Version getVersionApplication()
        {
            return Version.Parse(Application.ProductVersion);
        }
    }
}
