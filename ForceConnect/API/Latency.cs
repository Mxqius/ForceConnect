using System.Net.NetworkInformation;

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
            catch
            {
                return -1;
            }
        }
    }

}
