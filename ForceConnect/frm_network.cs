using ForceConnect.Interfaces;
using ForceConnect.Services;
using System;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_network : Form
    {
        public frm_network()
        {
            InitializeComponent();
        }

        private void btn_close_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        private async Task loadInformation()
        {
            await Task.Run(() =>
            {
                NetworkInterfaceInfo information = NetworkInformation.GetActiveNetworkInterfaceInfo();
                if (information == null) return;
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_intrfaceName.Text = information.ActiveInterfaceName;
                    lbl_intrfaceDesc.Text = information.Description;
                    lbl_intrfaceStatus.Text = information.Status.ToString();
                    lbl_ipAddress.Text = information.IPAddress.ToString();
                    lbl_hostName.Text = information.HostName.ToString();
                    lbl_speed.Text = NetworkInformation.ConvertBytesToMbps(information.Speed).ToString() + " Mbp/s";
                    if (information.DNSIPAddress.Length > 1)
                        lbl_activeServices.Text = information.DNSIPAddress[0].ToString() + " " + information.DNSIPAddress[1].ToString();
                    else
                        lbl_activeServices.Text = information.DNSIPAddress[0].ToString();
                    lbl_macAddress.Text = information.MACAddress.ToString();

                }));
            });
        }
        private async void frm_network_Load(object sender, EventArgs e)
        {
            await loadInformation();
        }

        private async void btn_refresh_Click(object sender, EventArgs e)
        {
            await loadInformation();
        }
    }
}
