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

        private void btn_clear_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        private void btn_addService_Click(object sender, EventArgs e)
        {
            if (txt_name.Text.Trim() == string.Empty) return;
            if (char.IsDigit(txt_name.Text.Trim()[0]))
            {
                new NotificationForm().showAlert("Not Valid", NotificationForm.enmType.Error);
                return;
            }

            if (txt_address1.Text.Trim() == string.Empty) return;
            if (txt_address2.Text != string.Empty)
                Service.AddService(txt_name.Text.Trim(), txt_address1.Text.Trim(), txt_address2.Text);
            else
                Service.AddService(txt_name.Text.Trim(), txt_address1.Text.Trim(), "empty");
            txt_name.Text = txt_address2.Text = txt_address1.Text = string.Empty;
            new NotificationForm().showAlert("Service Added", NotificationForm.enmType.Success);
            this.Close();
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
        private async Task<bool> syncLatencyDNS(Label latencyLabel, string address)
        {
            return await Task.Run(() =>
            {
                this.Invoke(new MethodInvoker(delegate
                {
                    latencyLabel.Text = Latency.MeasureLatency(address).ToString() + " ms";
                }));
                return true;
            });

        }
        private async void btn_ping_Click(object sender, EventArgs e)
        {
            if (txt_address1.Text.Trim() != string.Empty)
            {
                await syncLatencyDNS(lbl_ping, txt_address1.Text);
                updateLatencyPicture();
            }
        }
    }
}
