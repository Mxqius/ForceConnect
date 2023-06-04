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
        private static List<DnsAddress> _servicesUser = new List<DnsAddress>();

        public static List<DnsAddress> GetServicesUser()
        {
            _servicesUser.Clear();
            _servicesUser.Add(new DnsAddress()
            {
                dnsAddress = new string[] { "178.22.122.100", "185.51.200.2" },
                Latency = 170,
                Name = "Shecan",
                Picture = Properties.Resources.shecan
            });
            _servicesUser.Add(new DnsAddress()
            {
                dnsAddress = new string[] { "78.157.42.101", "78.157.42.100" },
                Latency = 100,
                Name = "Electro",
                Picture = Properties.Resources.electro
            });
            _servicesUser.Add(new DnsAddress()
            {
                dnsAddress = new string[] { "10.202.10.10", "10.202.10.11" },
                Latency = 70,
                Name = "RadarGame",
                Picture = Properties.Resources.radargame
            });
            _servicesUser.Add(new DnsAddress()
            {
                dnsAddress = new string[] { "10.202.10.202", "10.202.10.102" },
                Latency = 80,
                Name = "403.Online",
                Picture = Properties.Resources.online403
            });
            _servicesUser.Add(new DnsAddress()
            {
                dnsAddress = new string[] { "8.8.8.8", "8.8.4.4" },
                Latency = 24,
                Name = "Google",
                Picture = Properties.Resources.google
            });
            _servicesUser.Add(new DnsAddress()
            {
                dnsAddress = new string[] { "1.1.1.1", "1.0.0.1" },
                Latency = 50,
                Name = "Cloudflare",
                Picture = Properties.Resources.cloudflare
            });

            string[] services = Service.GetServices();
            if (services == null) return _servicesUser;
            foreach (string service in services)
            {
                string[] servicePart = service.Split(' ');
                string name = servicePart[0];
                string address1 = servicePart[1];
                string address2 = servicePart[2];
                DnsAddress newAddress = new DnsAddress()
                {
                    Latency = 1,
                    Name = name,
                    Picture = Properties.Resources.dns

                };
                if (address2 != "empty")
                    newAddress.dnsAddress = new string[] { address1, address2 };
                else
                    newAddress.dnsAddress = new string[] { address1 };

                _servicesUser.Add(newAddress);
            }

            return _servicesUser;
        }
    }
}
