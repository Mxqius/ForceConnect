using System.Net.NetworkInformation;

namespace ForceConnect.API
{
    /// <summary>
    /// Provides latency measurement functionality.
    /// </summary>
    public class Latency
    {
        /// <summary>
        /// Measures the latency to a specified IP address.
        /// </summary>
        /// <param name="ipAddress">The IP address to ping.</param>
        /// <returns>The round-trip time in milliseconds, or -1 if failed.</returns>
        /// <exception cref="ArgumentNullException">Thrown when ipAddress is null or empty.</exception>
        public static async Task<long> MeasureLatencyAsync(string ipAddress)
        {
            if (string.IsNullOrWhiteSpace(ipAddress))
                throw new ArgumentNullException(nameof(ipAddress), "IP address cannot be null or empty.");

            try
            {
                using var pingSender = new Ping();
                var reply = await pingSender.SendPingAsync(ipAddress, 5000); // 5 second timeout

                return reply.Status == IPStatus.Success ? reply.RoundtripTime : -1;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to measure latency for {ipAddress}: {ex.Message}");
                return -1;
            }
        }

        /// <summary>
        /// Measures the latency to a specified IP address (synchronous version).
        /// </summary>
        /// <param name="ipAddress">The IP address to ping.</param>
        /// <returns>The round-trip time in milliseconds, or -1 if failed.</returns>
        /// <exception cref="ArgumentNullException">Thrown when ipAddress is null or empty.</exception>
        public static long MeasureLatency(string ipAddress)
        {
            if (string.IsNullOrWhiteSpace(ipAddress))
                throw new ArgumentNullException(nameof(ipAddress), "IP address cannot be null or empty.");

            try
            {
                using var pingSender = new Ping();
                var reply = pingSender.Send(ipAddress, 5000); // 5 second timeout

                return reply.Status == IPStatus.Success ? reply.RoundtripTime : -1;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to measure latency for {ipAddress}: {ex.Message}");
                return -1;
            }
        }
    }

}
