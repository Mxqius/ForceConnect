using System;
using System.Diagnostics;
using System.Net;
using System.Threading.Tasks;

namespace ForceConnect.Utilities
{
    public class SpeedTest
    {
        /// <summary>
        /// Measures the download speed from a specified URL with optimized performance.
        /// </summary>
        /// <param name="downloadUrl">The URL to download from.</param>
        /// <param name="decimalPlaces">Number of decimal places for the result.</param>
        /// <param name="timeoutSeconds">Timeout in seconds (default: 30).</param>
        /// <returns>The download speed in MB/s.</returns>
        /// <exception cref="ArgumentNullException">Thrown when downloadUrl is null or empty.</exception>
        public static async Task<double> MeasureDownloadSpeedAsync(string downloadUrl, int decimalPlaces = 2, int timeoutSeconds = 30)
        {
            if (string.IsNullOrWhiteSpace(downloadUrl))
                throw new ArgumentNullException(nameof(downloadUrl), "Download URL cannot be null or empty.");

            using var webClient = new WebClient();
            using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(timeoutSeconds));
            
            try
            {
                // Configure WebClient for better performance
                webClient.Timeout = timeoutSeconds * 1000;
                webClient.Headers.Add("User-Agent", "ForceConnect/1.0");

                var stopwatch = Stopwatch.StartNew();
                var downloadTask = webClient.DownloadDataTaskAsync(downloadUrl);
                
                // Use cancellation token for timeout
                await downloadTask.WaitAsync(cts.Token);
                stopwatch.Stop();

                var fileSizeInBytes = webClient.ResponseHeaders["Content-Length"];
                if (string.IsNullOrEmpty(fileSizeInBytes))
                    throw new InvalidOperationException("Could not determine file size from response headers.");

                if (!long.TryParse(fileSizeInBytes, out var fileSize))
                    throw new InvalidOperationException("Invalid file size in response headers.");

                var fileSizeInMegaBytes = fileSize / (1024.0 * 1024.0);
                var elapsedSeconds = stopwatch.Elapsed.TotalSeconds;

                if (elapsedSeconds <= 0)
                    return 0;

                var downloadSpeed = fileSizeInMegaBytes / elapsedSeconds;
                return Math.Round(downloadSpeed, decimalPlaces);
            }
            catch (OperationCanceledException)
            {
                throw new TimeoutException($"Download timed out after {timeoutSeconds} seconds.");
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Download speed measurement failed: {ex.Message}");
                throw;
            }
        }

        /// <summary>
        /// Measures the upload speed to a specified URL with optimized performance.
        /// </summary>
        /// <param name="uploadUrl">The URL to upload to.</param>
        /// <param name="decimalPlaces">Number of decimal places for the result.</param>
        /// <param name="timeoutSeconds">Timeout in seconds (default: 30).</param>
        /// <returns>The upload speed in MB/s.</returns>
        /// <exception cref="ArgumentNullException">Thrown when uploadUrl is null or empty.</exception>
        public static async Task<double> MeasureUploadSpeedAsync(string uploadUrl, int decimalPlaces = 2, int timeoutSeconds = 30)
        {
            if (string.IsNullOrWhiteSpace(uploadUrl))
                throw new ArgumentNullException(nameof(uploadUrl), "Upload URL cannot be null or empty.");

            using var webClient = new WebClient();
            using var cts = new CancellationTokenSource(TimeSpan.FromSeconds(timeoutSeconds));
            
            try
            {
                // Configure WebClient for better performance
                webClient.Timeout = timeoutSeconds * 1000;
                webClient.Headers.Add("User-Agent", "ForceConnect/1.0");

                // Use smaller test file for faster measurement
                var fileSizeInBytes = 512 * 1024; // 512 KB for faster testing
                var fileData = new byte[fileSizeInBytes];

                var stopwatch = Stopwatch.StartNew();
                var uploadTask = webClient.UploadDataTaskAsync(uploadUrl, "POST", fileData);
                
                // Use cancellation token for timeout
                await uploadTask.WaitAsync(cts.Token);
                stopwatch.Stop();

                var fileSizeInMegaBytes = fileSizeInBytes / (1024.0 * 1024.0);
                var elapsedSeconds = stopwatch.Elapsed.TotalSeconds;

                if (elapsedSeconds <= 0)
                    return 0;

                var uploadSpeed = fileSizeInMegaBytes / elapsedSeconds;
                return Math.Round(uploadSpeed, decimalPlaces);
            }
            catch (OperationCanceledException)
            {
                throw new TimeoutException($"Upload timed out after {timeoutSeconds} seconds.");
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Upload speed measurement failed: {ex.Message}");
                throw;
            }
        }
    }
}
