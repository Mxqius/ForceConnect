using System;
using System.Diagnostics;
using System.Linq;
using System.Net.NetworkInformation;
using System.Text.RegularExpressions;
using System.Threading;
using System.Windows.Forms;

public class DnsManager
{
    public static void setDNS(string[] dnsAddress)
    {
        try
        {
            NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();

            string arg1 = "netsh interface ipv4 set dns name=" + network.Name + " static " + dnsAddress[0];
            execute(arg1);

            if (dnsAddress.Length > 1)
            {
                string arg2 = "netsh interface ip add dns " + network.Name + " " + dnsAddress[1] + " index=2";
                execute(arg2);
            }
        }
        catch (Exception ex)
        {
            MessageBox.Show(ex.Message);

        }
    }
    public static void clearDNS()
    {
        try
        {
            NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
            string arg = $"netsh interface ip set dns {network.Name} dhcp";
            execute(arg);

        }
        catch (Exception ex)
        {
            MessageBox.Show(ex.Message);

        }
    }
    public static string[] getActiveDNS()
    {
        NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
        string command = $"netsh interface ip show dns {network.Name}";
        string result = execute(command);
        return extractDNS(result);
    }
    private static string[] extractDNS(string input)
    {
        const string pattern = @"Statically Configured DNS Servers:\s+(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})\s+(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})";
        Regex regex = new Regex(pattern);
        // Use the Matches method to find all matches in the input string
        MatchCollection matches = regex.Matches(input);
        string[] address = new string[2];
        // Iterate over the matches and extract the DNS servers
        foreach (Match match in matches)
        {
            for (int i = 1; i <= 2; i++)
            {
                Group group = match.Groups[i];
                address[i - 1] = group.Value;
            }
        }
        return address;
    }
    public static void flushDNS()
    {
        try
        {
            string arg = $"ipconfig /flushdns";
            execute(arg);

        }
        catch (Exception ex)
        {
            MessageBox.Show(ex.Message);

        }
    }
    public static string execute(string arg)
    {
        try
        {
            NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
            Process process = new Process();

            // Set the required process information
            process.StartInfo.FileName = "cmd.exe";
            process.StartInfo.Arguments = @"/c " + arg;  // Command to execute
            process.StartInfo.UseShellExecute = false;
            process.StartInfo.CreateNoWindow = true;
            process.StartInfo.RedirectStandardOutput = true;
            process.StartInfo.Verb = "runas";
            process.StartInfo.WindowStyle = ProcessWindowStyle.Hidden;
            // Start the process
            process.Start();
            return process.StandardOutput.ReadToEnd();
        }
        catch (Exception ex)
        {
            MessageBox.Show(ex.Message);
            return ex.Message;
        }
    }
    public static NetworkInterface GetActiveEthernetOrWifiNetworkInterface()
    {
        NetworkInterface Nic = NetworkInterface.GetAllNetworkInterfaces().FirstOrDefault(
            a => a.OperationalStatus == OperationalStatus.Up &&
            (a.NetworkInterfaceType == NetworkInterfaceType.Wireless80211 || a.NetworkInterfaceType == NetworkInterfaceType.Ethernet) &&
            a.GetIPProperties().GatewayAddresses.Any(g => g.Address.AddressFamily.ToString() == "InterNetwork"));

        return Nic;
    }
}
