using ForceConnect.Interfaces;
using ForceConnect.Services;
using ForceConnect.Utility;
using System;
using System.Collections.Generic;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_network : Form
    {
        private List<NetworkInterfaceInfo> _networks;
        public frm_network()
        {
            InitializeComponent();
            btn_close.Enabled = false;
        }

        private void btn_close_Click(object sender, EventArgs e)
        {
            this.Close();
        }

        public void GetNetworkSpeed()
        {
            this.Invoke(new Action(async () =>
            {
                // lbl_downloadSpeed.Text = $"{await SpeedTest.MeasureDownloadSpeedAsync("")} MBps";
                loadingProgressSpeed.Stop();
                loadingProgressSpeed.Visible = false;
            }));
        }

        private async Task loadInformation()
        {
            await Task.Run(() =>
            {
                try
                {
                    NetworkInterfaceInfo information = NetworkInformation.GetActiveNetworkInterfaceInfo();
                    if (information == null) return;
                    this.Invoke(new MethodInvoker(delegate
                    {
                        loadingProgressSpeed.Visible = true;
                        loadingProgressSpeed.Start();
                        lbl_intrfaceName.Text = information.InterfaceName;
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
                    GetNetworkSpeed();
                }
                catch
                {
                    return;
                }
            });
        }
        private async void frm_network_Load(object sender, EventArgs e)
        {
            await loadInformation();
            await GetAllNetworkInterfacesInfo();
            btn_close.Enabled = true;
        }

        private async void btn_refresh_Click(object sender, EventArgs e)
        {
            await loadInformation();
        }
        private async Task GetAllNetworkInterfacesInfo()
        {
            await Task.Run(() =>
            {
                NetworkInterfaceInfo activeNetwork = NetworkInformation.GetActiveNetworkInterfaceInfo();
                _networks = NetworkInformation.GetAllNetworkInterfacesInfo();
                foreach (NetworkInterfaceInfo network in _networks)
                {
                    this.Invoke(new MethodInvoker(delegate
                    {
                        cb_selectNetworkAdapter.Items.Add(network.InterfaceName);
                        cb_selectNetworkAdapter.SelectedIndex = cb_selectNetworkAdapter.FindStringExact(activeNetwork.InterfaceName);
                    }));
                }
            });

        }
        private void cb_selectNetworkAdapter_SelectedIndexChanged(object sender, EventArgs e)
        {
            lbl_hintNetworkAdapter.Visible = false;
            NetworkInterfaceInfo networkSelected = _networks.Find(x => x.InterfaceName == cb_selectNetworkAdapter.Text);
            lbl_intrfaceName.Text = networkSelected.InterfaceName;
            lbl_intrfaceDesc.Text = networkSelected.Description;
            lbl_intrfaceStatus.Text = networkSelected.Status.ToString();
        }

    }
}
