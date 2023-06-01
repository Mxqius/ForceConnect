using System;
using System.Collections.Generic;
using System.Linq;
using System.Net.NetworkInformation;
using System.Net;
using System.Text;
using System.Threading.Tasks;
using static System.Net.Mime.MediaTypeNames;
using System.Web.Configuration;

namespace ForceConnect.API
{
    public class Latency
    {
        public static long MeasureLatency(string ipAddress)
        {
            try
            {
                Ping pingSender = new Ping();
                PingReply reply = pingSender.Send(ipAddress);

                if (reply.Status == IPStatus.Success)
                {
                    return reply.RoundtripTime;
                }
                else
                {
                    return -1; // Indicate failure or unreachable host
                }
            }
            catch (Exception e)
            {
                return -1;
            }
        }
    }

}
