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
            this.btn_networkInformation = new Guna.UI2.WinForms.Guna2Button();
            this.cb_autoUpdate = new Guna.UI2.WinForms.Guna2ToggleSwitch();
            this.ts_discordRPC = new Guna.UI2.WinForms.Guna2ToggleSwitch();
            this.cb_minimizeInTray = new Guna.UI2.WinForms.Guna2ToggleSwitch();
            this.cb_launchOnWindows = new Guna.UI2.WinForms.Guna2ToggleSwitch();
            this.lbl_theme = new System.Windows.Forms.Label();
            this.lbl_automaticUpdate = new System.Windows.Forms.Label();
            this.lbl_minimizeInTray = new System.Windows.Forms.Label();
            this.btn_updateSofware = new Guna.UI2.WinForms.Guna2Button();
            this.lbl_discordDescription = new System.Windows.Forms.Label();
            this.lbl_launchOnWindows = new System.Windows.Forms.Label();
            this.lbl_minimizeDescription = new System.Windows.Forms.Label();
            this.lbl_descriptionOption2 = new System.Windows.Forms.Label();
            this.lbl_descriptionOption1 = new System.Windows.Forms.Label();
            this.guna2Panel1 = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Panel2 = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Panel3 = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Panel4 = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Panel1.SuspendLayout();
            this.guna2Panel2.SuspendLayout();
            this.guna2Panel3.SuspendLayout();
            this.guna2Panel4.SuspendLayout();
            this.SuspendLayout();
            // 
            // btn_networkInformation
            // 
            this.btn_networkInformation.Animated = true;
            this.btn_networkInformation.BackColor = System.Drawing.Color.Transparent;
            this.btn_networkInformation.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.btn_networkInformation.BorderRadius = 10;
            this.btn_networkInformation.BorderThickness = 2;
            this.btn_networkInformation.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_networkInformation.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_networkInformation.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_networkInformation.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_networkInformation.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_networkInformation.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.btn_networkInformation.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_networkInformation.ForeColor = System.Drawing.Color.White;
            this.btn_networkInformation.Image = global::ForceConnect.Properties.Resources.network_panel;
            this.btn_networkInformation.ImageOffset = new System.Drawing.Point(-2, 0);
            this.btn_networkInformation.Location = new System.Drawing.Point(448, 323);
            this.btn_networkInformation.Name = "btn_networkInformation";
            this.btn_networkInformation.Size = new System.Drawing.Size(399, 41);
            this.btn_networkInformation.TabIndex = 26;
            this.btn_networkInformation.Text = "Network Information";
            this.btn_networkInformation.Click += new System.EventHandler(this.btn_networkInformation_Click);
            // 
            // cb_autoUpdate
            // 
            this.cb_autoUpdate.Animated = true;
            this.cb_autoUpdate.AutoRoundedCorners = true;
            this.cb_autoUpdate.BackColor = System.Drawing.Color.Transparent;
            this.cb_autoUpdate.CheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.cb_autoUpdate.CheckedState.BorderRadius = 11;
            this.cb_autoUpdate.CheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.cb_autoUpdate.CheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_autoUpdate.CheckedState.InnerBorderRadius = 7;
            this.cb_autoUpdate.CheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_autoUpdate.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_autoUpdate.Location = new System.Drawing.Point(315, 30);
            this.cb_autoUpdate.Name = "cb_autoUpdate";
            this.cb_autoUpdate.Size = new System.Drawing.Size(55, 24);
            this.cb_autoUpdate.TabIndex = 23;
            this.cb_autoUpdate.UncheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_autoUpdate.UncheckedState.BorderRadius = 11;
            this.cb_autoUpdate.UncheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_autoUpdate.UncheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_autoUpdate.UncheckedState.InnerBorderRadius = 7;
            this.cb_autoUpdate.UncheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_autoUpdate.CheckedChanged += new System.EventHandler(this.cb_autoUpdate_CheckedChanged);
            // 
            // ts_discordRPC
            // 
            this.ts_discordRPC.Animated = true;
            this.ts_discordRPC.AutoRoundedCorners = true;
            this.ts_discordRPC.BackColor = System.Drawing.Color.Transparent;
            this.ts_discordRPC.Checked = true;
            this.ts_discordRPC.CheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.ts_discordRPC.CheckedState.BorderRadius = 11;
            this.ts_discordRPC.CheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.ts_discordRPC.CheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.ts_discordRPC.CheckedState.InnerBorderRadius = 7;
            this.ts_discordRPC.CheckedState.InnerColor = System.Drawing.Color.White;
            this.ts_discordRPC.Cursor = System.Windows.Forms.Cursors.Hand;
            this.ts_discordRPC.Location = new System.Drawing.Point(316, 30);
            this.ts_discordRPC.Name = "ts_discordRPC";
            this.ts_discordRPC.Size = new System.Drawing.Size(55, 24);
            this.ts_discordRPC.TabIndex = 23;
            this.ts_discordRPC.UncheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.ts_discordRPC.UncheckedState.BorderRadius = 11;
            this.ts_discordRPC.UncheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.ts_discordRPC.UncheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.ts_discordRPC.UncheckedState.InnerBorderRadius = 7;
            this.ts_discordRPC.UncheckedState.InnerColor = System.Drawing.Color.White;
            this.ts_discordRPC.CheckedChanged += new System.EventHandler(this.ts_discordRPC_CheckedChanged);
            // 
            // cb_minimizeInTray
            // 
            this.cb_minimizeInTray.Animated = true;
            this.cb_minimizeInTray.AutoRoundedCorners = true;
            this.cb_minimizeInTray.BackColor = System.Drawing.Color.Transparent;
            this.cb_minimizeInTray.CheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.cb_minimizeInTray.CheckedState.BorderRadius = 11;
            this.cb_minimizeInTray.CheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.cb_minimizeInTray.CheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_minimizeInTray.CheckedState.InnerBorderRadius = 7;
            this.cb_minimizeInTray.CheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_minimizeInTray.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_minimizeInTray.Location = new System.Drawing.Point(316, 29);
            this.cb_minimizeInTray.Name = "cb_minimizeInTray";
            this.cb_minimizeInTray.Size = new System.Drawing.Size(55, 24);
            this.cb_minimizeInTray.TabIndex = 23;
            this.cb_minimizeInTray.UncheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_minimizeInTray.UncheckedState.BorderRadius = 11;
            this.cb_minimizeInTray.UncheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(125)))), ((int)(((byte)(137)))), ((int)(((byte)(149)))));
            this.cb_minimizeInTray.UncheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_minimizeInTray.UncheckedState.InnerBorderRadius = 7;
            this.cb_minimizeInTray.UncheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_minimizeInTray.CheckedChanged += new System.EventHandler(this.cb_minimizeInTray_CheckedChanged);
            // 
            // cb_launchOnWindows
            // 
            this.cb_launchOnWindows.Animated = true;
            this.cb_launchOnWindows.AutoRoundedCorners = true;
            this.cb_launchOnWindows.BackColor = System.Drawing.Color.Transparent;
            this.cb_launchOnWindows.CheckedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.cb_launchOnWindows.CheckedState.BorderRadius = 11;
            this.cb_launchOnWindows.CheckedState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(11)))), ((int)(((byte)(96)))), ((int)(((byte)(176)))));
            this.cb_launchOnWindows.CheckedState.InnerBorderColor = System.Drawing.Color.White;
            this.cb_launchOnWindows.CheckedState.InnerBorderRadius = 7;
            this.cb_launchOnWindows.CheckedState.InnerColor = System.Drawing.Color.White;
            this.cb_launchOnWindows.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_launchOnWindows.Location = new System.Drawing.Point(315, 29);
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
            // lbl_theme
            // 
            this.lbl_theme.AutoSize = true;
            this.lbl_theme.BackColor = System.Drawing.Color.Transparent;
            this.lbl_theme.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_theme.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_theme.Font = new System.Drawing.Font("Rubik SemiBold", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_theme.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_theme.Location = new System.Drawing.Point(39, 31);
            this.lbl_theme.Name = "lbl_theme";
            this.lbl_theme.Size = new System.Drawing.Size(118, 23);
            this.lbl_theme.TabIndex = 21;
            this.lbl_theme.Text = "Discord RPC";
            this.lbl_theme.Click += new System.EventHandler(this.lbl_theme_Click);
            // 
            // lbl_automaticUpdate
            // 
            this.lbl_automaticUpdate.AutoSize = true;
            this.lbl_automaticUpdate.BackColor = System.Drawing.Color.Transparent;
            this.lbl_automaticUpdate.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_automaticUpdate.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_automaticUpdate.Font = new System.Drawing.Font("Rubik SemiBold", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_automaticUpdate.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_automaticUpdate.Location = new System.Drawing.Point(28, 31);
            this.lbl_automaticUpdate.Name = "lbl_automaticUpdate";
            this.lbl_automaticUpdate.Size = new System.Drawing.Size(170, 23);
            this.lbl_automaticUpdate.TabIndex = 21;
            this.lbl_automaticUpdate.Text = "Automatic Update";
            this.lbl_automaticUpdate.Click += new System.EventHandler(this.lbl_automaticUpdate_Click);
            // 
            // lbl_minimizeInTray
            // 
            this.lbl_minimizeInTray.AutoSize = true;
            this.lbl_minimizeInTray.BackColor = System.Drawing.Color.Transparent;
            this.lbl_minimizeInTray.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_minimizeInTray.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_minimizeInTray.Font = new System.Drawing.Font("Rubik SemiBold", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_minimizeInTray.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_minimizeInTray.Location = new System.Drawing.Point(35, 30);
            this.lbl_minimizeInTray.Name = "lbl_minimizeInTray";
            this.lbl_minimizeInTray.Size = new System.Drawing.Size(149, 23);
            this.lbl_minimizeInTray.TabIndex = 21;
            this.lbl_minimizeInTray.Text = "Minimize in tray";
            this.lbl_minimizeInTray.Click += new System.EventHandler(this.lbl_minimizeInTray_Click);
            // 
            // btn_updateSofware
            // 
            this.btn_updateSofware.Animated = true;
            this.btn_updateSofware.AnimatedGIF = true;
            this.btn_updateSofware.BackColor = System.Drawing.Color.Transparent;
            this.btn_updateSofware.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.btn_updateSofware.BorderRadius = 10;
            this.btn_updateSofware.BorderThickness = 2;
            this.btn_updateSofware.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_updateSofware.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_updateSofware.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_updateSofware.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_updateSofware.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_updateSofware.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.btn_updateSofware.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_updateSofware.ForeColor = System.Drawing.Color.White;
            this.btn_updateSofware.Image = global::ForceConnect.Properties.Resources.update;
            this.btn_updateSofware.Location = new System.Drawing.Point(37, 323);
            this.btn_updateSofware.Name = "btn_updateSofware";
            this.btn_updateSofware.Size = new System.Drawing.Size(399, 41);
            this.btn_updateSofware.TabIndex = 22;
            this.btn_updateSofware.Text = "Check Updates";
            this.btn_updateSofware.Click += new System.EventHandler(this.btn_updateSofware_Click);
            // 
            // lbl_discordDescription
            // 
            this.lbl_discordDescription.BackColor = System.Drawing.Color.Transparent;
            this.lbl_discordDescription.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_discordDescription.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_discordDescription.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.lbl_discordDescription.Location = new System.Drawing.Point(39, 70);
            this.lbl_discordDescription.Name = "lbl_discordDescription";
            this.lbl_discordDescription.Size = new System.Drawing.Size(345, 42);
            this.lbl_discordDescription.TabIndex = 21;
            this.lbl_discordDescription.Text = "Show your active service in your activity";
            // 
            // lbl_launchOnWindows
            // 
            this.lbl_launchOnWindows.AutoSize = true;
            this.lbl_launchOnWindows.BackColor = System.Drawing.Color.Transparent;
            this.lbl_launchOnWindows.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_launchOnWindows.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_launchOnWindows.Font = new System.Drawing.Font("Rubik SemiBold", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_launchOnWindows.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_launchOnWindows.Location = new System.Drawing.Point(28, 30);
            this.lbl_launchOnWindows.Name = "lbl_launchOnWindows";
            this.lbl_launchOnWindows.Size = new System.Drawing.Size(185, 23);
            this.lbl_launchOnWindows.TabIndex = 21;
            this.lbl_launchOnWindows.Text = "Launch On Windows";
            this.lbl_launchOnWindows.Click += new System.EventHandler(this.lbl_launchOnWindows_Click);
            // 
            // lbl_minimizeDescription
            // 
            this.lbl_minimizeDescription.BackColor = System.Drawing.Color.Transparent;
            this.lbl_minimizeDescription.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_minimizeDescription.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_minimizeDescription.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.lbl_minimizeDescription.Location = new System.Drawing.Point(35, 67);
            this.lbl_minimizeDescription.Name = "lbl_minimizeDescription";
            this.lbl_minimizeDescription.Size = new System.Drawing.Size(349, 52);
            this.lbl_minimizeDescription.TabIndex = 21;
            this.lbl_minimizeDescription.Text = "The program move to tray in background when you close the program";
            // 
            // lbl_descriptionOption2
            // 
            this.lbl_descriptionOption2.BackColor = System.Drawing.Color.Transparent;
            this.lbl_descriptionOption2.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_descriptionOption2.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_descriptionOption2.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.lbl_descriptionOption2.Location = new System.Drawing.Point(28, 70);
            this.lbl_descriptionOption2.Name = "lbl_descriptionOption2";
            this.lbl_descriptionOption2.Size = new System.Drawing.Size(329, 36);
            this.lbl_descriptionOption2.TabIndex = 21;
            this.lbl_descriptionOption2.Text = "Download and Install automaticly updates";
            // 
            // lbl_descriptionOption1
            // 
            this.lbl_descriptionOption1.BackColor = System.Drawing.Color.Transparent;
            this.lbl_descriptionOption1.FlatStyle = System.Windows.Forms.FlatStyle.Flat;
            this.lbl_descriptionOption1.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_descriptionOption1.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.lbl_descriptionOption1.Location = new System.Drawing.Point(28, 67);
            this.lbl_descriptionOption1.Name = "lbl_descriptionOption1";
            this.lbl_descriptionOption1.Size = new System.Drawing.Size(329, 45);
            this.lbl_descriptionOption1.TabIndex = 21;
            this.lbl_descriptionOption1.Text = "Automatic execution of the program when\r\nthe system is turned on";
            // 
            // guna2Panel1
            // 
            this.guna2Panel1.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.guna2Panel1.BorderRadius = 10;
            this.guna2Panel1.BorderThickness = 2;
            this.guna2Panel1.Controls.Add(this.cb_launchOnWindows);
            this.guna2Panel1.Controls.Add(this.lbl_descriptionOption1);
            this.guna2Panel1.Controls.Add(this.lbl_launchOnWindows);
            this.guna2Panel1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2Panel1.Location = new System.Drawing.Point(37, 44);
            this.guna2Panel1.Name = "guna2Panel1";
            this.guna2Panel1.Size = new System.Drawing.Size(399, 128);
            this.guna2Panel1.TabIndex = 0;
            // 
            // guna2Panel2
            // 
            this.guna2Panel2.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.guna2Panel2.BorderRadius = 10;
            this.guna2Panel2.BorderThickness = 2;
            this.guna2Panel2.Controls.Add(this.cb_autoUpdate);
            this.guna2Panel2.Controls.Add(this.lbl_descriptionOption2);
            this.guna2Panel2.Controls.Add(this.lbl_automaticUpdate);
            this.guna2Panel2.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2Panel2.Location = new System.Drawing.Point(37, 182);
            this.guna2Panel2.Name = "guna2Panel2";
            this.guna2Panel2.Size = new System.Drawing.Size(399, 128);
            this.guna2Panel2.TabIndex = 0;
            // 
            // guna2Panel3
            // 
            this.guna2Panel3.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.guna2Panel3.BorderRadius = 10;
            this.guna2Panel3.BorderThickness = 2;
            this.guna2Panel3.Controls.Add(this.cb_minimizeInTray);
            this.guna2Panel3.Controls.Add(this.lbl_minimizeInTray);
            this.guna2Panel3.Controls.Add(this.lbl_minimizeDescription);
            this.guna2Panel3.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2Panel3.Location = new System.Drawing.Point(448, 44);
            this.guna2Panel3.Name = "guna2Panel3";
            this.guna2Panel3.Size = new System.Drawing.Size(399, 128);
            this.guna2Panel3.TabIndex = 0;
            // 
            // guna2Panel4
            // 
            this.guna2Panel4.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.guna2Panel4.BorderRadius = 10;
            this.guna2Panel4.BorderThickness = 2;
            this.guna2Panel4.Controls.Add(this.ts_discordRPC);
            this.guna2Panel4.Controls.Add(this.lbl_theme);
            this.guna2Panel4.Controls.Add(this.lbl_discordDescription);
            this.guna2Panel4.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2Panel4.Location = new System.Drawing.Point(448, 182);
            this.guna2Panel4.Name = "guna2Panel4";
            this.guna2Panel4.Size = new System.Drawing.Size(399, 128);
            this.guna2Panel4.TabIndex = 0;
            // 
            // frm_settings
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.ClientSize = new System.Drawing.Size(884, 408);
            this.Controls.Add(this.btn_networkInformation);
            this.Controls.Add(this.guna2Panel2);
            this.Controls.Add(this.guna2Panel4);
            this.Controls.Add(this.guna2Panel3);
            this.Controls.Add(this.guna2Panel1);
            this.Controls.Add(this.btn_updateSofware);
            this.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Name = "frm_settings";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Settings";
            this.Load += new System.EventHandler(this.frm_setttings_Load);
            this.guna2Panel1.ResumeLayout(false);
            this.guna2Panel1.PerformLayout();
            this.guna2Panel2.ResumeLayout(false);
            this.guna2Panel2.PerformLayout();
            this.guna2Panel3.ResumeLayout(false);
            this.guna2Panel3.PerformLayout();
            this.guna2Panel4.ResumeLayout(false);
            this.guna2Panel4.PerformLayout();
            this.ResumeLayout(false);

        }

        #endregion
        private System.Windows.Forms.Label lbl_descriptionOption1;
        private System.Windows.Forms.Label lbl_descriptionOption2;
        private Guna.UI2.WinForms.Guna2ToggleSwitch cb_launchOnWindows;
        private System.Windows.Forms.Label lbl_launchOnWindows;
        private Guna.UI2.WinForms.Guna2ToggleSwitch cb_autoUpdate;
        private System.Windows.Forms.Label lbl_automaticUpdate;
        private Guna.UI2.WinForms.Guna2ToggleSwitch cb_minimizeInTray;
        private System.Windows.Forms.Label lbl_minimizeInTray;
        private System.Windows.Forms.Label lbl_minimizeDescription;
        private Guna.UI2.WinForms.Guna2ToggleSwitch ts_discordRPC;
        private System.Windows.Forms.Label lbl_theme;
        private System.Windows.Forms.Label lbl_discordDescription;
        private Guna.UI2.WinForms.Guna2Button btn_networkInformation;
        private Guna.UI2.WinForms.Guna2Button btn_updateSofware;
        private Guna.UI2.WinForms.Guna2Panel guna2Panel1;
        private Guna.UI2.WinForms.Guna2Panel guna2Panel2;
        private Guna.UI2.WinForms.Guna2Panel guna2Panel3;
        private Guna.UI2.WinForms.Guna2Panel guna2Panel4;
    }
}