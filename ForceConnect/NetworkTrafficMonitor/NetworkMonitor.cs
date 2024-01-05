using System;
using System.Diagnostics;
using System.Reflection.Emit;
using System.Threading;
using System.Windows.Forms;
using Label = System.Windows.Forms.Label;

namespace ForceConnect.NetworkTrafficMonitor
{

    public class NetworkMonitor
    {
        private readonly string networkInterfaceInstanceName;
        private readonly PerformanceCounter downloadCounter;
        private readonly PerformanceCounter uploadCounter;

        public NetworkMonitor(string networkInterfaceInstanceName)
        {
            this.networkInterfaceInstanceName = networkInterfaceInstanceName;

            // ایجاد شیء PerformanceCounter برای نظارت بر حجم دانلود
            downloadCounter = new PerformanceCounter("Network Interface", "Bytes Received/sec", networkInterfaceInstanceName);

            // ایجاد شیء PerformanceCounter برای نظارت بر حجم آپلود
            uploadCounter = new PerformanceCounter("Network Interface", "Bytes Sent/sec", networkInterfaceInstanceName);
        }
        public void StartMonitoring(Label labelDownloadTraffic, Label labelUploadTraffic)
        {
            try
            {
                while (true)
                {
                    // خواندن و چاپ حجم دانلود و آپلود در هر ثانیه
                    float downloadSpeed = downloadCounter.NextValue() / 1024; // به کیلوبایت
                    float uploadSpeed = uploadCounter.NextValue() / 1024; // به کیلوبایت

                    //Console.WriteLine($"حجم دانلود: {downloadSpeed:F2} KB/s | حجم آپلود: {uploadSpeed:F2} KB/s");
                    labelDownloadTraffic.Text = $"{downloadSpeed:F2} KB/s ";
                    labelUploadTraffic.Text = $"{uploadSpeed:F2} KB/s ";

                    Thread.Sleep(1000); // تاخیر یک ثانیه
                }
            }
            catch (Exception ex)
            {
                MessageBox.Show($"خطا: {ex.Message}");
            }
        }

    }
}
