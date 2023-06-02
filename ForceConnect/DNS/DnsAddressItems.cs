using ForceConnect.API;
using ForceConnect.Services;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;

namespace ForceConnect.DNS
{
    internal class DnsAddressItems
    {
        public DnsAddress Shecan, Electro, Online403, Google, Cloudflare, RadarGame;
        public DnsAddressItems()
        {
            // Setup default servers        
            Shecan = new DnsAddress()
            {
                dnsAddress = new string[] { "178.22.122.100", "185.51.200.2" },
                Latency = 170,
                Name = "Shecan",
                Picture = Properties.Resources.shecan
            };
            Electro = new DnsAddress()
            {
                dnsAddress = new string[] { "78.157.42.101", "78.157.42.100" },
                Latency = 100,
                Name = "Electro",
                Picture = Properties.Resources.electro
            };
            RadarGame = new DnsAddress()
            {
                dnsAddress = new string[] { "10.202.10.10", "10.202.10.11" },
                Latency = 70,
                Name = "RadarGame",
                Picture = Properties.Resources.radargame
            };
            Online403 = new DnsAddress()
            {
                dnsAddress = new string[] { "10.202.10.202", "10.202.10.102" },
                Latency = 80,
                Name = "403.Online",
                Picture = Properties.Resources.online403
            };
            Google = new DnsAddress()
            {
                dnsAddress = new string[] { "8.8.8.8", "8.8.4.4" },
                Latency = 24,
                Name = "Google",
                Picture = Properties.Resources.google
            };
            Cloudflare = new DnsAddress()
            {
                dnsAddress = new string[] { "1.1.1.1", "1.0.0.1" },
                Latency = 50,
                Name = "Cloudflare",
                Picture = Properties.Resources.cloudflare
            };
        }
    }
}
