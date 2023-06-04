using Newtonsoft.Json;
using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Net;
using System.Text;
using System.Threading.Tasks;

namespace ForceConnect.Launch
{
    internal class LaunchUpdate
    {
        public string GetLatestVersionFromGitHub(string repositoryOwner, string repositoryName)
        {
            try
            {
                string apiUrl = $"https://api.github.com/repos/{repositoryOwner}/{repositoryName}/releases/latest";
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
                string latestVersion = release.tag_name;
                return latestVersion;
            }
            catch
            {
                return "ERROR"; 
            }
        }
        public bool IsUpdateAvailable(string latestVersion, string currentVersion)
        {
            Version latest = new Version(latestVersion);
            Version current = new Version(currentVersion);

            return latest > current;
        }
        public void DownloadUpdate(string repositoryOwner, string repositoryName, string downloadUrl, string savePath)
        {
            using (WebClient client = new WebClient())
            {
                client.DownloadFile(downloadUrl, savePath);
            }
        }
    }
}
