using System;
using System.Drawing;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_messageBox : Form
    {
        private Color successColor = Color.FromArgb(0, 222, 115);
        private Color errorColor = Color.FromArgb(250, 82, 82);
        private Color infoColor = Color.FromArgb(51, 154, 240);
        private Color warningColor = Color.FromArgb(255, 165, 0);
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
        public Color HeaderColor
        {
            get { return pnl_color.FillColor; }
            set { pnl_color.FillColor = value; }
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
                    pnl_color.FillColor = successColor;
                    pb_header.Image = Properties.Resources.successFlat;
                    break;

                case Icon.Error:
                    pnl_color.FillColor = errorColor;
                    pb_header.Image = Properties.Resources.errorFlat;
                    break;

                case Icon.Info:
                    pnl_color.FillColor = infoColor;
                    pb_header.Image = Properties.Resources.infoFlat;
                    break;

                case Icon.Warning:
                    pnl_color.FillColor = warningColor;
                    pb_header.Image = Properties.Resources.warningFlat;
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
    }
}
