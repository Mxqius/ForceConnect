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
            this.pnl_explore = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2CustomGradientPanel1 = new Guna.UI2.WinForms.Guna2CustomGradientPanel();
            this.lbl_name = new System.Windows.Forms.Label();
            this.lbl_previewAddress = new System.Windows.Forms.Label();
            this.label4 = new System.Windows.Forms.Label();
            this.label2 = new System.Windows.Forms.Label();
            this.btn_removeDNS = new Guna.UI2.WinForms.Guna2Button();
            this.label1 = new System.Windows.Forms.Label();
            this.lbl_latency = new System.Windows.Forms.Label();
            this.btn_addDNS = new Guna.UI2.WinForms.Guna2Button();
            this.lbl_counter = new System.Windows.Forms.Label();
            this.pb_latencyPicture = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.pb_dnsPicture = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.btn_previous = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.btn_next = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.pnl_explore.SuspendLayout();
            this.guna2CustomGradientPanel1.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_dnsPicture)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_previous)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_next)).BeginInit();
            this.SuspendLayout();
            // 
            // pnl_explore
            // 
            this.pnl_explore.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.pnl_explore.Controls.Add(this.guna2CustomGradientPanel1);
            this.pnl_explore.Controls.Add(this.btn_previous);
            this.pnl_explore.Controls.Add(this.btn_next);
            this.pnl_explore.Controls.Add(this.lbl_counter);
            this.pnl_explore.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pnl_explore.Location = new System.Drawing.Point(0, 0);
            this.pnl_explore.Name = "pnl_explore";
            this.pnl_explore.Size = new System.Drawing.Size(884, 408);
            this.pnl_explore.TabIndex = 0;
            // 
            // guna2CustomGradientPanel1
            // 
            this.guna2CustomGradientPanel1.BackColor = System.Drawing.Color.Transparent;
            this.guna2CustomGradientPanel1.BorderColor = System.Drawing.Color.White;
            this.guna2CustomGradientPanel1.BorderRadius = 20;
            this.guna2CustomGradientPanel1.BorderThickness = 2;
            this.guna2CustomGradientPanel1.Controls.Add(this.pb_latencyPicture);
            this.guna2CustomGradientPanel1.Controls.Add(this.btn_addDNS);
            this.guna2CustomGradientPanel1.Controls.Add(this.btn_removeDNS);
            this.guna2CustomGradientPanel1.Controls.Add(this.pb_dnsPicture);
            this.guna2CustomGradientPanel1.Controls.Add(this.lbl_latency);
            this.guna2CustomGradientPanel1.Controls.Add(this.lbl_name);
            this.guna2CustomGradientPanel1.Controls.Add(this.lbl_previewAddress);
            this.guna2CustomGradientPanel1.Controls.Add(this.label1);
            this.guna2CustomGradientPanel1.Controls.Add(this.label4);
            this.guna2CustomGradientPanel1.Controls.Add(this.label2);
            this.guna2CustomGradientPanel1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2CustomGradientPanel1.FillColor2 = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2CustomGradientPanel1.FillColor3 = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2CustomGradientPanel1.FillColor4 = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.guna2CustomGradientPanel1.Location = new System.Drawing.Point(167, 43);
            this.guna2CustomGradientPanel1.Name = "guna2CustomGradientPanel1";
            this.guna2CustomGradientPanel1.Size = new System.Drawing.Size(547, 306);
            this.guna2CustomGradientPanel1.TabIndex = 1;
            // 
            // lbl_name
            // 
            this.lbl_name.BackColor = System.Drawing.Color.Transparent;
            this.lbl_name.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_name.ForeColor = System.Drawing.Color.White;
            this.lbl_name.Location = new System.Drawing.Point(102, 93);
            this.lbl_name.Name = "lbl_name";
            this.lbl_name.Size = new System.Drawing.Size(111, 19);
            this.lbl_name.TabIndex = 9;
            this.lbl_name.Text = "Shecan";
            this.lbl_name.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_previewAddress
            // 
            this.lbl_previewAddress.BackColor = System.Drawing.Color.Transparent;
            this.lbl_previewAddress.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_previewAddress.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_previewAddress.ForeColor = System.Drawing.Color.White;
            this.lbl_previewAddress.Location = new System.Drawing.Point(366, 75);
            this.lbl_previewAddress.Name = "lbl_previewAddress";
            this.lbl_previewAddress.Size = new System.Drawing.Size(142, 66);
            this.lbl_previewAddress.TabIndex = 10;
            this.lbl_previewAddress.Text = "178.22.122.100 185.51.200.2";
            this.lbl_previewAddress.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            this.lbl_previewAddress.Click += new System.EventHandler(this.lbl_previewAddress_Click);
            // 
            // label4
            // 
            this.label4.BackColor = System.Drawing.Color.Transparent;
            this.label4.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label4.ForeColor = System.Drawing.Color.White;
            this.label4.Location = new System.Drawing.Point(353, 41);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(142, 34);
            this.label4.TabIndex = 11;
            this.label4.Text = "Address";
            this.label4.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // label2
            // 
            this.label2.BackColor = System.Drawing.Color.Transparent;
            this.label2.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label2.ForeColor = System.Drawing.Color.White;
            this.label2.Location = new System.Drawing.Point(59, 41);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(142, 34);
            this.label2.TabIndex = 12;
            this.label2.Text = "Name";
            this.label2.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // btn_removeDNS
            // 
            this.btn_removeDNS.Animated = true;
            this.btn_removeDNS.BackColor = System.Drawing.Color.Transparent;
            this.btn_removeDNS.BorderRadius = 5;
            this.btn_removeDNS.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_removeDNS.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_removeDNS.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_removeDNS.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_removeDNS.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_removeDNS.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(237)))), ((int)(((byte)(43)))), ((int)(((byte)(42)))));
            this.btn_removeDNS.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_removeDNS.ForeColor = System.Drawing.Color.White;
            this.btn_removeDNS.Location = new System.Drawing.Point(357, 213);
            this.btn_removeDNS.Name = "btn_removeDNS";
            this.btn_removeDNS.Size = new System.Drawing.Size(133, 35);
            this.btn_removeDNS.TabIndex = 14;
            this.btn_removeDNS.Text = "Remove";
            // 
            // label1
            // 
            this.label1.BackColor = System.Drawing.Color.Transparent;
            this.label1.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label1.ForeColor = System.Drawing.Color.White;
            this.label1.Location = new System.Drawing.Point(59, 160);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(142, 34);
            this.label1.TabIndex = 11;
            this.label1.Text = "DNS Latency";
            this.label1.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_latency
            // 
            this.lbl_latency.BackColor = System.Drawing.Color.Transparent;
            this.lbl_latency.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_latency.ForeColor = System.Drawing.Color.White;
            this.lbl_latency.Location = new System.Drawing.Point(90, 214);
            this.lbl_latency.Name = "lbl_latency";
            this.lbl_latency.Size = new System.Drawing.Size(111, 19);
            this.lbl_latency.TabIndex = 9;
            this.lbl_latency.Text = "120 ms";
            this.lbl_latency.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // btn_addDNS
            // 
            this.btn_addDNS.Animated = true;
            this.btn_addDNS.BackColor = System.Drawing.Color.Transparent;
            this.btn_addDNS.BorderRadius = 5;
            this.btn_addDNS.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_addDNS.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_addDNS.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_addDNS.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_addDNS.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_addDNS.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(0)))), ((int)(((byte)(159)))), ((int)(((byte)(189)))));
            this.btn_addDNS.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_addDNS.ForeColor = System.Drawing.Color.White;
            this.btn_addDNS.Location = new System.Drawing.Point(357, 159);
            this.btn_addDNS.Name = "btn_addDNS";
            this.btn_addDNS.Size = new System.Drawing.Size(133, 35);
            this.btn_addDNS.TabIndex = 14;
            this.btn_addDNS.Text = "Add DNS";
            // 
            // lbl_counter
            // 
            this.lbl_counter.BackColor = System.Drawing.Color.Transparent;
            this.lbl_counter.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_counter.ForeColor = System.Drawing.Color.White;
            this.lbl_counter.Location = new System.Drawing.Point(377, 370);
            this.lbl_counter.Name = "lbl_counter";
            this.lbl_counter.Size = new System.Drawing.Size(111, 19);
            this.lbl_counter.TabIndex = 9;
            this.lbl_counter.Text = "1 / 6";
            this.lbl_counter.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // pb_latencyPicture
            // 
            this.pb_latencyPicture.BackColor = System.Drawing.Color.Transparent;
            this.pb_latencyPicture.Image = global::ForceConnect.Properties.Resources.signal;
            this.pb_latencyPicture.ImageRotate = 0F;
            this.pb_latencyPicture.Location = new System.Drawing.Point(52, 210);
            this.pb_latencyPicture.Name = "pb_latencyPicture";
            this.pb_latencyPicture.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.pb_latencyPicture.Size = new System.Drawing.Size(32, 32);
            this.pb_latencyPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
            this.pb_latencyPicture.TabIndex = 15;
            this.pb_latencyPicture.TabStop = false;
            // 
            // pb_dnsPicture
            // 
            this.pb_dnsPicture.BackColor = System.Drawing.Color.Transparent;
            this.pb_dnsPicture.Image = global::ForceConnect.Properties.Resources.shecan;
            this.pb_dnsPicture.ImageRotate = 0F;
            this.pb_dnsPicture.Location = new System.Drawing.Point(54, 85);
            this.pb_dnsPicture.Name = "pb_dnsPicture";
            this.pb_dnsPicture.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.pb_dnsPicture.Size = new System.Drawing.Size(32, 32);
            this.pb_dnsPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_dnsPicture.TabIndex = 13;
            this.pb_dnsPicture.TabStop = false;
            // 
            // btn_previous
            // 
            this.btn_previous.BackColor = System.Drawing.Color.Transparent;
            this.btn_previous.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_previous.FillColor = System.Drawing.Color.Transparent;
            this.btn_previous.Image = global::ForceConnect.Properties.Resources.previous;
            this.btn_previous.ImageRotate = 0F;
            this.btn_previous.Location = new System.Drawing.Point(37, 160);
            this.btn_previous.Name = "btn_previous";
            this.btn_previous.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_previous.Size = new System.Drawing.Size(50, 47);
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
            this.btn_next.Location = new System.Drawing.Point(799, 160);
            this.btn_next.Name = "btn_next";
            this.btn_next.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_next.Size = new System.Drawing.Size(50, 47);
            this.btn_next.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.btn_next.TabIndex = 0;
            this.btn_next.TabStop = false;
            this.btn_next.Click += new System.EventHandler(this.btn_next_Click);
            // 
            // frm_explore
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(884, 408);
            this.Controls.Add(this.pnl_explore);
            this.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.MinimizeBox = false;
            this.Name = "frm_explore";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Explore";
            this.Load += new System.EventHandler(this.frm_explore_Load);
            this.pnl_explore.ResumeLayout(false);
            this.guna2CustomGradientPanel1.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_dnsPicture)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_previous)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.btn_next)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2Panel pnl_explore;
        private Guna.UI2.WinForms.Guna2CirclePictureBox btn_next;
        private Guna.UI2.WinForms.Guna2CirclePictureBox btn_previous;
        private Guna.UI2.WinForms.Guna2CustomGradientPanel guna2CustomGradientPanel1;
        private Guna.UI2.WinForms.Guna2CirclePictureBox pb_dnsPicture;
        private System.Windows.Forms.Label lbl_name;
        private System.Windows.Forms.Label lbl_previewAddress;
        private System.Windows.Forms.Label label4;
        private System.Windows.Forms.Label label2;
        private Guna.UI2.WinForms.Guna2Button btn_removeDNS;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label lbl_latency;
        private Guna.UI2.WinForms.Guna2CirclePictureBox pb_latencyPicture;
        private Guna.UI2.WinForms.Guna2Button btn_addDNS;
        private System.Windows.Forms.Label lbl_counter;
    }
}