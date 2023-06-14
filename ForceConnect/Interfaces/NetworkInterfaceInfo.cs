using System.Net.NetworkInformation;
using System.Net;

namespace ForceConnect.Interfaces
{
    public class NetworkInterfaceInfo
    {
        public string ActiveInterfaceName { get; set; }
        public string Description { get; set; }
        public OperationalStatus Status { get; set; }
        public string MACAddress { get; set; }
        public long Speed { get; set; }
        public IPAddress IPAddress { get; set; }
        public IPAddress SubnetMask { get; set; }
        public string HostName { get; set; }
        public IPAddress DNSIPAddress { get; set; }
    }
}
