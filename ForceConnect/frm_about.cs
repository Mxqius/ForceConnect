using Guna.UI2.WinForms;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Diagnostics;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_about : Form
    {
        public frm_about()
        {
            InitializeComponent();
        }

        private void focusSocialMedia(object sender, MouseEventArgs e)
        {
            ((Guna2CircleButton)sender).ImageSize = new Size(38, 38);
        }

        private void unFocusSocialMedia(object sender, EventArgs e)
        {
            ((Guna2CircleButton)sender).ImageSize = new Size(35, 35);
        }

        private void btn_telegram_Click(object sender, EventArgs e)
        {
            Process.Start("https://t.me/mxqius");
        }

        private void btn_instagram_Click(object sender, EventArgs e)
        {
            Process.Start("https://instagram.com/mxqius");
        }

        private void btn_github_Click(object sender, EventArgs e)
        {
            Process.Start("https://github.com/mxqius");
        }

        private void pb_aboutPicture_Click(object sender, EventArgs e)
        {
            Process.Start("https://mxqius.ir");
        }
    }
}
