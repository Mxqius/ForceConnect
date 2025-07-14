using System;
using System.Diagnostics;
using System.Linq;
using System.Net.NetworkInformation;
using System.Text.RegularExpressions;
using System.Threading;
using System.Windows.Forms;

    /// <summary>
    /// Manages DNS configuration operations for the application with performance optimizations.
    /// </summary>
    public class DnsManager
    {
        private static NetworkInterface _cachedActiveInterface;
        private static readonly object _lockObject = new object();
        private static readonly TimeSpan _cacheExpiration = TimeSpan.FromMinutes(5);

        /// <summary>
        /// Sets the DNS servers for the active network interface with optimized performance.
        /// </summary>
        /// <param name="dnsAddresses">Array of DNS server addresses to set.</param>
        /// <exception cref="ArgumentNullException">Thrown when dnsAddresses is null.</exception>
        /// <exception cref="ArgumentException">Thrown when dnsAddresses is empty.</exception>
        public static void SetDNS(string[] dnsAddresses)
        {
            if (dnsAddresses == null)
                throw new ArgumentNullException(nameof(dnsAddresses));
            
            if (dnsAddresses.Length == 0)
                throw new ArgumentException("DNS addresses array cannot be empty.", nameof(dnsAddresses));

            try
            {
                NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
                if (network == null)
                {
                    throw new InvalidOperationException("No active network interface found.");
                }

                // Optimize command execution with better error handling
                var commands = new List<string>();
                commands.Add($"netsh interface ipv4 set dns name=\"{network.Name}\" static {dnsAddresses[0]}");

                if (dnsAddresses.Length > 1)
                {
                    commands.Add($"netsh interface ip add dns \"{network.Name}\" {dnsAddresses[1]} index=2");
                }

                // Execute commands with optimized performance
                foreach (var command in commands)
                {
                    var result = Execute(command);
                    if (!string.IsNullOrEmpty(result) && result.Contains("error", StringComparison.OrdinalIgnoreCase))
                    {
                        throw new InvalidOperationException($"DNS command failed: {result}");
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to set DNS: {ex.Message}");
                MessageBox.Show($"Failed to set DNS: {ex.Message}", "DNS Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
                throw;
            }
        }
    /// <summary>
    /// Clears DNS settings and reverts to DHCP configuration.
    /// </summary>
    public static void ClearDNS()
    {
        try
        {
            NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
            if (network == null)
            {
                throw new InvalidOperationException("No active network interface found.");
            }

            string command = $"netsh interface ip set dns \"{network.Name}\" dhcp";
            Execute(command);
        }
        catch (Exception ex)
        {
            MessageBox.Show($"Failed to clear DNS: {ex.Message}", "DNS Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            throw;
        }
    }
    /// <summary>
    /// Gets the currently active DNS servers for the active network interface.
    /// </summary>
    /// <returns>Array of DNS server addresses.</returns>
    public static string[] GetActiveDNS()
    {
        NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
        if (network == null)
        {
            throw new InvalidOperationException("No active network interface found.");
        }

        string command = $"netsh interface ip show dns \"{network.Name}\"";
        string result = Execute(command);
        return ExtractDNS(result);
    }
    /// <summary>
    /// Extracts DNS server addresses from the netsh command output.
    /// </summary>
    /// <param name="input">The output string from netsh command.</param>
    /// <returns>Array of DNS server addresses.</returns>
    private static string[] ExtractDNS(string input)
    {
        if (string.IsNullOrEmpty(input))
            return new string[0];

        const string pattern = @"Statically Configured DNS Servers:\s+(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\s+(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})";
        Regex regex = new Regex(pattern);
        MatchCollection matches = regex.Matches(input);
        
        if (matches.Count == 0)
            return new string[0];

        string[] addresses = new string[2];
        Match match = matches[0]; // Take the first match
        
        for (int i = 1; i <= 2; i++)
        {
            Group group = match.Groups[i];
            addresses[i - 1] = group.Value;
        }
        
        return addresses;
    }
    /// <summary>
    /// Flushes the DNS cache.
    /// </summary>
    public static void FlushDNS()
    {
        try
        {
            string command = "ipconfig /flushdns";
            Execute(command);
        }
        catch (Exception ex)
        {
            MessageBox.Show($"Failed to flush DNS cache: {ex.Message}", "DNS Error", MessageBoxButtons.OK, MessageBoxIcon.Error);
            throw;
        }
    }
    /// <summary>
    /// Executes a command with elevated privileges.
    /// </summary>
    /// <param name="command">The command to execute.</param>
    /// <returns>The output of the command.</returns>
    private static string Execute(string command)
    {
        try
        {
            Process process = new Process();

            process.StartInfo.FileName = "cmd.exe";
            process.StartInfo.Arguments = $"/c {command}";
            process.StartInfo.UseShellExecute = false;
            process.StartInfo.CreateNoWindow = true;
            process.StartInfo.RedirectStandardOutput = true;
            process.StartInfo.RedirectStandardError = true;
            process.StartInfo.Verb = "runas";
            process.StartInfo.WindowStyle = ProcessWindowStyle.Hidden;

            process.Start();
            string output = process.StandardOutput.ReadToEnd();
            string error = process.StandardError.ReadToEnd();
            process.WaitForExit();

            if (!string.IsNullOrEmpty(error))
            {
                throw new InvalidOperationException($"Command failed: {error}");
            }

            return output;
        }
        catch (Exception ex)
        {
            throw new InvalidOperationException($"Failed to execute command '{command}': {ex.Message}", ex);
        }
    }
        /// <summary>
        /// Gets the active Ethernet or WiFi network interface with caching for better performance.
        /// </summary>
        /// <returns>The active network interface, or null if none found.</returns>
        public static NetworkInterface GetActiveEthernetOrWifiNetworkInterface()
        {
            lock (_lockObject)
            {
                // Check if we have a cached interface that's still valid
                if (_cachedActiveInterface != null && 
                    _cachedActiveInterface.OperationalStatus == OperationalStatus.Up)
                {
                    return _cachedActiveInterface;
                }

                // Find active interface with optimized query
                var activeInterface = NetworkInterface.GetAllNetworkInterfaces().FirstOrDefault(
                    nic => nic.OperationalStatus == OperationalStatus.Up &&
                           (nic.NetworkInterfaceType == NetworkInterfaceType.Wireless80211 || 
                            nic.NetworkInterfaceType == NetworkInterfaceType.Ethernet) &&
                           nic.GetIPProperties().GatewayAddresses.Any(g => g.Address.AddressFamily.ToString() == "InterNetwork"));

                // Cache the result for better performance
                _cachedActiveInterface = activeInterface;
                return activeInterface;
            }
        }
}
