using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Threading.Tasks;

namespace ForceConnect.Utilities
{
    /// <summary>
    /// Provides memory optimization utilities to prevent memory leaks and manage resources efficiently.
    /// </summary>
    public static class MemoryOptimizer
    {
        private static readonly List<WeakReference> _trackedObjects = new List<WeakReference>();
        private static readonly object _lockObject = new object();

        /// <summary>
        /// Tracks an object for memory leak detection.
        /// </summary>
        /// <param name="obj">The object to track.</param>
        public static void TrackObject(object obj)
        {
            if (obj == null) return;

            lock (_lockObject)
            {
                _trackedObjects.Add(new WeakReference(obj));
                CleanupDeadReferences();
            }
        }

        /// <summary>
        /// Cleans up dead references from the tracking list.
        /// </summary>
        private static void CleanupDeadReferences()
        {
            lock (_lockObject)
            {
                _trackedObjects.RemoveAll(wr => !wr.IsAlive);
            }
        }

        /// <summary>
        /// Gets the count of tracked objects.
        /// </summary>
        /// <returns>Number of tracked objects.</returns>
        public static int GetTrackedObjectCount()
        {
            lock (_lockObject)
            {
                CleanupDeadReferences();
                return _trackedObjects.Count;
            }
        }

        /// <summary>
        /// Forces garbage collection and memory cleanup.
        /// </summary>
        public static void ForceGarbageCollection()
        {
            try
            {
                GC.Collect();
                GC.WaitForPendingFinalizers();
                GC.Collect();
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Garbage collection failed: {ex.Message}");
            }
        }

        /// <summary>
        /// Gets current memory statistics.
        /// </summary>
        /// <returns>Memory statistics object.</returns>
        public static MemoryStatistics GetMemoryStatistics()
        {
            var process = Process.GetCurrentProcess();
            process.Refresh();

            return new MemoryStatistics
            {
                WorkingSetMB = Math.Round(process.WorkingSet64 / (1024.0 * 1024.0), 2),
                PrivateMemoryMB = Math.Round(process.PrivateMemorySize64 / (1024.0 * 1024.0), 2),
                VirtualMemoryMB = Math.Round(process.VirtualMemorySize64 / (1024.0 * 1024.0), 2),
                PeakWorkingSetMB = Math.Round(process.PeakWorkingSet64 / (1024.0 * 1024.0), 2),
                GCTotalMemoryMB = Math.Round(GC.GetTotalMemory(false) / (1024.0 * 1024.0), 2),
                TrackedObjectsCount = GetTrackedObjectCount()
            };
        }

        /// <summary>
        /// Monitors memory usage and performs cleanup when thresholds are exceeded.
        /// </summary>
        /// <param name="memoryThresholdMB">Memory threshold in MB to trigger cleanup.</param>
        /// <param name="intervalMs">Monitoring interval in milliseconds.</param>
        /// <returns>Task representing the monitoring operation.</returns>
        public static async Task StartMemoryMonitoringAsync(double memoryThresholdMB = 500, int intervalMs = 10000)
        {
            while (true)
            {
                try
                {
                    var stats = GetMemoryStatistics();
                    
                    // Log memory usage
                    System.Diagnostics.Debug.WriteLine($"Memory Usage: {stats.WorkingSetMB}MB, Tracked Objects: {stats.TrackedObjectsCount}");

                    // Perform cleanup if memory usage exceeds threshold
                    if (stats.WorkingSetMB > memoryThresholdMB)
                    {
                        System.Diagnostics.Debug.WriteLine($"Memory threshold exceeded ({stats.WorkingSetMB}MB > {memoryThresholdMB}MB). Performing cleanup...");
                        ForceGarbageCollection();
                        
                        // Log post-cleanup statistics
                        var postStats = GetMemoryStatistics();
                        System.Diagnostics.Debug.WriteLine($"Post-cleanup Memory: {postStats.WorkingSetMB}MB");
                    }

                    await Task.Delay(intervalMs);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Memory monitoring error: {ex.Message}");
                    await Task.Delay(intervalMs);
                }
            }
        }

        /// <summary>
        /// Disposes of multiple objects safely.
        /// </summary>
        /// <param name="disposables">Array of disposable objects.</param>
        public static void DisposeObjects(params IDisposable[] disposables)
        {
            if (disposables == null) return;

            foreach (var disposable in disposables)
            {
                try
                {
                    disposable?.Dispose();
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Failed to dispose object: {ex.Message}");
                }
            }
        }

        /// <summary>
        /// Clears collections and arrays to free memory.
        /// </summary>
        /// <typeparam name="T">Type of the collection.</typeparam>
        /// <param name="collections">Collections to clear.</param>
        public static void ClearCollections<T>(params ICollection<T>[] collections)
        {
            if (collections == null) return;

            foreach (var collection in collections)
            {
                try
                {
                    collection?.Clear();
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Failed to clear collection: {ex.Message}");
                }
            }
        }
    }

    /// <summary>
    /// Contains memory statistics data.
    /// </summary>
    public class MemoryStatistics
    {
        public double WorkingSetMB { get; set; }
        public double PrivateMemoryMB { get; set; }
        public double VirtualMemoryMB { get; set; }
        public double PeakWorkingSetMB { get; set; }
        public double GCTotalMemoryMB { get; set; }
        public int TrackedObjectsCount { get; set; }

        public override string ToString()
        {
            return $"Working Set: {WorkingSetMB}MB, Private: {PrivateMemoryMB}MB, Virtual: {VirtualMemoryMB}MB, Tracked: {TrackedObjectsCount}";
        }
    }
} 