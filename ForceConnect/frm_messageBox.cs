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
    public partial class frm_messageBox : Form
    {
        public enum Icon
        {
            Success,
            Error,
            Info,
            Warning

        }
        public enum Buttons
        {
            OK,
            YesNo,
            OKCancel,
            RetryCancel,
            YesNoCancel

        }
        public frm_messageBox()
        {
            InitializeComponent();
        }
        public string MessageCaption
        {
            get
            {
                return lbl_caption.Text;
            }
            set
            {
                lbl_caption.Text = value.ToUpper();
            }
        }
        public string MessageText
        {
            get
            {
                return lbl_text.Text;
            }
            set
            {
                lbl_text.Text = value;
            }
        }
        public Color PrimaryForeColor
        {
            get
            {
                return btn_primary.ForeColor;
            }
            set
            {
                btn_primary.ForeColor = value;
            }
        }
        public Color SecondaryForeColor
        {
            get
            {
                return btn_secoundry.ForeColor;
            }
            set
            {
                btn_secoundry.ForeColor = value;
            }
        }
        public Icon MessageIcon { get; set; }
        public Buttons MessageButtons { get; set; }

        public DialogResult ShowMessage()
        {
            switch (MessageIcon)
            {
                case Icon.Success:
                    pb_header.Image = Properties.Resources.successMessage;
                    break;

                case Icon.Error:
                    pb_header.Image = Properties.Resources.errorMessage;
                    break;

                case Icon.Info:
                    pb_header.Image = Properties.Resources.infoMessage;
                    break;

                case Icon.Warning:
                    pb_header.Image = Properties.Resources.warningMessage;
                    break;
            }
            switch (MessageButtons)
            {
                case Buttons.OK:
                    btn_primary.Text = "OK";
                    btn_secoundry.Visible = false;
                    btn_three.Visible = false;
                    break;
                case Buttons.YesNo:
                    btn_primary.Text = "YES";
                    btn_secoundry.Text = "NO";
                    btn_three.Visible = false;
                    break;
                case Buttons.YesNoCancel:
                    btn_primary.Text = "YES";
                    btn_secoundry.Text = "NO";
                    btn_three.Text = "CANCEL";
                    break;
                case Buttons.OKCancel:
                    btn_primary.Text = "OK";
                    btn_secoundry.Text = "CANCEL";
                    btn_three.Visible = false;
                    break;
                case Buttons.RetryCancel:
                    btn_primary.Text = "RETRY";
                    btn_secoundry.Text = "CANCEL";
                    btn_three.Visible = false;
                    break;

            }
            return this.ShowDialog();
        }

        private void btn_primary_Click(object sender, EventArgs e)
        {
            switch (MessageButtons)
            {
                case Buttons.OK:
                    DialogResult = DialogResult.OK;
                    break;
                case Buttons.YesNo:
                    DialogResult = DialogResult.Yes;
                    break;
                case Buttons.YesNoCancel:
                    DialogResult = DialogResult.Yes;
                    break;
                case Buttons.RetryCancel:
                    DialogResult = DialogResult.Retry;
                    break;
                case Buttons.OKCancel:
                    DialogResult = DialogResult.OK;
                    break;
            }

        }

        private void btn_secoundry_Click(object sender, EventArgs e)
        {
            switch (MessageButtons)
            {
                case Buttons.YesNo:
                    DialogResult = DialogResult.No;
                    break;
                case Buttons.YesNoCancel:
                    DialogResult = DialogResult.Yes;
                    break;
                case Buttons.RetryCancel:
                    DialogResult = DialogResult.Cancel;
                    break;
                case Buttons.OKCancel:
                    DialogResult = DialogResult.Cancel;
                    break;
            }
        }

        private void btn_three_Click(object sender, EventArgs e)
        {
            switch (MessageButtons)
            {
                case Buttons.YesNoCancel:
                    DialogResult = DialogResult.Cancel;
                    break;
            }
        }

        private void btn_close_Click(object sender, EventArgs e)
        {
            DialogResult = DialogResult.No;
        }
    }
}
