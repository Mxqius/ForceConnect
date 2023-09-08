using System.Windows.Forms;

namespace ForceConnect
{
    internal class FormManager
    {
        public static Form activeForm = null;
        public static Form openChildFormInPanel(Form childForm, Panel PnlActive)
        {
            // remember for use form setting GetInstance delete this if !
            if (activeForm != null)
                activeForm.Close();
            activeForm = childForm;
            childForm.TopLevel = false;
            childForm.FormBorderStyle = FormBorderStyle.None;
            childForm.Dock = DockStyle.Fill;
            PnlActive.Controls.Add(childForm);
            PnlActive.Tag = childForm;
            childForm.BringToFront();
            childForm.Show();
            return childForm;
        }
    }
}
