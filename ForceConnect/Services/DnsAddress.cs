using System;
using System.Collections.Generic;
using System.Drawing;

namespace ForceConnect.Services
{
    public class DnsAddress
    {
        public string UniqueID { get; set; }
        public string[] dnsAddress { get; set; }
        public string Name { get; set; }
        public int Latency { get; set; }
        public Bitmap Picture { get; set; }
    }
}
