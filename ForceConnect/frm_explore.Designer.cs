namespace ForceConnect
{
    partial class frm_explore
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
            this.pnl_cardDns = new Guna.UI2.WinForms.Guna2CustomGradientPanel();
            this.btn_removeDNS = new Guna.UI2.WinForms.Guna2CircleButton();
            this.pb_latencyPicture = new Guna.UI2.WinForms.Guna2PictureBox();
            this.pb_dnsPicture = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.lbl_name = new System.Windows.Forms.Label();
            this.lbl_latency = new System.Windows.Forms.Label();
            this.lbl_previewAddress = new System.Windows.Forms.Label();
            this.label1 = new System.Windows.Forms.Label();
            this.label4 = new System.Windows.Forms.Label();
            this.guna2Shapes1 = new Guna.UI2.WinForms.Guna2Shapes();
            this.lbl_counter = new System.Windows.Forms.Label();
            this.btn_openServices = new Guna.UI2.WinForms.Guna2Button();
            this.btn_previous = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.btn_next = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.btn_refresh = new Guna.UI2.WinForms.Guna2Button();
            this.pnl_cardDns.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_dnsPicture)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_previous)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_next)).BeginInit();
            this.SuspendLayout();
            // 
            // pnl_cardDns
            // 
            this.pnl_cardDns.BackColor = System.Drawing.Color.Transparent;
            this.pnl_cardDns.BorderColor = System.Drawing.Color.WhiteSmoke;
            this.pnl_cardDns.BorderRadius = 20;
            this.pnl_cardDns.BorderThickness = 2;
            this.pnl_cardDns.Controls.Add(this.btn_removeDNS);
            this.pnl_cardDns.Controls.Add(this.pb_latencyPicture);
            this.pnl_cardDns.Controls.Add(this.pb_dnsPicture);
            this.pnl_cardDns.Controls.Add(this.lbl_name);
            this.pnl_cardDns.Controls.Add(this.lbl_latency);
            this.pnl_cardDns.Controls.Add(this.lbl_previewAddress);
            this.pnl_cardDns.Controls.Add(this.label1);
            this.pnl_cardDns.Controls.Add(this.label4);
            this.pnl_cardDns.Controls.Add(this.guna2Shapes1);
            this.pnl_cardDns.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_cardDns.FillColor2 = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_cardDns.FillColor3 = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_cardDns.FillColor4 = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_cardDns.Location = new System.Drawing.Point(133, 80);
            this.pnl_cardDns.Name = "pnl_cardDns";
            this.pnl_cardDns.Size = new System.Drawing.Size(611, 231);
            this.pnl_cardDns.TabIndex = 1;
            // 
            // btn_removeDNS
            // 
            this.btn_removeDNS.Animated = true;
            this.btn_removeDNS.BackColor = System.Drawing.Color.WhiteSmoke;
            this.btn_removeDNS.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_removeDNS.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_removeDNS.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_removeDNS.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_removeDNS.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_removeDNS.FillColor = System.Drawing.Color.Transparent;
            this.btn_removeDNS.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_removeDNS.ForeColor = System.Drawing.Color.White;
            this.btn_removeDNS.Image = global::ForceConnect.Properties.Resources.delete;
            this.btn_removeDNS.ImageSize = new System.Drawing.Size(30, 30);
            this.btn_removeDNS.Location = new System.Drawing.Point(562, 7);
            this.btn_removeDNS.Name = "btn_removeDNS";
            this.btn_removeDNS.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_removeDNS.Size = new System.Drawing.Size(44, 40);
            this.btn_removeDNS.TabIndex = 17;
            // 
            // pb_latencyPicture
            // 
            this.pb_latencyPicture.FillColor = System.Drawing.Color.Transparent;
            this.pb_latencyPicture.Image = global::ForceConnect.Properties.Resources.signalGreen;
            this.pb_latencyPicture.ImageRotate = 0F;
            this.pb_latencyPicture.Location = new System.Drawing.Point(303, 151);
            this.pb_latencyPicture.Name = "pb_latencyPicture";
            this.pb_latencyPicture.Size = new System.Drawing.Size(28, 26);
            this.pb_latencyPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_latencyPicture.TabIndex = 15;
            this.pb_latencyPicture.TabStop = false;
            // 
            // pb_dnsPicture
            // 
            this.pb_dnsPicture.BackColor = System.Drawing.Color.Transparent;
            this.pb_dnsPicture.Image = global::ForceConnect.Properties.Resources.shecan;
            this.pb_dnsPicture.ImageRotate = 0F;
            this.pb_dnsPicture.Location = new System.Drawing.Point(26, 30);
            this.pb_dnsPicture.Name = "pb_dnsPicture";
            this.pb_dnsPicture.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.pb_dnsPicture.Size = new System.Drawing.Size(48, 48);
            this.pb_dnsPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_dnsPicture.TabIndex = 13;
            this.pb_dnsPicture.TabStop = false;
            // 
            // lbl_name
            // 
            this.lbl_name.BackColor = System.Drawing.Color.Transparent;
            this.lbl_name.Font = new System.Drawing.Font("JetBrains Mono", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_name.ForeColor = System.Drawing.Color.White;
            this.lbl_name.Location = new System.Drawing.Point(89, 36);
            this.lbl_name.Name = "lbl_name";
            this.lbl_name.Size = new System.Drawing.Size(175, 38);
            this.lbl_name.TabIndex = 9;
            this.lbl_name.Text = "Shecan";
            this.lbl_name.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_latency
            // 
            this.lbl_latency.BackColor = System.Drawing.Color.Transparent;
            this.lbl_latency.Font = new System.Drawing.Font("JetBrains Mono", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_latency.ForeColor = System.Drawing.Color.White;
            this.lbl_latency.Location = new System.Drawing.Point(337, 156);
            this.lbl_latency.Name = "lbl_latency";
            this.lbl_latency.Size = new System.Drawing.Size(71, 22);
            this.lbl_latency.TabIndex = 9;
            this.lbl_latency.Text = "??? ms";
            this.lbl_latency.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_previewAddress
            // 
            this.lbl_previewAddress.BackColor = System.Drawing.Color.Transparent;
            this.lbl_previewAddress.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_previewAddress.Font = new System.Drawing.Font("JetBrains Mono", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_previewAddress.ForeColor = System.Drawing.Color.White;
            this.lbl_previewAddress.Location = new System.Drawing.Point(51, 141);
            this.lbl_previewAddress.Name = "lbl_previewAddress";
            this.lbl_previewAddress.Size = new System.Drawing.Size(197, 46);
            this.lbl_previewAddress.TabIndex = 10;
            this.lbl_previewAddress.Text = "178.22.122.100 185.51.200.2";
            this.lbl_previewAddress.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.lbl_previewAddress.Click += new System.EventHandler(this.lbl_previewAddress_Click);
            // 
            // label1
            // 
            this.label1.BackColor = System.Drawing.Color.Transparent;
            this.label1.Font = new System.Drawing.Font("JetBrains Mono", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label1.ForeColor = System.Drawing.Color.White;
            this.label1.Location = new System.Drawing.Point(263, 106);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(175, 28);
            this.label1.TabIndex = 11;
            this.label1.Text = " Latency";
            this.label1.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // label4
            // 
            this.label4.BackColor = System.Drawing.Color.Transparent;
            this.label4.Font = new System.Drawing.Font("JetBrains Mono", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label4.ForeColor = System.Drawing.Color.White;
            this.label4.Location = new System.Drawing.Point(50, 106);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(179, 26);
            this.label4.TabIndex = 11;
            this.label4.Text = "Address";
            this.label4.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // guna2Shapes1
            // 
            this.guna2Shapes1.BorderThickness = 0;
            this.guna2Shapes1.FillColor = System.Drawing.Color.WhiteSmoke;
            this.guna2Shapes1.Location = new System.Drawing.Point(414, -111);
            this.guna2Shapes1.Name = "guna2Shapes1";
            this.guna2Shapes1.PolygonSkip = 1;
            this.guna2Shapes1.Rotate = 0F;
            this.guna2Shapes1.Shape = Guna.UI2.WinForms.Enums.ShapeType.Ellipse;
            this.guna2Shapes1.Size = new System.Drawing.Size(320, 388);
            this.guna2Shapes1.TabIndex = 18;
            this.guna2Shapes1.Zoom = 80;
            // 
            // lbl_counter
            // 
            this.lbl_counter.BackColor = System.Drawing.Color.Transparent;
            this.lbl_counter.Font = new System.Drawing.Font("JetBrains Mono ExtraBold", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_counter.ForeColor = System.Drawing.Color.White;
            this.lbl_counter.Location = new System.Drawing.Point(374, 361);
            this.lbl_counter.Name = "lbl_counter";
            this.lbl_counter.Size = new System.Drawing.Size(111, 19);
            this.lbl_counter.TabIndex = 9;
            this.lbl_counter.Text = "? / ?";
            this.lbl_counter.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // btn_openServices
            // 
            this.btn_openServices.Animated = true;
            this.btn_openServices.BackColor = System.Drawing.Color.Transparent;
            this.btn_openServices.BorderRadius = 15;
            this.btn_openServices.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_openServices.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_openServices.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_openServices.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_openServices.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_openServices.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(28)))), ((int)(((byte)(28)))), ((int)(((byte)(28)))));
            this.btn_openServices.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_openServices.ForeColor = System.Drawing.Color.White;
            this.btn_openServices.Image = global::ForceConnect.Properties.Resources.services;
            this.btn_openServices.ImageOffset = new System.Drawing.Point(-10, 0);
            this.btn_openServices.Location = new System.Drawing.Point(728, 12);
            this.btn_openServices.Name = "btn_openServices";
            this.btn_openServices.Size = new System.Drawing.Size(144, 43);
            this.btn_openServices.TabIndex = 0;
            this.btn_openServices.Text = "Services";
            this.btn_openServices.Click += new System.EventHandler(this.btn_openServices_Click);
            // 
            // btn_previous
            // 
            this.btn_previous.BackColor = System.Drawing.Color.Transparent;
            this.btn_previous.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_previous.FillColor = System.Drawing.Color.Transparent;
            this.btn_previous.Image = global::ForceConnect.Properties.Resources.previous;
            this.btn_previous.ImageRotate = 0F;
            this.btn_previous.Location = new System.Drawing.Point(32, 163);
            this.btn_previous.Name = "btn_previous";
            this.btn_previous.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_previous.Size = new System.Drawing.Size(53, 51);
            this.btn_previous.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.btn_previous.TabIndex = 0;
            this.btn_previous.TabStop = false;
            this.btn_previous.Click += new System.EventHandler(this.btn_previous_Click);
            // 
            // btn_next
            // 
            this.btn_next.BackColor = System.Drawing.Color.Transparent;
            this.btn_next.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_next.FillColor = System.Drawing.Color.Transparent;
            this.btn_next.Image = global::ForceConnect.Properties.Resources.next;
            this.btn_next.ImageRotate = 0F;
            this.btn_next.Location = new System.Drawing.Point(796, 163);
            this.btn_next.Name = "btn_next";
            this.btn_next.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_next.Size = new System.Drawing.Size(53, 51);
            this.btn_next.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.btn_next.TabIndex = 0;
            this.btn_next.TabStop = false;
            this.btn_next.Click += new System.EventHandler(this.btn_next_Click);
            // 
            // btn_refresh
            // 
            this.btn_refresh.Animated = true;
            this.btn_refresh.BackColor = System.Drawing.Color.Transparent;
            this.btn_refresh.BorderRadius = 15;
            this.btn_refresh.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_refresh.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_refresh.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_refresh.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_refresh.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_refresh.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(28)))), ((int)(((byte)(28)))), ((int)(((byte)(28)))));
            this.btn_refresh.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_refresh.ForeColor = System.Drawing.Color.White;
            this.btn_refresh.ImageOffset = new System.Drawing.Point(-10, 0);
            this.btn_refresh.Location = new System.Drawing.Point(578, 12);
            this.btn_refresh.Name = "btn_refresh";
            this.btn_refresh.Size = new System.Drawing.Size(144, 43);
            this.btn_refresh.TabIndex = 0;
            this.btn_refresh.Text = "Refresh";
            this.btn_refresh.Click += new System.EventHandler(this.btn_refresh_Click);
            // 
            // frm_explore
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.ClientSize = new System.Drawing.Size(884, 408);
            this.Controls.Add(this.btn_refresh);
            this.Controls.Add(this.btn_openServices);
            this.Controls.Add(this.btn_previous);
            this.Controls.Add(this.btn_next);
            this.Controls.Add(this.lbl_counter);
            this.Controls.Add(this.pnl_cardDns);
            this.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.KeyPreview = true;
            this.MinimizeBox = false;
            this.Name = "frm_explore";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Explore";
            this.Load += new System.EventHandler(this.frm_explore_Load);
            this.pnl_cardDns.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_dnsPicture)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_previous)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_next)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion
        private Guna.UI2.WinForms.Guna2CustomGradientPanel pnl_cardDns;
        private Guna.UI2.WinForms.Guna2CirclePictureBox pb_dnsPicture;
        private System.Windows.Forms.Label lbl_name;
        private System.Windows.Forms.Label lbl_previewAddress;
        private System.Windows.Forms.Label label4;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label lbl_latency;
        private System.Windows.Forms.Label lbl_counter;
        private Guna.UI2.WinForms.Guna2CirclePictureBox btn_next;
        private Guna.UI2.WinForms.Guna2CirclePictureBox btn_previous;
        private Guna.UI2.WinForms.Guna2PictureBox pb_latencyPicture;
        private Guna.UI2.WinForms.Guna2CircleButton btn_removeDNS;
        private Guna.UI2.WinForms.Guna2Button btn_openServices;
        private Guna.UI2.WinForms.Guna2Shapes guna2Shapes1;
        private Guna.UI2.WinForms.Guna2Button btn_refresh;
    }
}