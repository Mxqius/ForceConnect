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
        public static List<NetworkInterfaceInfo> GetAllNetworkInterfacesInfo()
        {
            List<NetworkInterfaceInfo> interfacesInfo = new List<NetworkInterfaceInfo>();

            NetworkInterface[] networkInterfaces = NetworkInterface.GetAllNetworkInterfaces();

            foreach (NetworkInterface networkInterface in networkInterfaces)
            {
                IPInterfaceProperties ipProperties = networkInterface.GetIPProperties();
                IPAddress ipAddress = ipProperties.UnicastAddresses.FirstOrDefault(a => a.Address.AddressFamily == AddressFamily.InterNetwork)?.Address;
                IPAddress subnetMask = ipProperties.UnicastAddresses.FirstOrDefault(a => a.Address.AddressFamily == AddressFamily.InterNetwork)?.IPv4Mask;
                string hostName = Dns.GetHostName();
                IPAddress[] dnsIPAddresses = ipProperties.DnsAddresses.Where(a => a.AddressFamily == AddressFamily.InterNetwork).ToArray();

                interfacesInfo.Add(new NetworkInterfaceInfo
                {
                    InterfaceName = networkInterface.Name,
                    Description = networkInterface.Description,
                    Status = networkInterface.OperationalStatus,
                    MACAddress = networkInterface.GetPhysicalAddress().ToString(),
                    Speed = networkInterface.Speed,
                    IPAddress = ipAddress,
                    SubnetMask = subnetMask,
                    HostName = hostName,
                    DNSIPAddress = dnsIPAddresses
                });
            }

            return interfacesInfo;
        }
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
                    InterfaceName = activeInterface.Name,
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
