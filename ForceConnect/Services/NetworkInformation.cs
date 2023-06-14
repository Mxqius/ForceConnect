using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.NetworkInformation;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using System.Net.Sockets;
using ForceConnect.Interfaces;

namespace ForceConnect.Services
{
    internal class NetworkInformation
    {
        public static NetworkInterfaceInfo GetActiveNetworkInterfaceInfo()
        {
            NetworkInterface activeInterface = NetworkInterface.GetAllNetworkInterfaces().FirstOrDefault(
                a => a.OperationalStatus == OperationalStatus.Up &&
                (a.NetworkInterfaceType == NetworkInterfaceType.Wireless80211 || a.NetworkInterfaceType == NetworkInterfaceType.Ethernet) &&
                a.GetIPProperties().GatewayAddresses.Any(g => g.Address.AddressFamily == AddressFamily.InterNetwork));

            if (activeInterface != null)
            {
                IPInterfaceProperties ipProperties = activeInterface.GetIPProperties();
                IPAddress ipAddress = ipProperties.UnicastAddresses.FirstOrDefault(a => a.Address.AddressFamily == AddressFamily.InterNetwork)?.Address;
                IPAddress subnetMask = ipProperties.UnicastAddresses.FirstOrDefault(a => a.Address.AddressFamily == AddressFamily.InterNetwork)?.IPv4Mask;
                string hostName = Dns.GetHostName();
                IPAddress[] dnsIPAddresses = ipProperties.DnsAddresses.Where(a => a.AddressFamily == AddressFamily.InterNetwork).ToArray();

                return new NetworkInterfaceInfo
                {
                    ActiveInterfaceName = activeInterface.Name,
                    Description = activeInterface.Description,
                    Status = activeInterface.OperationalStatus,
                    MACAddress = activeInterface.GetPhysicalAddress().ToString(),
                    Speed = activeInterface.Speed,
                    IPAddress = ipAddress,
                    SubnetMask = subnetMask,
                    HostName = hostName,
                    DNSIPAddress = dnsIPAddresses
                };
            }
            else
            {
                return null;
            }
        }
        public static double ConvertBytesToMbps(long bytes)
        {
            double bits = bytes * 8; // Convert bytes to bits
            double mbps = bits / 1000000; // Convert bits to megabits
            return Math.Round(mbps, 2);
        }
    }
}
