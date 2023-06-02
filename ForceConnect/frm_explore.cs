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
        private List<DnsAddress> listOfDNS;
        public frm_explore(frm_main mainForm)
        {
            InitializeComponent();
            _mainForm = mainForm;
            listOfDNS.Add(_mainForm.Shecan);
            listOfDNS.Add(_mainForm.Electro);
            listOfDNS.Add(_mainForm.Online403);
            listOfDNS.Add(_mainForm.Cloudflare);
            listOfDNS.Add(_mainForm.Google);
            listOfDNS.Add(_mainForm.RadarGame);
        }

        private async void lbl_previewAddress_Click(object sender, EventArgs e)
        {
            Clipboard.SetText(lbl_previewAddress.Text);
            lbl_previewAddress.Text = "Address copied";
            await _mainForm.delay(1000);
            lbl_previewAddress.Text = _mainForm.currentDNS.dnsAddress[0] + " " + _mainForm.currentDNS.dnsAddress[1];
        }

        private void frm_explore_Load(object sender, EventArgs e)
        {

        }
    }
}
