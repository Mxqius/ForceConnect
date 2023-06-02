using ForceConnect.API;
using ForceConnect.DNS;
using ForceConnect.Services;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

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

        private void changeInformationCardDNS(DnsAddress DNS)
        {
            lbl_name.Text = DNS.Name;
            lbl_previewAddress.Text = DNS.dnsAddress[0] + " " + DNS.dnsAddress[1];            
            pb_dnsPicture.Image = DNS.Picture;
            lbl_latency.Text = Latency.MeasureLatency(DNS.dnsAddress[0]).ToString() + " ms";
        }
        private void frm_explore_Load(object sender, EventArgs e)
        {
            changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }
        private void updateCounter()
        {
            lbl_counter.Text = $"{currentIndex + 1} / {listOfDNS.Count}";
        }

        private void btn_next_Click(object sender, EventArgs e)
        {
            if (currentIndex < listOfDNS.Count - 1) currentIndex++;
            else currentIndex = 0;
            changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }

        private void btn_previous_Click(object sender, EventArgs e)
        {
            if (currentIndex > 1) currentIndex--;
            else currentIndex = (byte)((byte)listOfDNS.Count - 1);
            changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }
    }
}
