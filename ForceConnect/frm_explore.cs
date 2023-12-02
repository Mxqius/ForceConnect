using ForceConnect.API;
using ForceConnect.Services;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Windows.Forms;


namespace ForceConnect
{
    public partial class frm_explore : Form
    {
        private frm_main _mainForm;
        private byte currentIndex = 0;
        private List<DnsAddress> listOfDNS = new List<DnsAddress>();

        public frm_explore(frm_main mainForm)
        {
            InitializeComponent();
            _mainForm = mainForm;
            listOfDNS.AddRange(DnsAddressItems.GetServicesUser());
        }
        private async Task<bool> updateList()
        {
            return await Task.Run(() =>
            {
                listOfDNS.Clear();
                listOfDNS.AddRange(DnsAddressItems.GetServicesUser());
                this.Invoke(new MethodInvoker(delegate
                {
                    updateCounter();
                }));
                return true;
            });
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
        private async Task<long> getLatencyDNS(string address)
        {
            return await Task.Run(() =>
            {
                return Latency.MeasureLatency(address);
            });
        }

        private async void changeInformationCardDNS(DnsAddress DNS)
        {
            long latency = await getLatencyDNS(DNS.dnsAddress[0]);
            lbl_name.Text = DNS.Name;
            if (DNS.dnsAddress.Length > 1)
                lbl_previewAddress.Text = DNS.dnsAddress[0] + " " + DNS.dnsAddress[1];
            else
                lbl_previewAddress.Text = DNS.dnsAddress[0];
            pb_dnsPicture.Image = DNS.Picture;
            lbl_latency.Text = latency.ToString() + " ms";
            updateLatencyPicture();
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
            if (currentIndex > 0) currentIndex--;
            else currentIndex = (byte)((byte)listOfDNS.Count - 1);
            changeInformationCardDNS(listOfDNS[currentIndex]);
            updateCounter();
        }

        private async void btn_openServices_Click(object sender, EventArgs e)
        {
            if (new frm_service().ShowDialog() == DialogResult.OK)
            {
                await updateList();
                currentIndex = ((byte)(listOfDNS.Count - 1));
                changeInformationCardDNS(listOfDNS[currentIndex]);
            }

        }
        private async void btn_refresh_Click(object sender, EventArgs e)
        {
            await updateList();
        }

        private async void btn_deleteService_Click(object sender, EventArgs e)
        {
            if (!_mainForm._connected)
            {
                DialogResult result = new frm_messageBox()
                {
                    MessageText = "Are you sure to delete this service?",
                    MessageCaption = "Warning",
                    MessageButtons = frm_messageBox.Buttons.YesNo,
                    MessageIcon = frm_messageBox.Icon.Warning
                }.ShowMessage();
                if (result == DialogResult.Yes)
                {
                    Service.DeleteService(listOfDNS[currentIndex].UniqueID);
                    await updateList();
                    changeInformationCardDNS(listOfDNS[0]);
                    updateCounter();
                }
            }
            else
            {
                DialogResult result = new frm_messageBox()
                {
                    MessageText = "Please first disconnect from active service",
                    MessageCaption = "Error",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Error
                }.ShowMessage();
            }
        }
    }
}
