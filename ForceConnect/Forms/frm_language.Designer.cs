namespace ForceConnect
{
    partial class frm_language
    {
        /// <summary>
        /// Required designer variable.
        /// </summary>
        private System.ComponentModel.IContainer components = null;

        /// <summary>
        /// Clean up any resources being used.
        /// </summary>
        /// <param name="disposing">true if managed resources should be disposed; otherwise, false.</param>
        protected override void Dispose(bool disposing)
        {
            if (disposing && (components != null))
            {
                components.Dispose();
            }
            base.Dispose(disposing);
        }

        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.components = new System.ComponentModel.Container();
            this.bf_form = new Guna.UI2.WinForms.Guna2BorderlessForm(this.components);
            this.btn_confirm = new Guna.UI2.WinForms.Guna2Button();
            this.gp_languageSelection = new System.Windows.Forms.GroupBox();
            this.pb_language = new Guna.UI2.WinForms.Guna2PictureBox();
            this.cb_selectLanguage = new Guna.UI2.WinForms.Guna2ComboBox();
            this.gp_languageSelection.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pb_language)).BeginInit();
            this.SuspendLayout();
            // 
            // bf_form
            // 
            this.bf_form.AnimateWindow = true;
            this.bf_form.AnimationInterval = 250;
            this.bf_form.BorderRadius = 15;
            this.bf_form.ContainerControl = this;
            this.bf_form.DockIndicatorColor = System.Drawing.Color.WhiteSmoke;
            this.bf_form.DockIndicatorTransparencyValue = 0.6D;
            this.bf_form.HasFormShadow = false;
            this.bf_form.ResizeForm = false;
            this.bf_form.ShadowColor = System.Drawing.Color.WhiteSmoke;
            this.bf_form.TransparentWhileDrag = true;
            // 
            // btn_confirm
            // 
            this.btn_confirm.Animated = true;
            this.btn_confirm.BackColor = System.Drawing.Color.Transparent;
            this.btn_confirm.BorderRadius = 5;
            this.btn_confirm.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_confirm.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_confirm.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_confirm.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_confirm.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_confirm.FillColor = System.Drawing.Color.WhiteSmoke;
            this.btn_confirm.Font = new System.Drawing.Font("Rubik", 7.8F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_confirm.ForeColor = System.Drawing.Color.Black;
            this.btn_confirm.Location = new System.Drawing.Point(209, 108);
            this.btn_confirm.Name = "btn_confirm";
            this.btn_confirm.Size = new System.Drawing.Size(131, 31);
            this.btn_confirm.TabIndex = 37;
            this.btn_confirm.Text = "Confirm";
            this.btn_confirm.Click += new System.EventHandler(this.btn_confirm_Click);
            // 
            // gp_languageSelection
            // 
            this.gp_languageSelection.Controls.Add(this.pb_language);
            this.gp_languageSelection.Controls.Add(this.cb_selectLanguage);
            this.gp_languageSelection.Controls.Add(this.btn_confirm);
            this.gp_languageSelection.FlatStyle = System.Windows.Forms.FlatStyle.Popup;
            this.gp_languageSelection.ForeColor = System.Drawing.Color.White;
            this.gp_languageSelection.Location = new System.Drawing.Point(29, 33);
            this.gp_languageSelection.Name = "gp_languageSelection";
            this.gp_languageSelection.Size = new System.Drawing.Size(368, 169);
            this.gp_languageSelection.TabIndex = 40;
            this.gp_languageSelection.TabStop = false;
            this.gp_languageSelection.Text = "Choose a language";
            // 
            // pb_language
            // 
            this.pb_language.BackColor = System.Drawing.Color.Transparent;
            this.pb_language.FillColor = System.Drawing.Color.Transparent;
            this.pb_language.Image = global::ForceConnect.Properties.Resources.english;
            this.pb_language.ImageRotate = 0F;
            this.pb_language.Location = new System.Drawing.Point(23, 108);
            this.pb_language.Name = "pb_language";
            this.pb_language.Size = new System.Drawing.Size(42, 31);
            this.pb_language.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
            this.pb_language.TabIndex = 45;
            this.pb_language.TabStop = false;
            this.pb_language.UseTransparentBackground = true;
            // 
            // cb_selectLanguage
            // 
            this.cb_selectLanguage.BackColor = System.Drawing.Color.Transparent;
            this.cb_selectLanguage.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(255)))), ((int)(((byte)(244)))), ((int)(((byte)(244)))));
            this.cb_selectLanguage.BorderRadius = 5;
            this.cb_selectLanguage.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_selectLanguage.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.cb_selectLanguage.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawFixed;
            this.cb_selectLanguage.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.cb_selectLanguage.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.cb_selectLanguage.FocusedColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectLanguage.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectLanguage.Font = new System.Drawing.Font("Segoe UI", 10F);
            this.cb_selectLanguage.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.cb_selectLanguage.ItemHeight = 40;
            this.cb_selectLanguage.Items.AddRange(new object[] {
            "English",
            "Persian",
            "Spain",
            "Germany"});
            this.cb_selectLanguage.Location = new System.Drawing.Point(23, 42);
            this.cb_selectLanguage.Name = "cb_selectLanguage";
            this.cb_selectLanguage.Size = new System.Drawing.Size(317, 46);
            this.cb_selectLanguage.StartIndex = 0;
            this.cb_selectLanguage.TabIndex = 44;
            this.cb_selectLanguage.TextOffset = new System.Drawing.Point(20, 0);
            this.cb_selectLanguage.SelectedIndexChanged += new System.EventHandler(this.cb_selectLanguage_SelectedIndexChanged);
            // 
            // frm_language
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.ClientSize = new System.Drawing.Size(425, 242);
            this.Controls.Add(this.gp_languageSelection);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Name = "frm_language";
            this.ShowIcon = false;
            this.ShowInTaskbar = false;
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "frm_language";
            this.gp_languageSelection.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.pb_language)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2BorderlessForm bf_form;
        private Guna.UI2.WinForms.Guna2Button btn_confirm;
        private System.Windows.Forms.GroupBox gp_languageSelection;
        private Guna.UI2.WinForms.Guna2ComboBox cb_selectLanguage;
        private Guna.UI2.WinForms.Guna2PictureBox pb_language;
    }
}