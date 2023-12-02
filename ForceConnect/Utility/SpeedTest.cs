using System;
using System.Diagnostics;
using System.Net;
using System.Threading.Tasks;

namespace ForceConnect.Utility
{
    public class SpeedTest
    {
        public async static Task<double> MeasureDownloadSpeedAsync(string downloadUrl, int decimalPlaces = 2)
        {
            using (var webClient = new WebClient())
            {
                var stopwatch = Stopwatch.StartNew();
                await webClient.DownloadDataTaskAsync(downloadUrl);
                stopwatch.Stop();

                var fileSizeInBytes = webClient.ResponseHeaders["Content-Length"];
                var fileSizeInMegaBytes = Convert.ToDouble(fileSizeInBytes) / (1024 * 1024);
                var elapsedSeconds = stopwatch.Elapsed.TotalSeconds;

                var downloadSpeed = fileSizeInMegaBytes / elapsedSeconds;
                return Math.Round(downloadSpeed, decimalPlaces);
            }
        }

        public async static Task<double> MeasureUploadSpeedAsync(string uploadUrl, int decimalPlaces = 2)
        {
            using (var webClient = new WebClient())
            {
                var fileSizeInBytes = 1024 * 1024; // 1 MB
                var fileData = new byte[fileSizeInBytes];

                var stopwatch = Stopwatch.StartNew();
                await webClient.UploadDataTaskAsync(uploadUrl, "POST", fileData);
                stopwatch.Stop();

                var fileSizeInMegaBytes = Convert.ToDouble(fileSizeInBytes) / (1024 * 1024);
                var elapsedSeconds = stopwatch.Elapsed.TotalSeconds;

                var uploadSpeed = fileSizeInMegaBytes / elapsedSeconds;
                return Math.Round(uploadSpeed, decimalPlaces);
            }
        }
    }
}
