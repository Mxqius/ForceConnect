using ForceConnect.API;
using ForceConnect.DNS;
using ForceConnect.Services;
using Guna.UI2.WinForms;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using static System.Net.Mime.MediaTypeNames;

namespace ForceConnect
{
    public partial class frm_explore : Form
    {
        private frm_main _mainForm;
        private byte currentIndex = 0;
        private List<DnsAddress> listOfDNS = new List<DnsAddress>();

        private DnsAddressItems DnsAddress;

        public frm_explore(frm_main mainForm)
        {
            InitializeComponent();
            _mainForm = mainForm;
            DnsAddress = new DnsAddressItems();
            listOfDNS.Add(DnsAddress.Shecan);
            listOfDNS.Add(DnsAddress.Electro);
            listOfDNS.Add(DnsAddress.Online403);
            listOfDNS.Add(DnsAddress.Cloudflare);
            listOfDNS.Add(DnsAddress.Google);
            listOfDNS.Add(DnsAddress.RadarGame);
        }

        private async void lbl_previewAddress_Click(object sender, EventArgs e)
        {
            Clipboard.SetText(lbl_previewAddress.Text);
            lbl_previewAddress.Text = "Address copied";
            await _mainForm.delay(1000);
            lbl_previewAddress.Text = _mainForm.currentDNS.dnsAddress[0] + " " + _mainForm.currentDNS.dnsAddress[1];
        }
        private void updateLatencyPicture()
        {
            int latency = int.Parse(lbl_latency.Text.Split(' ')[0]);
            if (latency >= 180 || latency == -1)
                pb_latencyPicture.Image = Properties.Resources.signalRed;
            else if (latency >= 120)
                pb_latencyPicture.Image = Properties.Resources.signalYellow;
            else
                pb_latencyPicture.Image = Properties.Resources.signalGreen;
        }
        private async Task<bool> changeInformationCardDNS(DnsAddress DNS)
        {
            return await Task.Run(() =>
            {
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_name.Text = DNS.Name;
                    lbl_previewAddress.Text = DNS.dnsAddress[0] + " " + DNS.dnsAddress[1];
                    pb_dnsPicture.Image = DNS.Picture;
                    lbl_latency.Text = Latency.MeasureLatency(DNS.dnsAddress[0]).ToString() + " ms";
                    updateLatencyPicture();
                }));
                return true;
            });
        }
        private async void frm_explore_Load(object sender, EventArgs e)
        {
            await changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }
        private void updateCounter()
        {
            lbl_counter.Text = $"{currentIndex + 1} / {listOfDNS.Count}";
        }

        private async void btn_next_Click(object sender, EventArgs e)
        {
            if (currentIndex < listOfDNS.Count - 1) currentIndex++;
            else currentIndex = 0;
            await changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }

        private async void btn_previous_Click(object sender, EventArgs e)
        {
            if (currentIndex > 1) currentIndex--;
            else currentIndex = (byte)((byte)listOfDNS.Count - 1);
            await changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }
        private void onlyAddress(object sender, KeyPressEventArgs e)
        {
            if (!char.IsDigit(e.KeyChar) && !char.IsControl(e.KeyChar) && e.KeyChar != '.')
                e.Handled = true;
        }

        private void btn_openServices_Click(object sender, EventArgs e)
        {
            new frm_service().ShowDialog();
        }
    }
}
