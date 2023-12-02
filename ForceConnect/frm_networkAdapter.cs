using ForceConnect.Interfaces;
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
    public partial class frm_networkAdapter : Form
    {
        private List<NetworkInterfaceInfo> _networks;
        public frm_networkAdapter()
        {
            InitializeComponent();
        }
        private async void GetAllNetworkInterfacesInfo()
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

        private void frm_networkAdapter_Load(object sender, EventArgs e)
        {
            GetAllNetworkInterfacesInfo();
        }

        private void cb_selectNetworkAdapter_SelectedIndexChanged(object sender, EventArgs e)
        {
            lbl_hintNetworkAdapter.Visible = false;
            NetworkInterfaceInfo networkSelected = _networks.Find(x => x.InterfaceName == cb_selectNetworkAdapter.Text);
            lbl_intrfaceName.Text = networkSelected.InterfaceName;
            lbl_intrfaceDesc.Text = networkSelected.Description;
            lbl_intrfaceStatus.Text = networkSelected.Status.ToString();
        }

        private void btn_cancel_Click(object sender, EventArgs e)
        {
            DialogResult = DialogResult.Cancel;
        }

        private void btn_confirm_Click(object sender, EventArgs e)
        {
            DialogResult = DialogResult.OK;
        }
    }
}
