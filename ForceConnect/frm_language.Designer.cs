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
            this.btn_germanyLanguage = new Guna.UI2.WinForms.Guna2Button();
            this.btn_faLanguage = new Guna.UI2.WinForms.Guna2Button();
            this.btn_spainLanguage = new Guna.UI2.WinForms.Guna2Button();
            this.btn_enLanguage = new Guna.UI2.WinForms.Guna2Button();
            this.gp_languageSelection.SuspendLayout();
            this.SuspendLayout();
            // 
            // bf_form
            // 
            this.bf_form.AnimateWindow = true;
            this.bf_form.AnimationInterval = 250;
            this.bf_form.BorderRadius = 15;
            this.bf_form.ContainerControl = this;
            this.bf_form.DockIndicatorTransparencyValue = 0.6D;
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
            this.btn_confirm.Location = new System.Drawing.Point(166, 165);
            this.btn_confirm.Name = "btn_confirm";
            this.btn_confirm.Size = new System.Drawing.Size(131, 31);
            this.btn_confirm.TabIndex = 37;
            this.btn_confirm.Text = "Confirm";
            this.btn_confirm.Click += new System.EventHandler(this.btn_confirm_Click);
            // 
            // gp_languageSelection
            // 
            this.gp_languageSelection.Controls.Add(this.btn_germanyLanguage);
            this.gp_languageSelection.Controls.Add(this.btn_confirm);
            this.gp_languageSelection.Controls.Add(this.btn_faLanguage);
            this.gp_languageSelection.Controls.Add(this.btn_spainLanguage);
            this.gp_languageSelection.Controls.Add(this.btn_enLanguage);
            this.gp_languageSelection.FlatStyle = System.Windows.Forms.FlatStyle.Popup;
            this.gp_languageSelection.ForeColor = System.Drawing.Color.White;
            this.gp_languageSelection.Location = new System.Drawing.Point(50, 53);
            this.gp_languageSelection.Name = "gp_languageSelection";
            this.gp_languageSelection.Size = new System.Drawing.Size(319, 219);
            this.gp_languageSelection.TabIndex = 40;
            this.gp_languageSelection.TabStop = false;
            this.gp_languageSelection.Text = "Choose a language:";
            // 
            // btn_germanyLanguage
            // 
            this.btn_germanyLanguage.Animated = true;
            this.btn_germanyLanguage.AnimatedGIF = true;
            this.btn_germanyLanguage.BackColor = System.Drawing.Color.Transparent;
            this.btn_germanyLanguage.BorderColor = System.Drawing.Color.Transparent;
            this.btn_germanyLanguage.BorderRadius = 5;
            this.btn_germanyLanguage.BorderThickness = 1;
            this.btn_germanyLanguage.ButtonMode = Guna.UI2.WinForms.Enums.ButtonMode.RadioButton;
            this.btn_germanyLanguage.CheckedState.BorderColor = System.Drawing.Color.White;
            this.btn_germanyLanguage.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_germanyLanguage.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_germanyLanguage.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_germanyLanguage.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_germanyLanguage.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_germanyLanguage.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(54)))), ((int)(((byte)(54)))), ((int)(((byte)(59)))));
            this.btn_germanyLanguage.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_germanyLanguage.ForeColor = System.Drawing.Color.White;
            this.btn_germanyLanguage.Image = global::ForceConnect.Properties.Resources.germany;
            this.btn_germanyLanguage.ImageOffset = new System.Drawing.Point(-5, 0);
            this.btn_germanyLanguage.ImageSize = new System.Drawing.Size(25, 25);
            this.btn_germanyLanguage.Location = new System.Drawing.Point(166, 95);
            this.btn_germanyLanguage.Name = "btn_germanyLanguage";
            this.btn_germanyLanguage.Size = new System.Drawing.Size(131, 36);
            this.btn_germanyLanguage.TabIndex = 40;
            this.btn_germanyLanguage.Text = "GRMANY";
            // 
            // btn_faLanguage
            // 
            this.btn_faLanguage.Animated = true;
            this.btn_faLanguage.AnimatedGIF = true;
            this.btn_faLanguage.BackColor = System.Drawing.Color.Transparent;
            this.btn_faLanguage.BorderColor = System.Drawing.Color.Transparent;
            this.btn_faLanguage.BorderRadius = 5;
            this.btn_faLanguage.BorderThickness = 1;
            this.btn_faLanguage.ButtonMode = Guna.UI2.WinForms.Enums.ButtonMode.RadioButton;
            this.btn_faLanguage.CheckedState.BorderColor = System.Drawing.Color.White;
            this.btn_faLanguage.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_faLanguage.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_faLanguage.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_faLanguage.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_faLanguage.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_faLanguage.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(54)))), ((int)(((byte)(54)))), ((int)(((byte)(59)))));
            this.btn_faLanguage.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_faLanguage.ForeColor = System.Drawing.Color.White;
            this.btn_faLanguage.Image = global::ForceConnect.Properties.Resources.iran;
            this.btn_faLanguage.ImageOffset = new System.Drawing.Point(-5, 0);
            this.btn_faLanguage.ImageSize = new System.Drawing.Size(25, 25);
            this.btn_faLanguage.Location = new System.Drawing.Point(166, 42);
            this.btn_faLanguage.Name = "btn_faLanguage";
            this.btn_faLanguage.Size = new System.Drawing.Size(131, 36);
            this.btn_faLanguage.TabIndex = 41;
            this.btn_faLanguage.Text = "FA";
            // 
            // btn_spainLanguage
            // 
            this.btn_spainLanguage.Animated = true;
            this.btn_spainLanguage.AnimatedGIF = true;
            this.btn_spainLanguage.BackColor = System.Drawing.Color.Transparent;
            this.btn_spainLanguage.BorderColor = System.Drawing.Color.Transparent;
            this.btn_spainLanguage.BorderRadius = 5;
            this.btn_spainLanguage.BorderThickness = 1;
            this.btn_spainLanguage.ButtonMode = Guna.UI2.WinForms.Enums.ButtonMode.RadioButton;
            this.btn_spainLanguage.CheckedState.BorderColor = System.Drawing.Color.White;
            this.btn_spainLanguage.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_spainLanguage.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_spainLanguage.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_spainLanguage.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_spainLanguage.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_spainLanguage.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(54)))), ((int)(((byte)(54)))), ((int)(((byte)(59)))));
            this.btn_spainLanguage.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_spainLanguage.ForeColor = System.Drawing.Color.White;
            this.btn_spainLanguage.Image = global::ForceConnect.Properties.Resources.spain;
            this.btn_spainLanguage.ImageOffset = new System.Drawing.Point(-5, 0);
            this.btn_spainLanguage.ImageSize = new System.Drawing.Size(25, 25);
            this.btn_spainLanguage.Location = new System.Drawing.Point(20, 95);
            this.btn_spainLanguage.Name = "btn_spainLanguage";
            this.btn_spainLanguage.Size = new System.Drawing.Size(131, 36);
            this.btn_spainLanguage.TabIndex = 42;
            this.btn_spainLanguage.Text = "SPAIN";
            // 
            // btn_enLanguage
            // 
            this.btn_enLanguage.Animated = true;
            this.btn_enLanguage.AnimatedGIF = true;
            this.btn_enLanguage.BackColor = System.Drawing.Color.Transparent;
            this.btn_enLanguage.BorderColor = System.Drawing.Color.Transparent;
            this.btn_enLanguage.BorderRadius = 5;
            this.btn_enLanguage.BorderThickness = 1;
            this.btn_enLanguage.ButtonMode = Guna.UI2.WinForms.Enums.ButtonMode.RadioButton;
            this.btn_enLanguage.CheckedState.BorderColor = System.Drawing.Color.White;
            this.btn_enLanguage.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_enLanguage.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_enLanguage.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_enLanguage.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_enLanguage.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_enLanguage.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(54)))), ((int)(((byte)(54)))), ((int)(((byte)(59)))));
            this.btn_enLanguage.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_enLanguage.ForeColor = System.Drawing.Color.White;
            this.btn_enLanguage.Image = global::ForceConnect.Properties.Resources.united_kingdom;
            this.btn_enLanguage.ImageOffset = new System.Drawing.Point(-5, 0);
            this.btn_enLanguage.ImageSize = new System.Drawing.Size(25, 25);
            this.btn_enLanguage.Location = new System.Drawing.Point(20, 42);
            this.btn_enLanguage.Name = "btn_enLanguage";
            this.btn_enLanguage.Size = new System.Drawing.Size(131, 36);
            this.btn_enLanguage.TabIndex = 43;
            this.btn_enLanguage.Text = "EN";
            // 
            // frm_language
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.ClientSize = new System.Drawing.Size(425, 332);
            this.Controls.Add(this.gp_languageSelection);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Name = "frm_language";
            this.ShowIcon = false;
            this.ShowInTaskbar = false;
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "frm_language";
            this.gp_languageSelection.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2BorderlessForm bf_form;
        private Guna.UI2.WinForms.Guna2Button btn_confirm;
        private System.Windows.Forms.GroupBox gp_languageSelection;
        private Guna.UI2.WinForms.Guna2Button btn_germanyLanguage;
        private Guna.UI2.WinForms.Guna2Button btn_faLanguage;
        private Guna.UI2.WinForms.Guna2Button btn_spainLanguage;
        private Guna.UI2.WinForms.Guna2Button btn_enLanguage;
    }
}