using System;
using System.Diagnostics;
using System.Threading.Tasks;
using System.Windows.Forms;
using Label = System.Windows.Forms.Label;

namespace ForceConnect.Network
{
    /// <summary>
    /// Monitors network traffic for a specific network interface.
    /// </summary>
    public class NetworkMonitor : IDisposable
    {
        private readonly string _networkInterfaceInstanceName;
        private readonly PerformanceCounter _downloadCounter;
        private readonly PerformanceCounter _uploadCounter;
        private bool _disposed = false;

        /// <summary>
        /// Initializes a new instance of the NetworkMonitor class.
        /// </summary>
        /// <param name="networkInterfaceInstanceName">The name of the network interface to monitor.</param>
        /// <exception cref="ArgumentNullException">Thrown when networkInterfaceInstanceName is null or empty.</exception>
        public NetworkMonitor(string networkInterfaceInstanceName)
        {
            if (string.IsNullOrWhiteSpace(networkInterfaceInstanceName))
                throw new ArgumentNullException(nameof(networkInterfaceInstanceName), "Network interface name cannot be null or empty.");

            _networkInterfaceInstanceName = networkInterfaceInstanceName;

            try
            {
                _downloadCounter = new PerformanceCounter("Network Interface", "Bytes Received/sec", networkInterfaceInstanceName);
                _uploadCounter = new PerformanceCounter("Network Interface", "Bytes Sent/sec", networkInterfaceInstanceName);
            }
            catch (Exception ex)
            {
                throw new InvalidOperationException($"Failed to initialize performance counters for {networkInterfaceInstanceName}: {ex.Message}", ex);
            }
        }

        /// <summary>
        /// Starts monitoring network traffic with optimized performance and resource management.
        /// </summary>
        /// <param name="labelDownloadTraffic">Label to display download traffic.</param>
        /// <param name="labelUploadTraffic">Label to display upload traffic.</param>
        /// <param name="updateIntervalMs">Update interval in milliseconds (default: 1000).</param>
        /// <exception cref="ArgumentNullException">Thrown when labels are null.</exception>
        public async Task StartMonitoringAsync(Label labelDownloadTraffic, Label labelUploadTraffic, int updateIntervalMs = 1000)
        {
            if (labelDownloadTraffic == null)
                throw new ArgumentNullException(nameof(labelDownloadTraffic));
            
            if (labelUploadTraffic == null)
                throw new ArgumentNullException(nameof(labelUploadTraffic));

            if (updateIntervalMs < 100)
                updateIntervalMs = 100; // Minimum 100ms for performance

            try
            {
                // Pre-allocate string builders for better performance
                var downloadTextBuilder = new System.Text.StringBuilder(20);
                var uploadTextBuilder = new System.Text.StringBuilder(20);

                while (!_disposed)
                {
                    try
                    {
                        var downloadSpeed = _downloadCounter.NextValue() / 1024; // Convert to KB/s
                        var uploadSpeed = _uploadCounter.NextValue() / 1024; // Convert to KB/s

                        // Use StringBuilder for efficient string building
                        downloadTextBuilder.Clear();
                        downloadTextBuilder.AppendFormat("{0:F2} KB/s", downloadSpeed);
                        
                        uploadTextBuilder.Clear();
                        uploadTextBuilder.AppendFormat("{0:F2} KB/s", uploadSpeed);

                        // Update UI on main thread with minimal overhead
                        if (!_disposed)
                        {
                            labelDownloadTraffic.Text = downloadTextBuilder.ToString();
                            labelUploadTraffic.Text = uploadTextBuilder.ToString();
                        }

                        await Task.Delay(updateIntervalMs);
                    }
                    catch (InvalidOperationException)
                    {
                        // Performance counter may be temporarily unavailable
                        await Task.Delay(100);
                        continue;
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Network monitoring error: {ex.Message}");
                if (!_disposed)
                {
                    MessageBox.Show($"Network monitoring error: {ex.Message}", "Network Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                }
            }
        }

        /// <summary>
        /// Synchronous version of StartMonitoring for backward compatibility.
        /// </summary>
        /// <param name="labelDownloadTraffic">Label to display download traffic.</param>
        /// <param name="labelUploadTraffic">Label to display upload traffic.</param>
        public void StartMonitoring(Label labelDownloadTraffic, Label labelUploadTraffic)
        {
            _ = StartMonitoringAsync(labelDownloadTraffic, labelUploadTraffic);
        }

        /// <summary>
        /// Disposes of the performance counters.
        /// </summary>
        public void Dispose()
        {
            if (!_disposed)
            {
                _downloadCounter?.Dispose();
                _uploadCounter?.Dispose();
                _disposed = true;
            }
        }
    }
}
