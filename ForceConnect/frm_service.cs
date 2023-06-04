using ForceConnect.API;
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

namespace ForceConnect
{
    public partial class frm_service : Form
    {
        public frm_service()
        {
            InitializeComponent();
        }

        private void btn_cancel_Click(object sender, EventArgs e)
        {
            DialogResult =  DialogResult.Cancel;
        }
        private void onlyAddress(object sender, KeyPressEventArgs e)
        {
            if (!char.IsDigit(e.KeyChar) && !char.IsControl(e.KeyChar) && e.KeyChar != '.')
                e.Handled = true;
        }
        private async void btn_addService_Click(object sender, EventArgs e)
        {
            if (txt_name.Text.Trim() == string.Empty) return;
            if (char.IsDigit(txt_name.Text.Trim()[0]))
            {
                new NotificationForm().showAlert("Not Valid", NotificationForm.enmType.Error);
                return;
            }

            if (txt_address1.Text.Trim() == string.Empty) return;


            if (await getLatencyDNS(txt_address1.Text.Trim()) != -1)
            {
                if (txt_address2.Text != string.Empty)
                    Service.AddService(txt_name.Text.Trim().Replace(" ", ""), txt_address1.Text.Trim(), txt_address2.Text.Trim());
                else
                    Service.AddService(txt_name.Text.Trim().Replace(" ",""), txt_address1.Text.Trim(), "empty");
                txt_name.Text = txt_address2.Text = txt_address1.Text = string.Empty;
                new NotificationForm().showAlert("Service Added", NotificationForm.enmType.Success);
                DialogResult = DialogResult.OK;
            }
            else
            {
                new NotificationForm().showAlert("Service Unavailable", NotificationForm.enmType.Error);
            }
        }
        private void updateLatencyPicture()
        {
            int latency = int.Parse(lbl_ping.Text.Split(' ')[0]);
            if (latency >= 180 || latency == -1)
                pb_latencyPicture.Image = Properties.Resources.signalRed;
            else if (latency >= 120)
                pb_latencyPicture.Image = Properties.Resources.signalYellow;
            else
                pb_latencyPicture.Image = Properties.Resources.signalGreen;
        }
        private async Task<long> getLatencyDNS(string address)
        {
            return (long)await Task.Run(() =>
            {
                return Latency.MeasureLatency(address);
            });

        }

        private async void btn_ping_Click(object sender, EventArgs e)
        {
            if (txt_address1.Text.Trim() != string.Empty)
            {
                long latency = await getLatencyDNS(txt_address1.Text.Trim());
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_ping.Text = latency.ToString() + " ms";
                }));
                updateLatencyPicture();
            }
        }
    }
}
