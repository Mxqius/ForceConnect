using ForceConnect.API;
using ForceConnect.Services;
using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Xml.Linq;

namespace ForceConnect.Services
{
    internal class DnsAddressItems
    {
        private static List<DnsAddress> _servicesUser = new List<DnsAddress>();

        /// <summary>
        /// Gets all available DNS services including built-in and user-added services.
        /// </summary>
        /// <returns>List of DNS address services.</returns>
        public static List<DnsAddress> GetServicesUser()
        {
            _servicesUser.Clear();

            // Add built-in services
            var builtInServices = new[]
            {
                new DnsAddress
                {
                    UniqueID = Service.GenerateUniqueID(6),
                    dnsAddress = new[] { "178.22.122.100", "185.51.200.2" },
                    Latency = 170,
                    Name = "Shecan",
                    Picture = Properties.Resources.shecan
                },
                new DnsAddress
                {
                    UniqueID = Service.GenerateUniqueID(6),
                    dnsAddress = new[] { "78.157.42.101", "78.157.42.100" },
                    Latency = 100,
                    Name = "Electro",
                    Picture = Properties.Resources.electro
                },
                new DnsAddress
                {
                    UniqueID = Service.GenerateUniqueID(6),
                    dnsAddress = new[] { "10.202.10.10", "10.202.10.11" },
                    Latency = 70,
                    Name = "RadarGame",
                    Picture = Properties.Resources.radargame
                },
                new DnsAddress
                {
                    UniqueID = Service.GenerateUniqueID(6),
                    dnsAddress = new[] { "10.202.10.202", "10.202.10.102" },
                    Latency = 80,
                    Name = "403.Online",
                    Picture = Properties.Resources.online403
                },
                new DnsAddress
                {
                    UniqueID = Service.GenerateUniqueID(6),
                    dnsAddress = new[] { "8.8.8.8", "8.8.4.4" },
                    Latency = 24,
                    Name = "Google",
                    Picture = Properties.Resources.google
                },
                new DnsAddress
                {
                    UniqueID = Service.GenerateUniqueID(6),
                    dnsAddress = new[] { "1.1.1.1", "1.0.0.1" },
                    Latency = 50,
                    Name = "Cloudflare",
                    Picture = Properties.Resources.cloudflare
                }
            };

            _servicesUser.AddRange(builtInServices);

            // Add user-defined services
            var userServices = Service.GetServices();
            if (userServices != null)
            {
                var userDnsAddresses = userServices
                    .Where(service => !string.IsNullOrWhiteSpace(service))
                    .Select(service =>
                    {
                        var serviceParts = service.Split(' ');
                        if (serviceParts.Length < 4) return null;

                        return new DnsAddress
                        {
                            UniqueID = serviceParts[0],
                            Latency = 1,
                            Name = serviceParts[1],
                            Picture = Properties.Resources.dns,
                            dnsAddress = serviceParts[3] != "empty" 
                                ? new[] { serviceParts[2], serviceParts[3] }
                                : new[] { serviceParts[2] }
                        };
                    })
                    .Where(dns => dns != null);

                _servicesUser.AddRange(userDnsAddresses);
            }

            return _servicesUser;
        }
    }
}
