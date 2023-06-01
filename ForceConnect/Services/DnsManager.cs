using System;
using System.Diagnostics;
using System.Linq;
using System.Management;
using System.Net;
using System.Net.NetworkInformation;
using System.Security.Cryptography;
using static Guna.UI2.Native.WinApi;
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
    public static void execute(string arg)
    {
        try
        {
            NetworkInterface network = GetActiveEthernetOrWifiNetworkInterface();
            ProcessStartInfo psi = new ProcessStartInfo("cmd.exe");
            psi.UseShellExecute = true;
            psi.WindowStyle = ProcessWindowStyle.Hidden;
            psi.Verb = "runas";
            psi.Arguments = @"/c " + arg;
            Process.Start(psi);

        }
        catch (Exception ex)
        {
            MessageBox.Show(ex.Message);

        }
    }
    public static NetworkInterface GetActiveEthernetOrWifiNetworkInterface()
    {
        var Nic = NetworkInterface.GetAllNetworkInterfaces().FirstOrDefault(
            a => a.OperationalStatus == OperationalStatus.Up &&
            (a.NetworkInterfaceType == NetworkInterfaceType.Wireless80211 || a.NetworkInterfaceType == NetworkInterfaceType.Ethernet) &&
            a.GetIPProperties().GatewayAddresses.Any(g => g.Address.AddressFamily.ToString() == "InterNetwork"));

        return Nic;
    }
}
