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
    public partial class frm_language : Form
    {
        public frm_language()
        {
            InitializeComponent();
        }

        private void btn_confirm_Click(object sender, EventArgs e)
        {
            DialogResult = DialogResult.OK;
        }

        private void cb_selectLanguage_SelectedIndexChanged(object sender, EventArgs e)
        {
            switch (cb_selectLanguage.Text)
            {
                case "English":
                    pb_language.Image = Properties.Resources.english;
                    break;
                case "Persian":
                    pb_language.Image = Properties.Resources.persian;
                    break;
                case "Spain":
                    pb_language.Image = Properties.Resources.spain;
                    break;
                case "Germany":
                    pb_language.Image = Properties.Resources.germany;
                    break;
            }
        }
    }
}
