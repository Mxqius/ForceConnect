namespace ForceConnect
{
    partial class frm_messageBox
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
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(frm_messageBox));
            this.lbl_caption = new System.Windows.Forms.Label();
            this.lbl_text = new System.Windows.Forms.Label();
            this.btn_primary = new Guna.UI2.WinForms.Guna2Button();
            this.btn_secoundry = new Guna.UI2.WinForms.Guna2Button();
            this.bf_messageBox = new Guna.UI2.WinForms.Guna2BorderlessForm(this.components);
            this.btn_three = new Guna.UI2.WinForms.Guna2Button();
            this.pnl_color = new Guna.UI2.WinForms.Guna2Panel();
            this.pb_header = new Guna.UI2.WinForms.Guna2PictureBox();
            ((System.ComponentModel.ISupportInitialize)(this.pb_header)).BeginInit();
            this.SuspendLayout();
            // 
            // lbl_caption
            // 
            this.lbl_caption.BackColor = System.Drawing.Color.Transparent;
            this.lbl_caption.Font = new System.Drawing.Font("Segoe UI", 12F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_caption.ForeColor = System.Drawing.Color.White;
            this.lbl_caption.Location = new System.Drawing.Point(111, 45);
            this.lbl_caption.Name = "lbl_caption";
            this.lbl_caption.Size = new System.Drawing.Size(257, 28);
            this.lbl_caption.TabIndex = 1;
            this.lbl_caption.Text = "Caption";
            this.lbl_caption.TextAlign = System.Drawing.ContentAlignment.MiddleLeft;
            // 
            // lbl_text
            // 
            this.lbl_text.Font = new System.Drawing.Font("Segoe UI", 9F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_text.ForeColor = System.Drawing.Color.White;
            this.lbl_text.Location = new System.Drawing.Point(112, 91);
            this.lbl_text.Name = "lbl_text";
            this.lbl_text.Size = new System.Drawing.Size(469, 52);
            this.lbl_text.TabIndex = 1;
            this.lbl_text.Text = "Message Content";
            // 
            // btn_primary
            // 
            this.btn_primary.Animated = true;
            this.btn_primary.BorderRadius = 5;
            this.btn_primary.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_primary.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_primary.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_primary.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_primary.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_primary.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(0)))), ((int)(((byte)(110)))), ((int)(((byte)(228)))));
            this.btn_primary.Font = new System.Drawing.Font("JetBrains Mono", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_primary.ForeColor = System.Drawing.Color.White;
            this.btn_primary.Location = new System.Drawing.Point(421, 178);
            this.btn_primary.Name = "btn_primary";
            this.btn_primary.Size = new System.Drawing.Size(156, 35);
            this.btn_primary.TabIndex = 2;
            this.btn_primary.Text = "PRIMARY";
            this.btn_primary.Click += new System.EventHandler(this.btn_primary_Click);
            // 
            // btn_secoundry
            // 
            this.btn_secoundry.Animated = true;
            this.btn_secoundry.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(0)))), ((int)(((byte)(110)))), ((int)(((byte)(228)))));
            this.btn_secoundry.BorderRadius = 5;
            this.btn_secoundry.BorderThickness = 1;
            this.btn_secoundry.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_secoundry.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_secoundry.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_secoundry.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_secoundry.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_secoundry.FillColor = System.Drawing.Color.Transparent;
            this.btn_secoundry.Font = new System.Drawing.Font("JetBrains Mono", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_secoundry.ForeColor = System.Drawing.Color.White;
            this.btn_secoundry.Location = new System.Drawing.Point(259, 178);
            this.btn_secoundry.Name = "btn_secoundry";
            this.btn_secoundry.Size = new System.Drawing.Size(156, 35);
            this.btn_secoundry.TabIndex = 2;
            this.btn_secoundry.Text = "SECONDRY";
            this.btn_secoundry.Click += new System.EventHandler(this.btn_secoundry_Click);
            // 
            // bf_messageBox
            // 
            this.bf_messageBox.BorderRadius = 5;
            this.bf_messageBox.ContainerControl = this;
            this.bf_messageBox.DockForm = false;
            this.bf_messageBox.DockIndicatorTransparencyValue = 0.6D;
            this.bf_messageBox.DragForm = false;
            this.bf_messageBox.HasFormShadow = false;
            this.bf_messageBox.ResizeForm = false;
            this.bf_messageBox.TransparentWhileDrag = true;
            // 
            // btn_three
            // 
            this.btn_three.Animated = true;
            this.btn_three.BorderRadius = 5;
            this.btn_three.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_three.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_three.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_three.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_three.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_three.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(0)))), ((int)(((byte)(110)))), ((int)(((byte)(228)))));
            this.btn_three.Font = new System.Drawing.Font("JetBrains Mono", 10.2F, System.Drawing.FontStyle.Regular, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.btn_three.ForeColor = System.Drawing.Color.White;
            this.btn_three.Location = new System.Drawing.Point(97, 178);
            this.btn_three.Name = "btn_three";
            this.btn_three.Size = new System.Drawing.Size(156, 35);
            this.btn_three.TabIndex = 2;
            this.btn_three.Text = "THREE";
            this.btn_three.Click += new System.EventHandler(this.btn_three_Click);
            // 
            // pnl_color
            // 
            this.pnl_color.Dock = System.Windows.Forms.DockStyle.Top;
            this.pnl_color.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(7)))), ((int)(((byte)(191)))), ((int)(((byte)(49)))));
            this.pnl_color.Location = new System.Drawing.Point(0, 0);
            this.pnl_color.Name = "pnl_color";
            this.pnl_color.Size = new System.Drawing.Size(596, 10);
            this.pnl_color.TabIndex = 3;
            // 
            // pb_header
            // 
            this.pb_header.FillColor = System.Drawing.Color.Transparent;
            this.pb_header.Image = ((System.Drawing.Image)(resources.GetObject("pb_header.Image")));
            this.pb_header.ImageRotate = 0F;
            this.pb_header.Location = new System.Drawing.Point(15, 60);
            this.pb_header.Name = "pb_header";
            this.pb_header.Size = new System.Drawing.Size(78, 69);
            this.pb_header.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_header.TabIndex = 0;
            this.pb_header.TabStop = false;
            // 
            // frm_messageBox
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 16F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.ClientSize = new System.Drawing.Size(596, 235);
            this.ControlBox = false;
            this.Controls.Add(this.lbl_caption);
            this.Controls.Add(this.pnl_color);
            this.Controls.Add(this.btn_three);
            this.Controls.Add(this.btn_secoundry);
            this.Controls.Add(this.btn_primary);
            this.Controls.Add(this.lbl_text);
            this.Controls.Add(this.pb_header);
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.KeyPreview = true;
            this.Name = "frm_messageBox";
            this.ShowIcon = false;
            this.ShowInTaskbar = false;
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "MessageBox";
            ((System.ComponentModel.ISupportInitialize)(this.pb_header)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2PictureBox pb_header;
        private System.Windows.Forms.Label lbl_caption;
        private System.Windows.Forms.Label lbl_text;
        private Guna.UI2.WinForms.Guna2Button btn_primary;
        private Guna.UI2.WinForms.Guna2Button btn_secoundry;
        private Guna.UI2.WinForms.Guna2BorderlessForm bf_messageBox;
        private Guna.UI2.WinForms.Guna2Button btn_three;
        private Guna.UI2.WinForms.Guna2Panel pnl_color;
    }
}