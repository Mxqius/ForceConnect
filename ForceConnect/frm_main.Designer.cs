namespace ForceConnect
{
    partial class frm_main
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
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(frm_main));
            this.lbl_title = new System.Windows.Forms.Label();
            this.pnl_footer = new Guna.UI2.WinForms.Guna2Panel();
            this.pnl_control = new Guna.UI2.WinForms.Guna2Panel();
            this.bf_cornerForm = new Guna.UI2.WinForms.Guna2BorderlessForm(this.components);
            this.pnl_container = new Guna.UI2.WinForms.Guna2Panel();
            this.lbl_status = new System.Windows.Forms.Label();
            this.wp_dnsProgress = new Guna.UI2.WinForms.Guna2WinProgressIndicator();
            this.shape_connect = new Guna.UI2.WinForms.Guna2Shapes();
            this.cb_selectDns = new Guna.UI2.WinForms.Guna2ComboBox();
            this.pnl_information = new Guna.UI2.WinForms.Guna2Panel();
            this.lbl_latency = new System.Windows.Forms.Label();
            this.lbl_name = new System.Windows.Forms.Label();
            this.lbl_previewAddress = new System.Windows.Forms.Label();
            this.lbl_dnsStatus = new System.Windows.Forms.Label();
            this.label5 = new System.Windows.Forms.Label();
            this.label4 = new System.Windows.Forms.Label();
            this.shapeStatus = new Guna.UI2.WinForms.Guna2Shapes();
            this.label3 = new System.Windows.Forms.Label();
            this.label2 = new System.Windows.Forms.Label();
            this.lbl_message = new System.Windows.Forms.Label();
            this.notifyForm = new System.Windows.Forms.NotifyIcon(this.components);
            this.cms_form = new Guna.UI2.WinForms.Guna2ContextMenuStrip();
            this.tsm_seprator2 = new System.Windows.Forms.ToolStripSeparator();
            this.tsm_status = new System.Windows.Forms.ToolStripMenuItem();
            this.tss_seprator1 = new System.Windows.Forms.ToolStripSeparator();
            this.tsm_exit = new System.Windows.Forms.ToolStripMenuItem();
            this.btn_flushDNS = new Guna.UI2.WinForms.Guna2CircleButton();
            this.iconConnect = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.pb_latencyPicture = new Guna.UI2.WinForms.Guna2PictureBox();
            this.pb_dnsPicture = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.btn_sync = new Guna.UI2.WinForms.Guna2Button();
            this.pb_header = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.btn_minimize = new Guna.UI2.WinForms.Guna2Button();
            this.btn_exit = new Guna.UI2.WinForms.Guna2Button();
            this.btn_home = new Guna.UI2.WinForms.Guna2Button();
            this.btn_about = new Guna.UI2.WinForms.Guna2Button();
            this.btn_explore = new Guna.UI2.WinForms.Guna2Button();
            this.btn_settings = new Guna.UI2.WinForms.Guna2Button();
            this.tsm_forceConnect = new System.Windows.Forms.ToolStripMenuItem();
            this.pnl_footer.SuspendLayout();
            this.pnl_control.SuspendLayout();
            this.pnl_container.SuspendLayout();
            this.pnl_information.SuspendLayout();
            this.cms_form.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.iconConnect)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_dnsPicture)).BeginInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_header)).BeginInit();
            this.SuspendLayout();
            // 
            // lbl_title
            // 
            this.lbl_title.BackColor = System.Drawing.Color.Transparent;
            this.lbl_title.Font = new System.Drawing.Font("JetBrains Mono", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_title.ForeColor = System.Drawing.Color.White;
            this.lbl_title.Location = new System.Drawing.Point(57, 13);
            this.lbl_title.Name = "lbl_title";
            this.lbl_title.Size = new System.Drawing.Size(191, 22);
            this.lbl_title.TabIndex = 0;
            this.lbl_title.Text = "ForceConnect";
            // 
            // pnl_footer
            // 
            this.pnl_footer.Controls.Add(this.btn_home);
            this.pnl_footer.Controls.Add(this.btn_about);
            this.pnl_footer.Controls.Add(this.btn_explore);
            this.pnl_footer.Controls.Add(this.btn_settings);
            this.pnl_footer.Dock = System.Windows.Forms.DockStyle.Bottom;
            this.pnl_footer.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_footer.Location = new System.Drawing.Point(0, 454);
            this.pnl_footer.Name = "pnl_footer";
            this.pnl_footer.Size = new System.Drawing.Size(884, 66);
            this.pnl_footer.TabIndex = 5;
            // 
            // pnl_control
            // 
            this.pnl_control.Controls.Add(this.lbl_title);
            this.pnl_control.Controls.Add(this.pb_header);
            this.pnl_control.Controls.Add(this.btn_minimize);
            this.pnl_control.Controls.Add(this.btn_exit);
            this.pnl_control.Dock = System.Windows.Forms.DockStyle.Top;
            this.pnl_control.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_control.Location = new System.Drawing.Point(0, 0);
            this.pnl_control.Name = "pnl_control";
            this.pnl_control.Size = new System.Drawing.Size(884, 46);
            this.pnl_control.TabIndex = 8;
            this.pnl_control.MouseDown += new System.Windows.Forms.MouseEventHandler(this.pnl_control_MouseDown);
            this.pnl_control.MouseMove += new System.Windows.Forms.MouseEventHandler(this.pnl_control_MouseMove);
            this.pnl_control.MouseUp += new System.Windows.Forms.MouseEventHandler(this.pnl_control_MouseUp);
            // 
            // bf_cornerForm
            // 
            this.bf_cornerForm.AnimateWindow = true;
            this.bf_cornerForm.BorderRadius = 15;
            this.bf_cornerForm.ContainerControl = this;
            this.bf_cornerForm.DockForm = false;
            this.bf_cornerForm.DockIndicatorTransparencyValue = 0.6D;
            this.bf_cornerForm.HasFormShadow = false;
            this.bf_cornerForm.ResizeForm = false;
            this.bf_cornerForm.TransparentWhileDrag = true;
            // 
            // pnl_container
            // 
            this.pnl_container.BackColor = System.Drawing.Color.Transparent;
            this.pnl_container.Controls.Add(this.btn_flushDNS);
            this.pnl_container.Controls.Add(this.lbl_status);
            this.pnl_container.Controls.Add(this.wp_dnsProgress);
            this.pnl_container.Controls.Add(this.iconConnect);
            this.pnl_container.Controls.Add(this.shape_connect);
            this.pnl_container.Controls.Add(this.cb_selectDns);
            this.pnl_container.Controls.Add(this.pnl_information);
            this.pnl_container.Controls.Add(this.lbl_message);
            this.pnl_container.Dock = System.Windows.Forms.DockStyle.Fill;
            this.pnl_container.Location = new System.Drawing.Point(0, 46);
            this.pnl_container.Name = "pnl_container";
            this.pnl_container.Size = new System.Drawing.Size(884, 408);
            this.pnl_container.TabIndex = 9;
            // 
            // lbl_status
            // 
            this.lbl_status.Font = new System.Drawing.Font("JetBrains Mono", 16.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_status.ForeColor = System.Drawing.Color.White;
            this.lbl_status.Location = new System.Drawing.Point(12, 312);
            this.lbl_status.Name = "lbl_status";
            this.lbl_status.Size = new System.Drawing.Size(399, 39);
            this.lbl_status.TabIndex = 22;
            this.lbl_status.Text = "CLICK TO CONNECT";
            this.lbl_status.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // wp_dnsProgress
            // 
            this.wp_dnsProgress.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(236)))), ((int)(((byte)(248)))), ((int)(((byte)(249)))));
            this.wp_dnsProgress.CircleSize = 2F;
            this.wp_dnsProgress.Location = new System.Drawing.Point(134, 110);
            this.wp_dnsProgress.Name = "wp_dnsProgress";
            this.wp_dnsProgress.NumberOfCircles = 9;
            this.wp_dnsProgress.ProgressColor = System.Drawing.Color.DarkGray;
            this.wp_dnsProgress.Size = new System.Drawing.Size(136, 138);
            this.wp_dnsProgress.TabIndex = 23;
            this.wp_dnsProgress.Visible = false;
            // 
            // shape_connect
            // 
            this.shape_connect.BorderColor = System.Drawing.Color.Transparent;
            this.shape_connect.Cursor = System.Windows.Forms.Cursors.Hand;
            this.shape_connect.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(236)))), ((int)(((byte)(248)))), ((int)(((byte)(249)))));
            this.shape_connect.Location = new System.Drawing.Point(90, 69);
            this.shape_connect.Name = "shape_connect";
            this.shape_connect.PolygonSkip = 1;
            this.shape_connect.Rotate = 0F;
            this.shape_connect.Shape = Guna.UI2.WinForms.Enums.ShapeType.Ellipse;
            this.shape_connect.Size = new System.Drawing.Size(226, 219);
            this.shape_connect.TabIndex = 0;
            this.shape_connect.Zoom = 100;
            this.shape_connect.Click += new System.EventHandler(this.connectEvent);
            // 
            // cb_selectDns
            // 
            this.cb_selectDns.BackColor = System.Drawing.Color.Transparent;
            this.cb_selectDns.BorderColor = System.Drawing.Color.Silver;
            this.cb_selectDns.BorderRadius = 10;
            this.cb_selectDns.Cursor = System.Windows.Forms.Cursors.Hand;
            this.cb_selectDns.DrawMode = System.Windows.Forms.DrawMode.OwnerDrawFixed;
            this.cb_selectDns.DropDownStyle = System.Windows.Forms.ComboBoxStyle.DropDownList;
            this.cb_selectDns.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.cb_selectDns.FocusedColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectDns.FocusedState.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(94)))), ((int)(((byte)(148)))), ((int)(((byte)(255)))));
            this.cb_selectDns.Font = new System.Drawing.Font("Segoe UI", 10F);
            this.cb_selectDns.ForeColor = System.Drawing.Color.Silver;
            this.cb_selectDns.ItemHeight = 40;
            this.cb_selectDns.Items.AddRange(new object[] {
            "Shecan",
            "Electro",
            "Radar Game",
            "403.online",
            "Google",
            "Cloudflare"});
            this.cb_selectDns.Location = new System.Drawing.Point(427, 83);
            this.cb_selectDns.Name = "cb_selectDns";
            this.cb_selectDns.Size = new System.Drawing.Size(408, 46);
            this.cb_selectDns.StartIndex = 0;
            this.cb_selectDns.TabIndex = 1;
            this.cb_selectDns.SelectedIndexChanged += new System.EventHandler(this.cb_selectDns_SelectedIndexChanged);
            // 
            // pnl_information
            // 
            this.pnl_information.BorderRadius = 15;
            this.pnl_information.Controls.Add(this.pb_latencyPicture);
            this.pnl_information.Controls.Add(this.pb_dnsPicture);
            this.pnl_information.Controls.Add(this.btn_sync);
            this.pnl_information.Controls.Add(this.lbl_latency);
            this.pnl_information.Controls.Add(this.lbl_name);
            this.pnl_information.Controls.Add(this.lbl_previewAddress);
            this.pnl_information.Controls.Add(this.lbl_dnsStatus);
            this.pnl_information.Controls.Add(this.label5);
            this.pnl_information.Controls.Add(this.label4);
            this.pnl_information.Controls.Add(this.shapeStatus);
            this.pnl_information.Controls.Add(this.label3);
            this.pnl_information.Controls.Add(this.label2);
            this.pnl_information.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_information.Location = new System.Drawing.Point(427, 147);
            this.pnl_information.Name = "pnl_information";
            this.pnl_information.Size = new System.Drawing.Size(408, 204);
            this.pnl_information.TabIndex = 14;
            // 
            // lbl_latency
            // 
            this.lbl_latency.BackColor = System.Drawing.Color.Transparent;
            this.lbl_latency.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_latency.ForeColor = System.Drawing.Color.White;
            this.lbl_latency.Location = new System.Drawing.Point(283, 64);
            this.lbl_latency.Name = "lbl_latency";
            this.lbl_latency.Size = new System.Drawing.Size(69, 19);
            this.lbl_latency.TabIndex = 0;
            this.lbl_latency.Text = "??? ms";
            this.lbl_latency.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // lbl_name
            // 
            this.lbl_name.BackColor = System.Drawing.Color.Transparent;
            this.lbl_name.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_name.ForeColor = System.Drawing.Color.White;
            this.lbl_name.Location = new System.Drawing.Point(65, 64);
            this.lbl_name.Name = "lbl_name";
            this.lbl_name.Size = new System.Drawing.Size(148, 19);
            this.lbl_name.TabIndex = 0;
            this.lbl_name.Text = "Shecan";
            this.lbl_name.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_previewAddress
            // 
            this.lbl_previewAddress.BackColor = System.Drawing.Color.Transparent;
            this.lbl_previewAddress.Cursor = System.Windows.Forms.Cursors.Hand;
            this.lbl_previewAddress.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_previewAddress.ForeColor = System.Drawing.Color.White;
            this.lbl_previewAddress.Location = new System.Drawing.Point(23, 149);
            this.lbl_previewAddress.Name = "lbl_previewAddress";
            this.lbl_previewAddress.Size = new System.Drawing.Size(128, 19);
            this.lbl_previewAddress.TabIndex = 0;
            this.lbl_previewAddress.Text = "178.22.122.100 185.51.200.2";
            this.lbl_previewAddress.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            this.lbl_previewAddress.Click += new System.EventHandler(this.lbl_previewAddress_Click);
            // 
            // lbl_dnsStatus
            // 
            this.lbl_dnsStatus.BackColor = System.Drawing.Color.Transparent;
            this.lbl_dnsStatus.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_dnsStatus.ForeColor = System.Drawing.Color.White;
            this.lbl_dnsStatus.Location = new System.Drawing.Point(257, 149);
            this.lbl_dnsStatus.Name = "lbl_dnsStatus";
            this.lbl_dnsStatus.Size = new System.Drawing.Size(131, 19);
            this.lbl_dnsStatus.TabIndex = 0;
            this.lbl_dnsStatus.Text = "Disconnectd";
            this.lbl_dnsStatus.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // label5
            // 
            this.label5.BackColor = System.Drawing.Color.Transparent;
            this.label5.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label5.ForeColor = System.Drawing.Color.White;
            this.label5.Location = new System.Drawing.Point(240, 111);
            this.label5.Name = "label5";
            this.label5.Size = new System.Drawing.Size(98, 19);
            this.label5.TabIndex = 0;
            this.label5.Text = "Status";
            this.label5.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // label4
            // 
            this.label4.BackColor = System.Drawing.Color.Transparent;
            this.label4.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label4.ForeColor = System.Drawing.Color.White;
            this.label4.Location = new System.Drawing.Point(26, 111);
            this.label4.Name = "label4";
            this.label4.Size = new System.Drawing.Size(101, 19);
            this.label4.TabIndex = 0;
            this.label4.Text = "Address";
            this.label4.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // shapeStatus
            // 
            this.shapeStatus.BackColor = System.Drawing.Color.Transparent;
            this.shapeStatus.BorderColor = System.Drawing.Color.Transparent;
            this.shapeStatus.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(248)))), ((int)(((byte)(114)))), ((int)(((byte)(114)))));
            this.shapeStatus.Location = new System.Drawing.Point(232, 149);
            this.shapeStatus.Name = "shapeStatus";
            this.shapeStatus.PolygonSkip = 1;
            this.shapeStatus.Rotate = 0F;
            this.shapeStatus.Shape = Guna.UI2.WinForms.Enums.ShapeType.Ellipse;
            this.shapeStatus.Size = new System.Drawing.Size(19, 19);
            this.shapeStatus.TabIndex = 7;
            this.shapeStatus.Text = "guna2Shapes1";
            this.shapeStatus.Zoom = 80;
            // 
            // label3
            // 
            this.label3.BackColor = System.Drawing.Color.Transparent;
            this.label3.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label3.ForeColor = System.Drawing.Color.White;
            this.label3.Location = new System.Drawing.Point(240, 26);
            this.label3.Name = "label3";
            this.label3.Size = new System.Drawing.Size(94, 19);
            this.label3.TabIndex = 0;
            this.label3.Text = "Latency";
            this.label3.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // label2
            // 
            this.label2.BackColor = System.Drawing.Color.Transparent;
            this.label2.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.label2.ForeColor = System.Drawing.Color.White;
            this.label2.Location = new System.Drawing.Point(26, 26);
            this.label2.Name = "label2";
            this.label2.Size = new System.Drawing.Size(105, 19);
            this.label2.TabIndex = 0;
            this.label2.Text = "Name";
            this.label2.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_message
            // 
            this.lbl_message.Font = new System.Drawing.Font("JetBrains Mono", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_message.ForeColor = System.Drawing.Color.White;
            this.lbl_message.Location = new System.Drawing.Point(427, 359);
            this.lbl_message.Name = "lbl_message";
            this.lbl_message.Size = new System.Drawing.Size(408, 39);
            this.lbl_message.TabIndex = 13;
            this.lbl_message.Text = "VERSION ???";
            this.lbl_message.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // notifyForm
            // 
            this.notifyForm.BalloonTipTitle = "ForceConnect";
            this.notifyForm.ContextMenuStrip = this.cms_form;
            this.notifyForm.Icon = ((System.Drawing.Icon)(resources.GetObject("notifyForm.Icon")));
            this.notifyForm.Text = "Show Form";
            this.notifyForm.DoubleClick += new System.EventHandler(this.notifyForm_DoubleClick);
            // 
            // cms_form
            // 
            this.cms_form.Font = new System.Drawing.Font("Segoe UI Semibold", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.cms_form.ImageScalingSize = new System.Drawing.Size(20, 20);
            this.cms_form.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.tsm_forceConnect,
            this.tsm_seprator2,
            this.tsm_status,
            this.tss_seprator1,
            this.tsm_exit});
            this.cms_form.Name = "cms_form";
            this.cms_form.RenderStyle.ArrowColor = System.Drawing.Color.Gray;
            this.cms_form.RenderStyle.BorderColor = System.Drawing.Color.Gainsboro;
            this.cms_form.RenderStyle.ColorTable = null;
            this.cms_form.RenderStyle.RoundedEdges = true;
            this.cms_form.RenderStyle.SelectionArrowColor = System.Drawing.Color.White;
            this.cms_form.RenderStyle.SelectionBackColor = System.Drawing.Color.FromArgb(((int)(((byte)(64)))), ((int)(((byte)(64)))), ((int)(((byte)(64)))));
            this.cms_form.RenderStyle.SelectionForeColor = System.Drawing.Color.White;
            this.cms_form.RenderStyle.SeparatorColor = System.Drawing.Color.Gainsboro;
            this.cms_form.RenderStyle.TextRenderingHint = System.Drawing.Text.TextRenderingHint.AntiAlias;
            this.cms_form.Size = new System.Drawing.Size(191, 100);
            // 
            // tsm_seprator2
            // 
            this.tsm_seprator2.Name = "tsm_seprator2";
            this.tsm_seprator2.Size = new System.Drawing.Size(187, 6);
            // 
            // tsm_status
            // 
            this.tsm_status.Name = "tsm_status";
            this.tsm_status.Size = new System.Drawing.Size(190, 28);
            this.tsm_status.Text = "Disconnected";
            this.tsm_status.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // tss_seprator1
            // 
            this.tss_seprator1.Name = "tss_seprator1";
            this.tss_seprator1.Size = new System.Drawing.Size(187, 6);
            // 
            // tsm_exit
            // 
            this.tsm_exit.Name = "tsm_exit";
            this.tsm_exit.Size = new System.Drawing.Size(190, 28);
            this.tsm_exit.Text = "Exit";
            // 
            // btn_flushDNS
            // 
            this.btn_flushDNS.BorderColor = System.Drawing.Color.White;
            this.btn_flushDNS.BorderThickness = 1;
            this.btn_flushDNS.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_flushDNS.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_flushDNS.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_flushDNS.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_flushDNS.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_flushDNS.FillColor = System.Drawing.Color.Transparent;
            this.btn_flushDNS.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_flushDNS.ForeColor = System.Drawing.Color.White;
            this.btn_flushDNS.Image = global::ForceConnect.Properties.Resources.clean;
            this.btn_flushDNS.ImageSize = new System.Drawing.Size(25, 25);
            this.btn_flushDNS.Location = new System.Drawing.Point(342, 161);
            this.btn_flushDNS.Name = "btn_flushDNS";
            this.btn_flushDNS.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_flushDNS.Size = new System.Drawing.Size(52, 52);
            this.btn_flushDNS.TabIndex = 24;
            this.btn_flushDNS.Click += new System.EventHandler(this.btn_flushDNS_Click);
            // 
            // iconConnect
            // 
            this.iconConnect.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(236)))), ((int)(((byte)(248)))), ((int)(((byte)(249)))));
            this.iconConnect.Cursor = System.Windows.Forms.Cursors.Hand;
            this.iconConnect.Image = global::ForceConnect.Properties.Resources.connectIcon;
            this.iconConnect.ImageRotate = 0F;
            this.iconConnect.InitialImage = global::ForceConnect.Properties.Resources.connectIcon;
            this.iconConnect.Location = new System.Drawing.Point(130, 108);
            this.iconConnect.Name = "iconConnect";
            this.iconConnect.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.iconConnect.Size = new System.Drawing.Size(148, 139);
            this.iconConnect.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.iconConnect.TabIndex = 21;
            this.iconConnect.TabStop = false;
            this.iconConnect.Click += new System.EventHandler(this.connectEvent);
            // 
            // pb_latencyPicture
            // 
            this.pb_latencyPicture.FillColor = System.Drawing.Color.Transparent;
            this.pb_latencyPicture.Image = global::ForceConnect.Properties.Resources.signalGreen;
            this.pb_latencyPicture.ImageRotate = 0F;
            this.pb_latencyPicture.Location = new System.Drawing.Point(245, 58);
            this.pb_latencyPicture.Name = "pb_latencyPicture";
            this.pb_latencyPicture.Size = new System.Drawing.Size(32, 29);
            this.pb_latencyPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_latencyPicture.TabIndex = 9;
            this.pb_latencyPicture.TabStop = false;
            // 
            // pb_dnsPicture
            // 
            this.pb_dnsPicture.BackColor = System.Drawing.Color.Transparent;
            this.pb_dnsPicture.Image = global::ForceConnect.Properties.Resources.shecan;
            this.pb_dnsPicture.ImageRotate = 0F;
            this.pb_dnsPicture.Location = new System.Drawing.Point(26, 58);
            this.pb_dnsPicture.Name = "pb_dnsPicture";
            this.pb_dnsPicture.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.pb_dnsPicture.Size = new System.Drawing.Size(29, 29);
            this.pb_dnsPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.StretchImage;
            this.pb_dnsPicture.TabIndex = 8;
            this.pb_dnsPicture.TabStop = false;
            // 
            // btn_sync
            // 
            this.btn_sync.Animated = true;
            this.btn_sync.AutoRoundedCorners = true;
            this.btn_sync.BackColor = System.Drawing.Color.Transparent;
            this.btn_sync.BorderColor = System.Drawing.Color.Transparent;
            this.btn_sync.BorderRadius = 15;
            this.btn_sync.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_sync.DisabledState.FillColor = System.Drawing.Color.Transparent;
            this.btn_sync.FillColor = System.Drawing.Color.Transparent;
            this.btn_sync.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold);
            this.btn_sync.ForeColor = System.Drawing.Color.White;
            this.btn_sync.Image = global::ForceConnect.Properties.Resources.sync;
            this.btn_sync.ImageSize = new System.Drawing.Size(27, 27);
            this.btn_sync.Location = new System.Drawing.Point(357, 58);
            this.btn_sync.Name = "btn_sync";
            this.btn_sync.PressedColor = System.Drawing.Color.Transparent;
            this.btn_sync.Size = new System.Drawing.Size(37, 33);
            this.btn_sync.TabIndex = 4;
            this.btn_sync.TextOffset = new System.Drawing.Point(4, 0);
            this.btn_sync.Click += new System.EventHandler(this.btn_sync_Click);
            // 
            // pb_header
            // 
            this.pb_header.BackColor = System.Drawing.Color.Transparent;
            this.pb_header.Image = global::ForceConnect.Properties.Resources.headerIcon;
            this.pb_header.ImageRotate = 0F;
            this.pb_header.Location = new System.Drawing.Point(12, 6);
            this.pb_header.Name = "pb_header";
            this.pb_header.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.pb_header.Size = new System.Drawing.Size(39, 34);
            this.pb_header.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_header.TabIndex = 8;
            this.pb_header.TabStop = false;
            // 
            // btn_minimize
            // 
            this.btn_minimize.Animated = true;
            this.btn_minimize.BackColor = System.Drawing.Color.Transparent;
            this.btn_minimize.BorderColor = System.Drawing.Color.Transparent;
            this.btn_minimize.BorderRadius = 5;
            this.btn_minimize.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_minimize.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_minimize.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_minimize.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_minimize.FillColor = System.Drawing.Color.Transparent;
            this.btn_minimize.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_minimize.ForeColor = System.Drawing.Color.Transparent;
            this.btn_minimize.Image = global::ForceConnect.Properties.Resources.minimize;
            this.btn_minimize.ImageSize = new System.Drawing.Size(35, 35);
            this.btn_minimize.Location = new System.Drawing.Point(797, 5);
            this.btn_minimize.Name = "btn_minimize";
            this.btn_minimize.Size = new System.Drawing.Size(37, 37);
            this.btn_minimize.TabIndex = 1;
            this.btn_minimize.Click += new System.EventHandler(this.btn_minimize_Click);
            this.btn_minimize.MouseLeave += new System.EventHandler(this.leaveControls);
            this.btn_minimize.MouseMove += new System.Windows.Forms.MouseEventHandler(this.moveControls);
            // 
            // btn_exit
            // 
            this.btn_exit.Animated = true;
            this.btn_exit.BackColor = System.Drawing.Color.Transparent;
            this.btn_exit.BorderColor = System.Drawing.Color.Empty;
            this.btn_exit.BorderRadius = 5;
            this.btn_exit.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_exit.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_exit.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_exit.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_exit.FillColor = System.Drawing.Color.Empty;
            this.btn_exit.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_exit.ForeColor = System.Drawing.Color.Transparent;
            this.btn_exit.Image = global::ForceConnect.Properties.Resources.exit;
            this.btn_exit.ImageSize = new System.Drawing.Size(35, 35);
            this.btn_exit.Location = new System.Drawing.Point(835, 6);
            this.btn_exit.Name = "btn_exit";
            this.btn_exit.Size = new System.Drawing.Size(37, 37);
            this.btn_exit.TabIndex = 0;
            this.btn_exit.Click += new System.EventHandler(this.btn_exit_Click);
            this.btn_exit.MouseLeave += new System.EventHandler(this.leaveControls);
            this.btn_exit.MouseMove += new System.Windows.Forms.MouseEventHandler(this.moveControls);
            // 
            // btn_home
            // 
            this.btn_home.Animated = true;
            this.btn_home.AutoRoundedCorners = true;
            this.btn_home.BackColor = System.Drawing.Color.Transparent;
            this.btn_home.BorderColor = System.Drawing.Color.Transparent;
            this.btn_home.BorderRadius = 23;
            this.btn_home.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_home.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_home.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_home.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_home.FillColor = System.Drawing.Color.Transparent;
            this.btn_home.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold);
            this.btn_home.ForeColor = System.Drawing.Color.White;
            this.btn_home.Image = ((System.Drawing.Image)(resources.GetObject("btn_home.Image")));
            this.btn_home.ImageSize = new System.Drawing.Size(40, 40);
            this.btn_home.Location = new System.Drawing.Point(60, 8);
            this.btn_home.Name = "btn_home";
            this.btn_home.PressedColor = System.Drawing.Color.Transparent;
            this.btn_home.Size = new System.Drawing.Size(173, 48);
            this.btn_home.TabIndex = 0;
            this.btn_home.TextOffset = new System.Drawing.Point(4, 0);
            this.btn_home.Click += new System.EventHandler(this.clickControlMenu);
            this.btn_home.MouseLeave += new System.EventHandler(this.leaveControlMenu);
            this.btn_home.MouseMove += new System.Windows.Forms.MouseEventHandler(this.moveControlMenu);
            // 
            // btn_about
            // 
            this.btn_about.Animated = true;
            this.btn_about.AutoRoundedCorners = true;
            this.btn_about.BackColor = System.Drawing.Color.Transparent;
            this.btn_about.BorderColor = System.Drawing.Color.Transparent;
            this.btn_about.BorderRadius = 23;
            this.btn_about.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_about.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_about.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_about.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_about.FillColor = System.Drawing.Color.Transparent;
            this.btn_about.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_about.ForeColor = System.Drawing.Color.White;
            this.btn_about.Image = global::ForceConnect.Properties.Resources.about;
            this.btn_about.ImageSize = new System.Drawing.Size(40, 40);
            this.btn_about.Location = new System.Drawing.Point(448, 8);
            this.btn_about.Name = "btn_about";
            this.btn_about.PressedColor = System.Drawing.Color.Transparent;
            this.btn_about.Size = new System.Drawing.Size(173, 48);
            this.btn_about.TabIndex = 2;
            this.btn_about.TextOffset = new System.Drawing.Point(4, 0);
            this.btn_about.Click += new System.EventHandler(this.clickControlMenu);
            this.btn_about.MouseLeave += new System.EventHandler(this.leaveControlMenu);
            this.btn_about.MouseMove += new System.Windows.Forms.MouseEventHandler(this.moveControlMenu);
            // 
            // btn_explore
            // 
            this.btn_explore.Animated = true;
            this.btn_explore.AutoRoundedCorners = true;
            this.btn_explore.BackColor = System.Drawing.Color.Transparent;
            this.btn_explore.BorderColor = System.Drawing.Color.Transparent;
            this.btn_explore.BorderRadius = 23;
            this.btn_explore.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_explore.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_explore.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_explore.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_explore.FillColor = System.Drawing.Color.Transparent;
            this.btn_explore.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_explore.ForeColor = System.Drawing.Color.White;
            this.btn_explore.Image = ((System.Drawing.Image)(resources.GetObject("btn_explore.Image")));
            this.btn_explore.ImageSize = new System.Drawing.Size(40, 40);
            this.btn_explore.Location = new System.Drawing.Point(254, 8);
            this.btn_explore.Name = "btn_explore";
            this.btn_explore.PressedColor = System.Drawing.Color.Transparent;
            this.btn_explore.Size = new System.Drawing.Size(173, 48);
            this.btn_explore.TabIndex = 1;
            this.btn_explore.TextOffset = new System.Drawing.Point(4, 0);
            this.btn_explore.Click += new System.EventHandler(this.clickControlMenu);
            this.btn_explore.MouseLeave += new System.EventHandler(this.leaveControlMenu);
            this.btn_explore.MouseMove += new System.Windows.Forms.MouseEventHandler(this.moveControlMenu);
            // 
            // btn_settings
            // 
            this.btn_settings.Animated = true;
            this.btn_settings.AutoRoundedCorners = true;
            this.btn_settings.BackColor = System.Drawing.Color.Transparent;
            this.btn_settings.BorderColor = System.Drawing.Color.Transparent;
            this.btn_settings.BorderRadius = 23;
            this.btn_settings.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_settings.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_settings.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_settings.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_settings.FillColor = System.Drawing.Color.Transparent;
            this.btn_settings.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 10.2F, System.Drawing.FontStyle.Bold);
            this.btn_settings.ForeColor = System.Drawing.Color.White;
            this.btn_settings.Image = global::ForceConnect.Properties.Resources.setting;
            this.btn_settings.ImageSize = new System.Drawing.Size(40, 40);
            this.btn_settings.Location = new System.Drawing.Point(642, 8);
            this.btn_settings.Name = "btn_settings";
            this.btn_settings.PressedColor = System.Drawing.Color.Transparent;
            this.btn_settings.Size = new System.Drawing.Size(173, 48);
            this.btn_settings.TabIndex = 3;
            this.btn_settings.TextOffset = new System.Drawing.Point(4, 0);
            this.btn_settings.Click += new System.EventHandler(this.clickControlMenu);
            this.btn_settings.MouseLeave += new System.EventHandler(this.leaveControlMenu);
            this.btn_settings.MouseMove += new System.Windows.Forms.MouseEventHandler(this.moveControlMenu);
            // 
            // tsm_forceConnect
            // 
            this.tsm_forceConnect.Image = global::ForceConnect.Properties.Resources.headerIcon;
            this.tsm_forceConnect.ImageAlign = System.Drawing.ContentAlignment.MiddleLeft;
            this.tsm_forceConnect.Name = "tsm_forceConnect";
            this.tsm_forceConnect.Size = new System.Drawing.Size(190, 28);
            this.tsm_forceConnect.Text = "ForceConnect";
            this.tsm_forceConnect.TextAlign = System.Drawing.ContentAlignment.MiddleRight;
            // 
            // frm_main
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.ClientSize = new System.Drawing.Size(884, 520);
            this.Controls.Add(this.pnl_container);
            this.Controls.Add(this.pnl_control);
            this.Controls.Add(this.pnl_footer);
            this.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.Icon = ((System.Drawing.Icon)(resources.GetObject("$this.Icon")));
            this.KeyPreview = true;
            this.MaximizeBox = false;
            this.Name = "frm_main";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "ForceConnect";
            this.Load += new System.EventHandler(this.frm_main_Load);
            this.pnl_footer.ResumeLayout(false);
            this.pnl_control.ResumeLayout(false);
            this.pnl_container.ResumeLayout(false);
            this.pnl_information.ResumeLayout(false);
            this.cms_form.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.iconConnect)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_latencyPicture)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_dnsPicture)).EndInit();
            ((System.ComponentModel.ISupportInitialize)(this.pb_header)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion
        private System.Windows.Forms.Label lbl_title;
        private Guna.UI2.WinForms.Guna2Button btn_exit;
        private Guna.UI2.WinForms.Guna2Button btn_minimize;
        private Guna.UI2.WinForms.Guna2Panel pnl_footer;
        private Guna.UI2.WinForms.Guna2Panel pnl_control;
        private Guna.UI2.WinForms.Guna2Button btn_settings;
        private Guna.UI2.WinForms.Guna2Button btn_home;
        private Guna.UI2.WinForms.Guna2Button btn_explore;
        private Guna.UI2.WinForms.Guna2BorderlessForm bf_cornerForm;
        private Guna.UI2.WinForms.Guna2Panel pnl_container;
        private Guna.UI2.WinForms.Guna2ComboBox cb_selectDns;
        private Guna.UI2.WinForms.Guna2Shapes shape_connect;
        private Guna.UI2.WinForms.Guna2Panel pnl_information;
        private Guna.UI2.WinForms.Guna2CirclePictureBox pb_dnsPicture;
        private Guna.UI2.WinForms.Guna2Button btn_sync;
        private System.Windows.Forms.Label lbl_latency;
        private System.Windows.Forms.Label lbl_name;
        private System.Windows.Forms.Label lbl_previewAddress;
        private System.Windows.Forms.Label lbl_dnsStatus;
        private System.Windows.Forms.Label label5;
        private System.Windows.Forms.Label label4;
        private Guna.UI2.WinForms.Guna2Shapes shapeStatus;
        private System.Windows.Forms.Label label3;
        private System.Windows.Forms.Label label2;
        private System.Windows.Forms.Label lbl_message;
        private Guna.UI2.WinForms.Guna2CirclePictureBox pb_header;
        private Guna.UI2.WinForms.Guna2CirclePictureBox iconConnect;
        private System.Windows.Forms.Label lbl_status;
        private Guna.UI2.WinForms.Guna2WinProgressIndicator wp_dnsProgress;
        private Guna.UI2.WinForms.Guna2Button btn_about;
        private Guna.UI2.WinForms.Guna2PictureBox pb_latencyPicture;
        private System.Windows.Forms.NotifyIcon notifyForm;
        private Guna.UI2.WinForms.Guna2ContextMenuStrip cms_form;
        private System.Windows.Forms.ToolStripSeparator tss_seprator1;
        private System.Windows.Forms.ToolStripMenuItem tsm_exit;
        private System.Windows.Forms.ToolStripMenuItem tsm_forceConnect;
        private System.Windows.Forms.ToolStripMenuItem tsm_status;
        private System.Windows.Forms.ToolStripSeparator tsm_seprator2;
        private Guna.UI2.WinForms.Guna2CircleButton btn_flushDNS;
    }
}

