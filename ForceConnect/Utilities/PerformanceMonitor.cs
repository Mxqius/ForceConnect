using System;
using System.Diagnostics;
using System.Threading.Tasks;

namespace ForceConnect.Utilities
{
    /// <summary>
    /// Monitors application performance metrics including memory and CPU usage.
    /// </summary>
    public class PerformanceMonitor : IDisposable
    {
        private readonly Process _currentProcess;
        private readonly PerformanceCounter _cpuCounter;
        private bool _disposed = false;

        /// <summary>
        /// Initializes a new instance of the PerformanceMonitor class.
        /// </summary>
        public PerformanceMonitor()
        {
            _currentProcess = Process.GetCurrentProcess();
            _cpuCounter = new PerformanceCounter("Processor", "% Processor Time", "_Total");
        }

        /// <summary>
        /// Gets the current memory usage in MB.
        /// </summary>
        /// <returns>Memory usage in MB.</returns>
        public double GetMemoryUsageMB()
        {
            try
            {
                _currentProcess.Refresh();
                return Math.Round(_currentProcess.WorkingSet64 / (1024.0 * 1024.0), 2);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get memory usage: {ex.Message}");
                return 0;
            }
        }

        /// <summary>
        /// Gets the current CPU usage percentage.
        /// </summary>
        /// <returns>CPU usage percentage.</returns>
        public double GetCpuUsagePercentage()
        {
            try
            {
                return Math.Round(_cpuCounter.NextValue(), 2);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get CPU usage: {ex.Message}");
                return 0;
            }
        }

        /// <summary>
        /// Gets the current thread count.
        /// </summary>
        /// <returns>Number of active threads.</returns>
        public int GetThreadCount()
        {
            try
            {
                _currentProcess.Refresh();
                return _currentProcess.Threads.Count;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get thread count: {ex.Message}");
                return 0;
            }
        }

        /// <summary>
        /// Gets comprehensive performance metrics.
        /// </summary>
        /// <returns>Performance metrics object.</returns>
        public PerformanceMetrics GetPerformanceMetrics()
        {
            return new PerformanceMetrics
            {
                MemoryUsageMB = GetMemoryUsageMB(),
                CpuUsagePercentage = GetCpuUsagePercentage(),
                ThreadCount = GetThreadCount(),
                ProcessTime = _currentProcess.TotalProcessorTime,
                Timestamp = DateTime.Now
            };
        }

        /// <summary>
        /// Monitors performance continuously and logs metrics.
        /// </summary>
        /// <param name="intervalMs">Monitoring interval in milliseconds.</param>
        /// <param name="callback">Callback to receive performance updates.</param>
        /// <returns>Task representing the monitoring operation.</returns>
        public async Task StartMonitoringAsync(int intervalMs = 5000, Action<PerformanceMetrics> callback = null)
        {
            while (!_disposed)
            {
                try
                {
                    var metrics = GetPerformanceMetrics();
                    callback?.Invoke(metrics);

                    // Log performance warnings
                    if (metrics.MemoryUsageMB > 500) // 500MB threshold
                    {
                        System.Diagnostics.Debug.WriteLine($"High memory usage detected: {metrics.MemoryUsageMB}MB");
                    }

                    if (metrics.CpuUsagePercentage > 80) // 80% threshold
                    {
                        System.Diagnostics.Debug.WriteLine($"High CPU usage detected: {metrics.CpuUsagePercentage}%");
                    }

                    await Task.Delay(intervalMs);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Performance monitoring error: {ex.Message}");
                    await Task.Delay(intervalMs);
                }
            }
        }

        /// <summary>
        /// Disposes of the performance monitor and frees resources.
        /// </summary>
        public void Dispose()
        {
            if (!_disposed)
            {
                _cpuCounter?.Dispose();
                _disposed = true;
            }
        }
    }

    /// <summary>
    /// Contains performance metrics data.
    /// </summary>
    public class PerformanceMetrics
    {
        public double MemoryUsageMB { get; set; }
        public double CpuUsagePercentage { get; set; }
        public int ThreadCount { get; set; }
        public TimeSpan ProcessTime { get; set; }
        public DateTime Timestamp { get; set; }

        public override string ToString()
        {
            return $"Memory: {MemoryUsageMB}MB, CPU: {CpuUsagePercentage}%, Threads: {ThreadCount}, Time: {Timestamp:HH:mm:ss}";
        }
    }
} 