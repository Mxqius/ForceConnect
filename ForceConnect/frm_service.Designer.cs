namespace ForceConnect
{
    partial class frm_service
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
            this.pnl_services = new Guna.UI2.WinForms.Guna2Panel();
            this.txt_address2 = new Guna.UI2.WinForms.Guna2TextBox();
            this.pb_latencyPicture = new Guna.UI2.WinForms.Guna2PictureBox();
            this.txt_address1 = new Guna.UI2.WinForms.Guna2TextBox();
            this.txt_name = new Guna.UI2.WinForms.Guna2TextBox();
            this.label3 = new System.Windows.Forms.Label();
            this.label1 = new System.Windows.Forms.Label();
            this.lbl_ping = new System.Windows.Forms.Label();
            this.btn_ping = new Guna.UI2.WinForms.Guna2Button();
            this.btn_cancel = new Guna.UI2.WinForms.Guna2Button();
            this.btn_addService = new Guna.UI2.WinForms.Guna2Button();
            this.bf_form = new Guna.UI2.WinForms.Guna2BorderlessForm(this.components);
            this.pnl_latencySection = new Guna.UI2.WinForms.Guna2Panel();
            this.pnl_services.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).BeginInit();
            this.pnl_latencySection.SuspendLayout();
            this.SuspendLayout();
            // 
            // pnl_services
            // 
            this.pnl_services.BackColor = System.Drawing.Color.Transparent;
            this.pnl_services.Controls.Add(this.pnl_latencySection);
            this.pnl_services.Controls.Add(this.txt_address2);
            this.pnl_services.Controls.Add(this.txt_address1);
            this.pnl_services.Controls.Add(this.txt_name);
            this.pnl_services.Controls.Add(this.label3);
            this.pnl_services.Controls.Add(this.label1);
            this.pnl_services.Controls.Add(this.btn_cancel);
            this.pnl_services.Controls.Add(this.btn_addService);
            this.pnl_services.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pnl_services.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_services.Location = new System.Drawing.Point(0, 0);
            this.pnl_services.Name = "pnl_services";
            this.pnl_services.Size = new System.Drawing.Size(422, 406);
            this.pnl_services.TabIndex = 11;
            // 
            // txt_address2
            // 
            this.txt_address2.Animated = true;
            this.txt_address2.BackColor = System.Drawing.Color.Transparent;
            this.txt_address2.BorderRadius = 5;
            this.txt_address2.Cursor = System.Windows.Forms.Cursors.IBeam;
            this.txt_address2.DefaultText = "";
            this.txt_address2.DisabledState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(208)))), ((int)(((byte)(208)))), ((int)(((byte)(208)))));
            this.txt_address2.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(226)))), ((int)(((byte)(226)))), ((int)(((byte)(226)))));
            this.txt_address2.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(138)))), ((int)(((byte)(138)))), ((int)(((byte)(138)))));
            this.txt_address2.DisabledState.PlaceholderForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(138)))), ((int)(((byte)(138)))), ((int)(((byte)(138)))));
            this.txt_address2.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.txt_address2.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.txt_address2.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.txt_address2.HoverState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.txt_address2.Location = new System.Drawing.Point(55, 198);
            this.txt_address2.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
            this.txt_address2.Name = "txt_address2";
            this.txt_address2.PasswordChar = '\0';
            this.txt_address2.PlaceholderForeColor = System.Drawing.Color.Gray;
            this.txt_address2.PlaceholderText = "Address 2";
            this.txt_address2.SelectedText = "";
            this.txt_address2.Size = new System.Drawing.Size(319, 38);
            this.txt_address2.TabIndex = 2;
            this.txt_address2.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.onlyAddress);
            // 
            // pb_latencyPicture
            // 
            this.pb_latencyPicture.BackColor = System.Drawing.Color.Transparent;
            this.pb_latencyPicture.FillColor = System.Drawing.Color.Transparent;
            this.pb_latencyPicture.Image = global::ForceConnect.Properties.Resources.signalGreen;
            this.pb_latencyPicture.ImageRotate = 0F;
            this.pb_latencyPicture.Location = new System.Drawing.Point(11, 15);
            this.pb_latencyPicture.Name = "pb_latencyPicture";
            this.pb_latencyPicture.Size = new System.Drawing.Size(37, 31);
            this.pb_latencyPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_latencyPicture.TabIndex = 27;
            this.pb_latencyPicture.TabStop = false;
            // 
            // txt_address1
            // 
            this.txt_address1.Animated = true;
            this.txt_address1.BackColor = System.Drawing.Color.Transparent;
            this.txt_address1.BorderRadius = 5;
            this.txt_address1.Cursor = System.Windows.Forms.Cursors.IBeam;
            this.txt_address1.DefaultText = "";
            this.txt_address1.DisabledState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(208)))), ((int)(((byte)(208)))), ((int)(((byte)(208)))));
            this.txt_address1.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(226)))), ((int)(((byte)(226)))), ((int)(((byte)(226)))));
            this.txt_address1.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(138)))), ((int)(((byte)(138)))), ((int)(((byte)(138)))));
            this.txt_address1.DisabledState.PlaceholderForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(138)))), ((int)(((byte)(138)))), ((int)(((byte)(138)))));
            this.txt_address1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.txt_address1.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.txt_address1.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.txt_address1.HoverState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.txt_address1.Location = new System.Drawing.Point(55, 152);
            this.txt_address1.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
            this.txt_address1.Name = "txt_address1";
            this.txt_address1.PasswordChar = '\0';
            this.txt_address1.PlaceholderForeColor = System.Drawing.Color.Gray;
            this.txt_address1.PlaceholderText = "Address 1";
            this.txt_address1.SelectedText = "";
            this.txt_address1.Size = new System.Drawing.Size(319, 38);
            this.txt_address1.TabIndex = 1;
            this.txt_address1.KeyPress += new System.Windows.Forms.KeyPressEventHandler(this.onlyAddress);
            // 
            // txt_name
            // 
            this.txt_name.Animated = true;
            this.txt_name.BackColor = System.Drawing.Color.Transparent;
            this.txt_name.BorderRadius = 5;
            this.txt_name.Cursor = System.Windows.Forms.Cursors.IBeam;
            this.txt_name.DefaultText = "";
            this.txt_name.DisabledState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(208)))), ((int)(((byte)(208)))), ((int)(((byte)(208)))));
            this.txt_name.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(226)))), ((int)(((byte)(226)))), ((int)(((byte)(226)))));
            this.txt_name.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(138)))), ((int)(((byte)(138)))), ((int)(((byte)(138)))));
            this.txt_name.DisabledState.PlaceholderForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(138)))), ((int)(((byte)(138)))), ((int)(((byte)(138)))));
            this.txt_name.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.txt_name.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.txt_name.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.txt_name.HoverState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.txt_name.Location = new System.Drawing.Point(55, 64);
            this.txt_name.Margin = new System.Windows.Forms.Padding(3, 4, 3, 4);
            this.txt_name.MaxLength = 15;
            this.txt_name.Name = "txt_name";
            this.txt_name.PasswordChar = '\0';
            this.txt_name.PlaceholderForeColor = System.Drawing.Color.Gray;
            this.txt_name.PlaceholderText = "Name";
            this.txt_name.SelectedText = "";
            this.txt_name.Size = new System.Drawing.Size(319, 38);
            this.txt_name.TabIndex = 0;
            // 
            // label3
            // 
            this.label3.BackColor = System.Drawing.Color.Transparent;
            this.label3.Font = new System.Drawing.Font("JetBrains Mono", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label3.ForeColor = System.Drawing.Color.White;
            this.label3.Location = new System.Drawing.Point(59, 117);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(151, 27);
            this.label3.TabIndex = 23;
            this.label3.Text = "Server Address";
            this.label3.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // label1
            // 
            this.label1.BackColor = System.Drawing.Color.Transparent;
            this.label1.Font = new System.Drawing.Font("JetBrains Mono", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label1.ForeColor = System.Drawing.Color.White;
            this.label1.Location = new System.Drawing.Point(59, 27);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(117, 27);
            this.label1.TabIndex = 23;
            this.label1.Text = "Server Name";
            this.label1.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_ping
            // 
            this.lbl_ping.BackColor = System.Drawing.Color.Transparent;
            this.lbl_ping.Font = new System.Drawing.Font("JetBrains Mono", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_ping.ForeColor = System.Drawing.Color.White;
            this.lbl_ping.Location = new System.Drawing.Point(54, 19);
            this.lbl_ping.Name = "lbl_ping";
            this.lbl_ping.Size = new System.Drawing.Size(69, 27);
            this.lbl_ping.TabIndex = 23;
            this.lbl_ping.Text = "??? ms";
            this.lbl_ping.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // btn_ping
            // 
            this.btn_ping.Animated = true;
            this.btn_ping.BackColor = System.Drawing.Color.Transparent;
            this.btn_ping.BorderRadius = 15;
            this.btn_ping.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_ping.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_ping.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_ping.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_ping.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_ping.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(0)))), ((int)(((byte)(121)))), ((int)(((byte)(255)))));
            this.btn_ping.Font = new System.Drawing.Font("JetBrains Mono", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_ping.ForeColor = System.Drawing.Color.White;
            this.btn_ping.Location = new System.Drawing.Point(221, 15);
            this.btn_ping.Name = "btn_ping";
            this.btn_ping.Size = new System.Drawing.Size(77, 31);
            this.btn_ping.TabIndex = 3;
            this.btn_ping.Text = "Ping";
            this.btn_ping.Click += new System.EventHandler(this.btn_ping_Click);
            // 
            // btn_cancel
            // 
            this.btn_cancel.Animated = true;
            this.btn_cancel.BackColor = System.Drawing.Color.Transparent;
            this.btn_cancel.BorderRadius = 5;
            this.btn_cancel.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_cancel.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_cancel.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_cancel.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_cancel.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_cancel.FillColor = System.Drawing.Color.WhiteSmoke;
            this.btn_cancel.Font = new System.Drawing.Font("Arial", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(178)));
            this.btn_cancel.ForeColor = System.Drawing.Color.Black;
            this.btn_cancel.Location = new System.Drawing.Point(55, 327);
            this.btn_cancel.Name = "btn_cancel";
            this.btn_cancel.Size = new System.Drawing.Size(155, 39);
            this.btn_cancel.TabIndex = 5;
            this.btn_cancel.Text = "Cancel";
            this.btn_cancel.Click += new System.EventHandler(this.btn_cancel_Click);
            // 
            // btn_addService
            // 
            this.btn_addService.Animated = true;
            this.btn_addService.BackColor = System.Drawing.Color.Transparent;
            this.btn_addService.BorderColor = System.Drawing.Color.White;
            this.btn_addService.BorderRadius = 5;
            this.btn_addService.BorderThickness = 1;
            this.btn_addService.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_addService.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_addService.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_addService.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_addService.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_addService.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.btn_addService.Font = new System.Drawing.Font("Arial", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(178)));
            this.btn_addService.ForeColor = System.Drawing.Color.White;
            this.btn_addService.Location = new System.Drawing.Point(214, 327);
            this.btn_addService.Name = "btn_addService";
            this.btn_addService.Size = new System.Drawing.Size(160, 39);
            this.btn_addService.TabIndex = 4;
            this.btn_addService.Text = "Add Server";
            this.btn_addService.Click += new System.EventHandler(this.btn_addService_Click);
            // 
            // bf_form
            // 
            this.bf_form.AnimationInterval = 250;
            this.bf_form.BorderRadius = 15;
            this.bf_form.ContainerControl = this;
            this.bf_form.DockIndicatorTransparencyValue = 0.6D;
            this.bf_form.DragForm = false;
            this.bf_form.ResizeForm = false;
            this.bf_form.ShadowColor = System.Drawing.Color.WhiteSmoke;
            this.bf_form.TransparentWhileDrag = true;
            // 
            // pnl_latencySection
            // 
            this.pnl_latencySection.BorderColor = System.Drawing.Color.White;
            this.pnl_latencySection.BorderRadius = 5;
            this.pnl_latencySection.BorderThickness = 1;
            this.pnl_latencySection.Controls.Add(this.pb_latencyPicture);
            this.pnl_latencySection.Controls.Add(this.btn_ping);
            this.pnl_latencySection.Controls.Add(this.lbl_ping);
            this.pnl_latencySection.Location = new System.Drawing.Point(55, 250);
            this.pnl_latencySection.Name = "pnl_latencySection";
            this.pnl_latencySection.Size = new System.Drawing.Size(319, 62);
            this.pnl_latencySection.TabIndex = 28;
            // 
            // frm_service
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(422, 406);
            this.Controls.Add(this.pnl_services);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Name = "frm_service";
            this.ShowIcon = false;
            this.ShowInTaskbar = false;
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Service ";
            this.pnl_services.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).EndInit();
            this.pnl_latencySection.ResumeLayout(false);
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2Panel pnl_services;
        private Guna.UI2.WinForms.Guna2TextBox txt_address2;
        private Guna.UI2.WinForms.Guna2PictureBox pb_latencyPicture;
        private Guna.UI2.WinForms.Guna2TextBox txt_address1;
        private Guna.UI2.WinForms.Guna2TextBox txt_name;
        private System.Windows.Forms.Label lbl_ping;
        private Guna.UI2.WinForms.Guna2Button btn_ping;
        private Guna.UI2.WinForms.Guna2Button btn_addService;
        private Guna.UI2.WinForms.Guna2Button btn_cancel;
        private Guna.UI2.WinForms.Guna2BorderlessForm bf_form;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label label3;
        private Guna.UI2.WinForms.Guna2Panel pnl_latencySection;
    }
}