namespace ForceConnect
{
    partial class frm_settings
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
            this.guna2Panel1 = new Guna.UI2.WinForms.Guna2Panel();
            this.btn_updateSofware = new Guna.UI2.WinForms.Guna2Button();
            this.label1 = new System.Windows.Forms.Label();
            this.lbl_descriptionOption1 = new System.Windows.Forms.Label();
            this.cb_selectDns = new Guna.UI2.WinForms.Guna2ComboBox();
            this.cb_launchOnWindows = new Guna.UI2.WinForms.Guna2ToggleSwitch();
            this.lbl_launchOnWindows = new System.Windows.Forms.Label();
            this.lbl_automaticUpdate = new System.Windows.Forms.Label();
            this.cb_autoUpdate = new Guna.UI2.WinForms.Guna2ToggleSwitch();
            this.guna2Panel1.SuspendLayout();
            this.SuspendLayout();
            // 
            // guna2Panel1
            // 
            this.guna2Panel1.BorderRadius = 15;
            this.guna2Panel1.Controls.Add(this.cb_autoUpdate);
            this.guna2Panel1.Controls.Add(this.cb_launchOnWindows);
            this.guna2Panel1.Controls.Add(this.lbl_automaticUpdate);
            this.guna2Panel1.Controls.Add(this.btn_updateSofware);
            this.guna2Panel1.Controls.Add(this.lbl_launchOnWindows);
            this.guna2Panel1.Controls.Add(this.label1);
            this.guna2Panel1.Controls.Add(this.lbl_descriptionOption1);
            this.guna2Panel1.Controls.Add(this.cb_selectDns);
            this.guna2Panel1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2Panel1.Location = new System.Drawing.Point(218, 35);
            this.guna2Panel1.Name = "guna2Panel1";
            this.guna2Panel1.Size = new System.Drawing.Size(433, 339);
            this.guna2Panel1.TabIndex = 0;
            // 
            // btn_updateSofware
            // 
            this.btn_updateSofware.Animated = true;
            this.btn_updateSofware.BackColor = System.Drawing.Color.Transparent;
            this.btn_updateSofware.BorderRadius = 12;
            this.btn_updateSofware.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_updateSofware.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_updateSofware.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_updateSofware.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_updateSofware.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_updateSofware.FillColor = System.Drawing.Color.Transparent;
            this.btn_updateSofware.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_updateSofware.ForeColor = System.Drawing.Color.White;
            this.btn_updateSofware.Location = new System.Drawing.Point(21, 278);
            this.btn_updateSofware.Name = "btn_updateSofware";
            this.btn_updateSofware.Size = new System.Drawing.Size(163, 45);
            this.btn_updateSofware.TabIndex = 22;
            this.btn_updateSofware.Text = "Check Update";
            // 
            // label1
            // 
            this.label1.BackColor = System.Drawing.Color.Transparent;
            this.label1.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.label1.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label1.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.label1.Location = new System.Drawing.Point(31, 239);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(385, 36);
            this.label1.TabIndex = 21;
            this.label1.Text = "Download and Install Automaticly Updates";
            // 
            // lbl_descriptionOption1
            // 
            this.lbl_descriptionOption1.BackColor = System.Drawing.Color.Transparent;
            this.lbl_descriptionOption1.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_descriptionOption1.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_descriptionOption1.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.lbl_descriptionOption1.Location = new System.Drawing.Point(31, 143);
            this.lbl_descriptionOption1.Name = "lbl_descriptionOption1";
            this.lbl_descriptionOption1.Size = new System.Drawing.Size(399, 52);
            this.lbl_descriptionOption1.TabIndex = 21;
            this.lbl_descriptionOption1.Text = "Automatic execution of the program when\r\nthe system is turned on";
            // 
            // cb_selectDns
            // 
            this.cb_selectDns.BackColor = System.Drawing.Color.Transparent;
            this.cb_selectDns.BorderColor = System.Drawing.Color.Silver;
            this.cb_selectDns.BorderRadius = 10;
            this.cb_selectDns.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_selectDns.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawFixed;
            this.cb_selectDns.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.cb_selectDns.Enabled = false;
            this.cb_selectDns.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.cb_selectDns.FocusedColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectDns.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectDns.Font = new System.Drawing.Font("Segoe UI", 10F);
            this.cb_selectDns.ForeColor = System.Drawing.Color.Silver;
            this.cb_selectDns.ItemHeight = 40;
            this.cb_selectDns.Items.AddRange(new object[] {
            "Dark",
            "Light"});
            this.cb_selectDns.Location = new System.Drawing.Point(12, 22);
            this.cb_selectDns.Name = "cb_selectDns";
            this.cb_selectDns.Size = new System.Drawing.Size(408, 46);
            this.cb_selectDns.StartIndex = 0;
            this.cb_selectDns.TabIndex = 19;
            // 
            // cb_launchOnWindows
            // 
            this.cb_launchOnWindows.Animated = true;
            this.cb_launchOnWindows.AutoRoundedCorners = true;
            this.cb_launchOnWindows.BackColor = System.Drawing.Color.Transparent;
            this.cb_launchOnWindows.CheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_launchOnWindows.CheckedState.BorderRadius = 11;
            this.cb_launchOnWindows.CheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_launchOnWindows.CheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_launchOnWindows.CheckedState.InnerBorderRadius = 7;
            this.cb_launchOnWindows.CheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_launchOnWindows.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_launchOnWindows.Location = new System.Drawing.Point(24, 111);
            this.cb_launchOnWindows.Name = "cb_launchOnWindows";
            this.cb_launchOnWindows.Size = new System.Drawing.Size(55, 24);
            this.cb_launchOnWindows.TabIndex = 23;
            this.cb_launchOnWindows.UncheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_launchOnWindows.UncheckedState.BorderRadius = 11;
            this.cb_launchOnWindows.UncheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_launchOnWindows.UncheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_launchOnWindows.UncheckedState.InnerBorderRadius = 7;
            this.cb_launchOnWindows.UncheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_launchOnWindows.CheckedChanged += new System.EventHandler(this.cb_launchOnWindows_CheckedChanged);
            // 
            // lbl_launchOnWindows
            // 
            this.lbl_launchOnWindows.BackColor = System.Drawing.Color.Transparent;
            this.lbl_launchOnWindows.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_launchOnWindows.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_launchOnWindows.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_launchOnWindows.Location = new System.Drawing.Point(85, 111);
            this.lbl_launchOnWindows.Name = "lbl_launchOnWindows";
            this.lbl_launchOnWindows.Size = new System.Drawing.Size(239, 28);
            this.lbl_launchOnWindows.TabIndex = 21;
            this.lbl_launchOnWindows.Text = "Launch On Windows";
            this.lbl_launchOnWindows.Click += new System.EventHandler(this.cb_launchOnWindows_CheckedChanged);
            // 
            // lbl_automaticUpdate
            // 
            this.lbl_automaticUpdate.BackColor = System.Drawing.Color.Transparent;
            this.lbl_automaticUpdate.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_automaticUpdate.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_automaticUpdate.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_automaticUpdate.Location = new System.Drawing.Point(85, 202);
            this.lbl_automaticUpdate.Name = "lbl_automaticUpdate";
            this.lbl_automaticUpdate.Size = new System.Drawing.Size(239, 28);
            this.lbl_automaticUpdate.TabIndex = 21;
            this.lbl_automaticUpdate.Text = "Automatic Update";
            // 
            // cb_autoUpdate
            // 
            this.cb_autoUpdate.Animated = true;
            this.cb_autoUpdate.AutoRoundedCorners = true;
            this.cb_autoUpdate.BackColor = System.Drawing.Color.Transparent;
            this.cb_autoUpdate.CheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_autoUpdate.CheckedState.BorderRadius = 11;
            this.cb_autoUpdate.CheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_autoUpdate.CheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_autoUpdate.CheckedState.InnerBorderRadius = 7;
            this.cb_autoUpdate.CheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_autoUpdate.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_autoUpdate.Location = new System.Drawing.Point(24, 202);
            this.cb_autoUpdate.Name = "cb_autoUpdate";
            this.cb_autoUpdate.Size = new System.Drawing.Size(55, 24);
            this.cb_autoUpdate.TabIndex = 23;
            this.cb_autoUpdate.UncheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_autoUpdate.UncheckedState.BorderRadius = 11;
            this.cb_autoUpdate.UncheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_autoUpdate.UncheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_autoUpdate.UncheckedState.InnerBorderRadius = 7;
            this.cb_autoUpdate.UncheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_autoUpdate.CheckedChanged += new System.EventHandler(this.cb_launchOnWindows_CheckedChanged);
            // 
            // frm_settings
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.ClientSize = new System.Drawing.Size(884, 408);
            this.Controls.Add(this.guna2Panel1);
            this.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Name = "frm_settings";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Settings";
            this.Load += new System.EventHandler(this.frm_setttings_Load);
            this.guna2Panel1.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2Panel guna2Panel1;
        private Guna.UI2.WinForms.Guna2ComboBox cb_selectDns;
        private System.Windows.Forms.Label lbl_descriptionOption1;
        private System.Windows.Forms.Label label1;
        private Guna.UI2.WinForms.Guna2Button btn_updateSofware;
        private Guna.UI2.WinForms.Guna2ToggleSwitch cb_launchOnWindows;
        private System.Windows.Forms.Label lbl_launchOnWindows;
        private Guna.UI2.WinForms.Guna2ToggleSwitch cb_autoUpdate;
        private System.Windows.Forms.Label lbl_automaticUpdate;
    }
}