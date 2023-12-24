namespace ForceConnect
{
    partial class frm_network
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
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(frm_network));
            this.bf_form = new Guna.UI2.WinForms.Guna2BorderlessForm(this.components);
            this.label1 = new System.Windows.Forms.Label();
            this.label2 = new System.Windows.Forms.Label();
            this.lbl_interface = new System.Windows.Forms.Label();
            this.label4 = new System.Windows.Forms.Label();
            this.label5 = new System.Windows.Forms.Label();
            this.label6 = new System.Windows.Forms.Label();
            this.label7 = new System.Windows.Forms.Label();
            this.label8 = new System.Windows.Forms.Label();
            this.lbl_ipAddress = new System.Windows.Forms.Label();
            this.lbl_hostName = new System.Windows.Forms.Label();
            this.lbl_speed = new System.Windows.Forms.Label();
            this.lbl_activeServices = new System.Windows.Forms.Label();
            this.lbl_intrfaceDesc = new System.Windows.Forms.Label();
            this.lbl_intrfaceStatus = new System.Windows.Forms.Label();
            this.lbl_intrfaceName = new System.Windows.Forms.Label();
            this.btn_close = new Guna.UI2.WinForms.Guna2Button();
            this.btn_refresh = new Guna.UI2.WinForms.Guna2Button();
            this.label3 = new System.Windows.Forms.Label();
            this.lbl_macAddress = new System.Windows.Forms.Label();
            this.lbl_downloadSpeed = new System.Windows.Forms.Label();
            this.guna2Shapes4 = new Guna.UI2.WinForms.Guna2Shapes();
            this.guna2Panel1 = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Shapes2 = new Guna.UI2.WinForms.Guna2Shapes();
            this.lbl_uploadSpeed = new System.Windows.Forms.Label();
            this.loadingProgressSpeed = new Guna.UI2.WinForms.Guna2WinProgressIndicator();
            this.lbl_hintNetworkAdapter = new System.Windows.Forms.Label();
            this.cb_selectNetworkAdapter = new Guna.UI2.WinForms.Guna2ComboBox();
            this.pnl_networkInformation = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Panel1.SuspendLayout();
            this.pnl_networkInformation.SuspendLayout();
            this.SuspendLayout();
            // 
            // bf_form
            // 
            this.bf_form.BorderRadius = 15;
            this.bf_form.ContainerControl = this;
            this.bf_form.DockForm = false;
            this.bf_form.DockIndicatorTransparencyValue = 0.6D;
            this.bf_form.HasFormShadow = false;
            this.bf_form.ResizeForm = false;
            this.bf_form.TransparentWhileDrag = true;
            // 
            // label1
            // 
            this.label1.AutoSize = true;
            this.label1.BackColor = System.Drawing.Color.Transparent;
            this.label1.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label1.ForeColor = System.Drawing.Color.White;
            this.label1.Location = new System.Drawing.Point(49, 61);
            this.label1.Name = "label1";
            this.label1.Size = new System.Drawing.Size(122, 21);
            this.label1.TabIndex = 1;
            this.label1.Text = "IP Addresses: ";
            // 
            // label2
            // 
            this.label2.AutoSize = true;
            this.label2.BackColor = System.Drawing.Color.Transparent;
            this.label2.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label2.ForeColor = System.Drawing.Color.White;
            this.label2.Location = new System.Drawing.Point(49, 150);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(106, 21);
            this.label2.TabIndex = 1;
            this.label2.Text = "Host Name: ";
            // 
            // lbl_interface
            // 
            this.lbl_interface.AutoSize = true;
            this.lbl_interface.BackColor = System.Drawing.Color.Transparent;
            this.lbl_interface.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_interface.ForeColor = System.Drawing.Color.White;
            this.lbl_interface.Location = new System.Drawing.Point(25, 23);
            this.lbl_interface.Name = "lbl_interface";
            this.lbl_interface.Size = new System.Drawing.Size(159, 21);
            this.lbl_interface.TabIndex = 1;
            this.lbl_interface.Text = "Network Interface:";
            // 
            // label4
            // 
            this.label4.AutoSize = true;
            this.label4.BackColor = System.Drawing.Color.Transparent;
            this.label4.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label4.ForeColor = System.Drawing.Color.White;
            this.label4.Location = new System.Drawing.Point(49, 242);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(138, 21);
            this.label4.TabIndex = 1;
            this.label4.Text = "DNS IP Address:";
            // 
            // label5
            // 
            this.label5.AutoSize = true;
            this.label5.BackColor = System.Drawing.Color.Transparent;
            this.label5.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label5.ForeColor = System.Drawing.Color.White;
            this.label5.Location = new System.Drawing.Point(25, 67);
            this.label5.Name = "label5";
            this.label5.Size = new System.Drawing.Size(64, 21);
            this.label5.TabIndex = 1;
            this.label5.Text = "Name: ";
            // 
            // label6
            // 
            this.label6.AutoSize = true;
            this.label6.BackColor = System.Drawing.Color.Transparent;
            this.label6.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label6.ForeColor = System.Drawing.Color.White;
            this.label6.Location = new System.Drawing.Point(25, 111);
            this.label6.Name = "label6";
            this.label6.Size = new System.Drawing.Size(109, 21);
            this.label6.TabIndex = 1;
            this.label6.Text = "Description: ";
            // 
            // label7
            // 
            this.label7.AutoSize = true;
            this.label7.BackColor = System.Drawing.Color.Transparent;
            this.label7.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label7.ForeColor = System.Drawing.Color.White;
            this.label7.Location = new System.Drawing.Point(25, 155);
            this.label7.Name = "label7";
            this.label7.Size = new System.Drawing.Size(71, 21);
            this.label7.TabIndex = 1;
            this.label7.Text = "Status: ";
            // 
            // label8
            // 
            this.label8.AutoSize = true;
            this.label8.BackColor = System.Drawing.Color.Transparent;
            this.label8.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label8.ForeColor = System.Drawing.Color.White;
            this.label8.Location = new System.Drawing.Point(49, 196);
            this.label8.Name = "label8";
            this.label8.Size = new System.Drawing.Size(69, 21);
            this.label8.TabIndex = 1;
            this.label8.Text = "Speed: ";
            // 
            // lbl_ipAddress
            // 
            this.lbl_ipAddress.BackColor = System.Drawing.Color.Transparent;
            this.lbl_ipAddress.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_ipAddress.ForeColor = System.Drawing.Color.White;
            this.lbl_ipAddress.Location = new System.Drawing.Point(215, 61);
            this.lbl_ipAddress.Name = "lbl_ipAddress";
            this.lbl_ipAddress.Size = new System.Drawing.Size(199, 22);
            this.lbl_ipAddress.TabIndex = 1;
            this.lbl_ipAddress.Text = "???";
            // 
            // lbl_hostName
            // 
            this.lbl_hostName.BackColor = System.Drawing.Color.Transparent;
            this.lbl_hostName.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_hostName.ForeColor = System.Drawing.Color.White;
            this.lbl_hostName.Location = new System.Drawing.Point(215, 151);
            this.lbl_hostName.Name = "lbl_hostName";
            this.lbl_hostName.Size = new System.Drawing.Size(199, 22);
            this.lbl_hostName.TabIndex = 1;
            this.lbl_hostName.Text = "???";
            // 
            // lbl_speed
            // 
            this.lbl_speed.BackColor = System.Drawing.Color.Transparent;
            this.lbl_speed.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_speed.ForeColor = System.Drawing.Color.White;
            this.lbl_speed.Location = new System.Drawing.Point(215, 197);
            this.lbl_speed.Name = "lbl_speed";
            this.lbl_speed.Size = new System.Drawing.Size(198, 22);
            this.lbl_speed.TabIndex = 1;
            this.lbl_speed.Text = "???";
            // 
            // lbl_activeServices
            // 
            this.lbl_activeServices.BackColor = System.Drawing.Color.Transparent;
            this.lbl_activeServices.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_activeServices.ForeColor = System.Drawing.Color.White;
            this.lbl_activeServices.Location = new System.Drawing.Point(215, 243);
            this.lbl_activeServices.Name = "lbl_activeServices";
            this.lbl_activeServices.Size = new System.Drawing.Size(199, 89);
            this.lbl_activeServices.TabIndex = 1;
            this.lbl_activeServices.Text = "???";
            // 
            // lbl_intrfaceDesc
            // 
            this.lbl_intrfaceDesc.BackColor = System.Drawing.Color.Transparent;
            this.lbl_intrfaceDesc.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_intrfaceDesc.ForeColor = System.Drawing.Color.White;
            this.lbl_intrfaceDesc.Location = new System.Drawing.Point(160, 113);
            this.lbl_intrfaceDesc.Name = "lbl_intrfaceDesc";
            this.lbl_intrfaceDesc.Size = new System.Drawing.Size(137, 22);
            this.lbl_intrfaceDesc.TabIndex = 1;
            this.lbl_intrfaceDesc.Text = "???";
            // 
            // lbl_intrfaceStatus
            // 
            this.lbl_intrfaceStatus.BackColor = System.Drawing.Color.Transparent;
            this.lbl_intrfaceStatus.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_intrfaceStatus.ForeColor = System.Drawing.Color.White;
            this.lbl_intrfaceStatus.Location = new System.Drawing.Point(160, 155);
            this.lbl_intrfaceStatus.Name = "lbl_intrfaceStatus";
            this.lbl_intrfaceStatus.Size = new System.Drawing.Size(137, 22);
            this.lbl_intrfaceStatus.TabIndex = 1;
            this.lbl_intrfaceStatus.Text = "???";
            // 
            // lbl_intrfaceName
            // 
            this.lbl_intrfaceName.BackColor = System.Drawing.Color.Transparent;
            this.lbl_intrfaceName.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_intrfaceName.ForeColor = System.Drawing.Color.White;
            this.lbl_intrfaceName.Location = new System.Drawing.Point(160, 67);
            this.lbl_intrfaceName.Name = "lbl_intrfaceName";
            this.lbl_intrfaceName.Size = new System.Drawing.Size(137, 22);
            this.lbl_intrfaceName.TabIndex = 1;
            this.lbl_intrfaceName.Text = "???";
            // 
            // btn_close
            // 
            this.btn_close.Animated = true;
            this.btn_close.BackColor = System.Drawing.Color.Transparent;
            this.btn_close.BorderColor = System.Drawing.Color.White;
            this.btn_close.BorderRadius = 5;
            this.btn_close.BorderThickness = 1;
            this.btn_close.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_close.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_close.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_close.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_close.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_close.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.btn_close.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_close.ForeColor = System.Drawing.Color.White;
            this.btn_close.ImageOffset = new System.Drawing.Point(-10, 0);
            this.btn_close.Location = new System.Drawing.Point(599, 361);
            this.btn_close.Name = "btn_close";
            this.btn_close.Size = new System.Drawing.Size(154, 38);
            this.btn_close.TabIndex = 3;
            this.btn_close.Text = "Close";
            this.btn_close.Click += new System.EventHandler(this.btn_close_Click);
            // 
            // btn_refresh
            // 
            this.btn_refresh.Animated = true;
            this.btn_refresh.BackColor = System.Drawing.Color.Transparent;
            this.btn_refresh.BorderColor = System.Drawing.Color.White;
            this.btn_refresh.BorderRadius = 5;
            this.btn_refresh.BorderThickness = 1;
            this.btn_refresh.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_refresh.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_refresh.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_refresh.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_refresh.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_refresh.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.btn_refresh.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_refresh.ForeColor = System.Drawing.Color.White;
            this.btn_refresh.ImageOffset = new System.Drawing.Point(-10, 0);
            this.btn_refresh.Location = new System.Drawing.Point(435, 361);
            this.btn_refresh.Name = "btn_refresh";
            this.btn_refresh.Size = new System.Drawing.Size(154, 38);
            this.btn_refresh.TabIndex = 3;
            this.btn_refresh.Text = "Refresh";
            this.btn_refresh.Click += new System.EventHandler(this.btn_refresh_Click);
            // 
            // label3
            // 
            this.label3.AutoSize = true;
            this.label3.BackColor = System.Drawing.Color.Transparent;
            this.label3.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label3.ForeColor = System.Drawing.Color.White;
            this.label3.Location = new System.Drawing.Point(49, 104);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(122, 21);
            this.label3.TabIndex = 1;
            this.label3.Text = "MAC Address: ";
            // 
            // lbl_macAddress
            // 
            this.lbl_macAddress.BackColor = System.Drawing.Color.Transparent;
            this.lbl_macAddress.Font = new System.Drawing.Font("Rubik", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_macAddress.ForeColor = System.Drawing.Color.White;
            this.lbl_macAddress.Location = new System.Drawing.Point(215, 107);
            this.lbl_macAddress.Name = "lbl_macAddress";
            this.lbl_macAddress.Size = new System.Drawing.Size(199, 22);
            this.lbl_macAddress.TabIndex = 1;
            this.lbl_macAddress.Text = "???";
            // 
            // lbl_downloadSpeed
            // 
            this.lbl_downloadSpeed.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.lbl_downloadSpeed.AutoSize = true;
            this.lbl_downloadSpeed.BackColor = System.Drawing.Color.Transparent;
            this.lbl_downloadSpeed.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_downloadSpeed.ForeColor = System.Drawing.Color.White;
            this.lbl_downloadSpeed.Location = new System.Drawing.Point(142, 6);
            this.lbl_downloadSpeed.Name = "lbl_downloadSpeed";
            this.lbl_downloadSpeed.Size = new System.Drawing.Size(68, 19);
            this.lbl_downloadSpeed.TabIndex = 9;
            this.lbl_downloadSpeed.Text = "?? MBps";
            this.lbl_downloadSpeed.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // guna2Shapes4
            // 
            this.guna2Shapes4.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.guna2Shapes4.BackColor = System.Drawing.Color.Transparent;
            this.guna2Shapes4.BorderColor = System.Drawing.Color.Transparent;
            this.guna2Shapes4.FillColor = System.Drawing.Color.White;
            this.guna2Shapes4.Location = new System.Drawing.Point(117, 6);
            this.guna2Shapes4.Name = "guna2Shapes4";
            this.guna2Shapes4.PolygonSides = 3;
            this.guna2Shapes4.PolygonSkip = 1;
            this.guna2Shapes4.Rotate = 180F;
            this.guna2Shapes4.Size = new System.Drawing.Size(19, 19);
            this.guna2Shapes4.TabIndex = 10;
            this.guna2Shapes4.Zoom = 80;
            // 
            // guna2Panel1
            // 
            this.guna2Panel1.BorderRadius = 5;
            this.guna2Panel1.Controls.Add(this.guna2Shapes2);
            this.guna2Panel1.Controls.Add(this.lbl_uploadSpeed);
            this.guna2Panel1.Controls.Add(this.guna2Shapes4);
            this.guna2Panel1.Controls.Add(this.lbl_downloadSpeed);
            this.guna2Panel1.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.guna2Panel1.Location = new System.Drawing.Point(56, 363);
            this.guna2Panel1.Name = "guna2Panel1";
            this.guna2Panel1.Size = new System.Drawing.Size(235, 31);
            this.guna2Panel1.TabIndex = 12;
            // 
            // guna2Shapes2
            // 
            this.guna2Shapes2.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.guna2Shapes2.BackColor = System.Drawing.Color.Transparent;
            this.guna2Shapes2.BorderColor = System.Drawing.Color.Transparent;
            this.guna2Shapes2.FillColor = System.Drawing.Color.White;
            this.guna2Shapes2.Location = new System.Drawing.Point(22, 6);
            this.guna2Shapes2.Name = "guna2Shapes2";
            this.guna2Shapes2.PolygonSides = 3;
            this.guna2Shapes2.PolygonSkip = 1;
            this.guna2Shapes2.Rotate = 0F;
            this.guna2Shapes2.Size = new System.Drawing.Size(19, 19);
            this.guna2Shapes2.TabIndex = 11;
            this.guna2Shapes2.Zoom = 80;
            // 
            // lbl_uploadSpeed
            // 
            this.lbl_uploadSpeed.Anchor = ((System.Windows.Forms.AnchorStyles)((System.Windows.Forms.AnchorStyles.Top | System.Windows.Forms.AnchorStyles.Right)));
            this.lbl_uploadSpeed.AutoSize = true;
            this.lbl_uploadSpeed.BackColor = System.Drawing.Color.Transparent;
            this.lbl_uploadSpeed.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_uploadSpeed.ForeColor = System.Drawing.Color.White;
            this.lbl_uploadSpeed.Location = new System.Drawing.Point(47, 6);
            this.lbl_uploadSpeed.Name = "lbl_uploadSpeed";
            this.lbl_uploadSpeed.Size = new System.Drawing.Size(68, 19);
            this.lbl_uploadSpeed.TabIndex = 8;
            this.lbl_uploadSpeed.Text = "?? MBps";
            this.lbl_uploadSpeed.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // loadingProgressSpeed
            // 
            this.loadingProgressSpeed.AnimationSpeed = 40;
            this.loadingProgressSpeed.CircleSize = 1F;
            this.loadingProgressSpeed.Location = new System.Drawing.Point(307, 361);
            this.loadingProgressSpeed.Name = "loadingProgressSpeed";
            this.loadingProgressSpeed.NumberOfCircles = 3;
            this.loadingProgressSpeed.ProgressColor = System.Drawing.Color.White;
            this.loadingProgressSpeed.Size = new System.Drawing.Size(29, 33);
            this.loadingProgressSpeed.TabIndex = 14;
            // 
            // lbl_hintNetworkAdapter
            // 
            this.lbl_hintNetworkAdapter.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.lbl_hintNetworkAdapter.Font = new System.Drawing.Font("Rubik", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_hintNetworkAdapter.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(255)))), ((int)(((byte)(244)))), ((int)(((byte)(244)))));
            this.lbl_hintNetworkAdapter.Location = new System.Drawing.Point(445, 63);
            this.lbl_hintNetworkAdapter.Name = "lbl_hintNetworkAdapter";
            this.lbl_hintNetworkAdapter.Size = new System.Drawing.Size(209, 19);
            this.lbl_hintNetworkAdapter.TabIndex = 36;
            this.lbl_hintNetworkAdapter.Text = "Select a Network Adapter";
            this.lbl_hintNetworkAdapter.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.lbl_hintNetworkAdapter.UseCompatibleTextRendering = true;
            // 
            // cb_selectNetworkAdapter
            // 
            this.cb_selectNetworkAdapter.BackColor = System.Drawing.Color.Transparent;
            this.cb_selectNetworkAdapter.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(255)))), ((int)(((byte)(244)))), ((int)(((byte)(244)))));
            this.cb_selectNetworkAdapter.BorderRadius = 5;
            this.cb_selectNetworkAdapter.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_selectNetworkAdapter.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.cb_selectNetworkAdapter.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawFixed;
            this.cb_selectNetworkAdapter.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.cb_selectNetworkAdapter.Enabled = false;
            this.cb_selectNetworkAdapter.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(60)))), ((int)(((byte)(60)))), ((int)(((byte)(67)))));
            this.cb_selectNetworkAdapter.FocusedColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectNetworkAdapter.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectNetworkAdapter.Font = new System.Drawing.Font("Segoe UI", 10F);
            this.cb_selectNetworkAdapter.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(224)))), ((int)(((byte)(224)))), ((int)(((byte)(224)))));
            this.cb_selectNetworkAdapter.ItemHeight = 40;
            this.cb_selectNetworkAdapter.Location = new System.Drawing.Point(435, 48);
            this.cb_selectNetworkAdapter.Name = "cb_selectNetworkAdapter";
            this.cb_selectNetworkAdapter.Size = new System.Drawing.Size(319, 46);
            this.cb_selectNetworkAdapter.TabIndex = 37;
            this.cb_selectNetworkAdapter.TextOffset = new System.Drawing.Point(20, 0);
            this.cb_selectNetworkAdapter.SelectedIndexChanged += new System.EventHandler(this.cb_selectNetworkAdapter_SelectedIndexChanged);
            // 
            // pnl_networkInformation
            // 
            this.pnl_networkInformation.BorderColor = System.Drawing.Color.White;
            this.pnl_networkInformation.BorderRadius = 5;
            this.pnl_networkInformation.BorderThickness = 1;
            this.pnl_networkInformation.Controls.Add(this.lbl_interface);
            this.pnl_networkInformation.Controls.Add(this.lbl_intrfaceDesc);
            this.pnl_networkInformation.Controls.Add(this.lbl_intrfaceStatus);
            this.pnl_networkInformation.Controls.Add(this.lbl_intrfaceName);
            this.pnl_networkInformation.Controls.Add(this.label6);
            this.pnl_networkInformation.Controls.Add(this.label7);
            this.pnl_networkInformation.Controls.Add(this.label5);
            this.pnl_networkInformation.Location = new System.Drawing.Point(435, 114);
            this.pnl_networkInformation.Name = "pnl_networkInformation";
            this.pnl_networkInformation.Size = new System.Drawing.Size(319, 218);
            this.pnl_networkInformation.TabIndex = 38;
            // 
            // frm_network
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(44)))), ((int)(((byte)(45)))), ((int)(((byte)(52)))));
            this.ClientSize = new System.Drawing.Size(810, 450);
            this.ControlBox = false;
            this.Controls.Add(this.pnl_networkInformation);
            this.Controls.Add(this.lbl_hintNetworkAdapter);
            this.Controls.Add(this.cb_selectNetworkAdapter);
            this.Controls.Add(this.loadingProgressSpeed);
            this.Controls.Add(this.guna2Panel1);
            this.Controls.Add(this.btn_refresh);
            this.Controls.Add(this.btn_close);
            this.Controls.Add(this.lbl_activeServices);
            this.Controls.Add(this.label4);
            this.Controls.Add(this.lbl_speed);
            this.Controls.Add(this.label8);
            this.Controls.Add(this.lbl_hostName);
            this.Controls.Add(this.label2);
            this.Controls.Add(this.lbl_macAddress);
            this.Controls.Add(this.lbl_ipAddress);
            this.Controls.Add(this.label3);
            this.Controls.Add(this.label1);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.MaximizeBox = false;
            this.Name = "frm_network";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "Network Information";
            this.Load += new System.EventHandler(this.frm_network_Load);
            this.guna2Panel1.ResumeLayout(false);
            this.guna2Panel1.PerformLayout();
            this.pnl_networkInformation.ResumeLayout(false);
            this.pnl_networkInformation.PerformLayout();
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion

        private Guna.UI2.WinForms.Guna2BorderlessForm bf_form;
        private System.Windows.Forms.Label label1;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.Label lbl_interface;
        private System.Windows.Forms.Label label4;
        private System.Windows.Forms.Label label5;
        private System.Windows.Forms.Label label6;
        private System.Windows.Forms.Label label7;
        private System.Windows.Forms.Label label8;
        private System.Windows.Forms.Label lbl_activeServices;
        private System.Windows.Forms.Label lbl_speed;
        private System.Windows.Forms.Label lbl_hostName;
        private System.Windows.Forms.Label lbl_ipAddress;
        private System.Windows.Forms.Label lbl_intrfaceDesc;
        private System.Windows.Forms.Label lbl_intrfaceStatus;
        private System.Windows.Forms.Label lbl_intrfaceName;
        private Guna.UI2.WinForms.Guna2Button btn_close;
        private Guna.UI2.WinForms.Guna2Button btn_refresh;
        private System.Windows.Forms.Label lbl_macAddress;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.Label lbl_downloadSpeed;
        private Guna.UI2.WinForms.Guna2Shapes guna2Shapes4;
        private Guna.UI2.WinForms.Guna2Panel guna2Panel1;
        private Guna.UI2.WinForms.Guna2WinProgressIndicator loadingProgressSpeed;
        private Guna.UI2.WinForms.Guna2Shapes guna2Shapes2;
        private System.Windows.Forms.Label lbl_uploadSpeed;
        private System.Windows.Forms.Label lbl_hintNetworkAdapter;
        private Guna.UI2.WinForms.Guna2ComboBox cb_selectNetworkAdapter;
        private Guna.UI2.WinForms.Guna2Panel pnl_networkInformation;
    }
}