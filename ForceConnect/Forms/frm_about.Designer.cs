namespace ForceConnect
{
    partial class frm_about
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
            System.ComponentModel.ComponentResourceManager resources = new System.ComponentModel.ComponentResourceManager(typeof(frm_about));
            this.pnl_descriptionAbout = new Guna.UI2.WinForms.Guna2Panel();
            this.guna2Shapes1 = new Guna.UI2.WinForms.Guna2Shapes();
            this.lbl_description = new System.Windows.Forms.Label();
            this.lbl_titleDescription = new System.Windows.Forms.Label();
            this.pnl_developreCard = new Guna.UI2.WinForms.Guna2Panel();
            this.btn_github = new Guna.UI2.WinForms.Guna2CircleButton();
            this.btn_instagram = new Guna.UI2.WinForms.Guna2CircleButton();
            this.btn_telegram = new Guna.UI2.WinForms.Guna2CircleButton();
            this.lbl_descriptionDeveloper = new System.Windows.Forms.Label();
            this.lbl_nameDeveloper = new System.Windows.Forms.Label();
            this.pb_aboutPicture = new Guna.UI2.WinForms.Guna2CirclePictureBox();
            this.pnl_descriptionAbout.SuspendLayout();
            this.pnl_developreCard.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.pb_aboutPicture)).BeginInit();
            this.SuspendLayout();
            // 
            // pnl_descriptionAbout
            // 
            this.pnl_descriptionAbout.BackColor = System.Drawing.Color.Transparent;
            this.pnl_descriptionAbout.BorderColor = System.Drawing.Color.FromArgb(((int)(((byte)(73)))), ((int)(((byte)(73)))), ((int)(((byte)(73)))));
            this.pnl_descriptionAbout.BorderRadius = 10;
            this.pnl_descriptionAbout.BorderThickness = 2;
            this.pnl_descriptionAbout.Controls.Add(this.guna2Shapes1);
            this.pnl_descriptionAbout.Controls.Add(this.lbl_description);
            this.pnl_descriptionAbout.Controls.Add(this.lbl_titleDescription);
            this.pnl_descriptionAbout.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(38)))), ((int)(((byte)(38)))), ((int)(((byte)(38)))));
            this.pnl_descriptionAbout.Location = new System.Drawing.Point(430, 35);
            this.pnl_descriptionAbout.Name = "pnl_descriptionAbout";
            this.pnl_descriptionAbout.Size = new System.Drawing.Size(404, 325);
            this.pnl_descriptionAbout.TabIndex = 0;
            // 
            // guna2Shapes1
            // 
            this.guna2Shapes1.BorderThickness = 0;
            this.guna2Shapes1.FillColor = System.Drawing.Color.Silver;
            this.guna2Shapes1.LineThickness = 1;
            this.guna2Shapes1.Location = new System.Drawing.Point(25, 65);
            this.guna2Shapes1.Name = "guna2Shapes1";
            this.guna2Shapes1.PolygonSkip = 1;
            this.guna2Shapes1.Rotate = 0F;
            this.guna2Shapes1.Shape = Guna.UI2.WinForms.Enums.ShapeType.Line;
            this.guna2Shapes1.Size = new System.Drawing.Size(68, 10);
            this.guna2Shapes1.TabIndex = 25;
            this.guna2Shapes1.Zoom = 100;
            // 
            // lbl_description
            // 
            this.lbl_description.FlatStyle = System.Windows.Forms.FlatStyle.Popup;
            this.lbl_description.Font = new System.Drawing.Font("Rubik SemiBold", 9F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_description.ForeColor = System.Drawing.Color.WhiteSmoke;
            this.lbl_description.Location = new System.Drawing.Point(22, 68);
            this.lbl_description.Name = "lbl_description";
            this.lbl_description.Size = new System.Drawing.Size(360, 231);
            this.lbl_description.TabIndex = 1;
            this.lbl_description.Text = resources.GetString("lbl_description.Text");
            // 
            // lbl_titleDescription
            // 
            this.lbl_titleDescription.AutoSize = true;
            this.lbl_titleDescription.Font = new System.Drawing.Font("Rubik", 10.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_titleDescription.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(236)))), ((int)(((byte)(248)))), ((int)(((byte)(249)))));
            this.lbl_titleDescription.Location = new System.Drawing.Point(18, 29);
            this.lbl_titleDescription.Name = "lbl_titleDescription";
            this.lbl_titleDescription.Size = new System.Drawing.Size(224, 23);
            this.lbl_titleDescription.TabIndex = 1;
            this.lbl_titleDescription.Text = "What is ForceConnect ?";
            // 
            // pnl_developreCard
            // 
            this.pnl_developreCard.BackColor = System.Drawing.Color.Transparent;
            this.pnl_developreCard.BorderRadius = 10;
            this.pnl_developreCard.Controls.Add(this.btn_github);
            this.pnl_developreCard.Controls.Add(this.btn_instagram);
            this.pnl_developreCard.Controls.Add(this.btn_telegram);
            this.pnl_developreCard.Controls.Add(this.lbl_descriptionDeveloper);
            this.pnl_developreCard.Controls.Add(this.lbl_nameDeveloper);
            this.pnl_developreCard.Controls.Add(this.pb_aboutPicture);
            this.pnl_developreCard.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(238)))), ((int)(((byte)(238)))), ((int)(((byte)(238)))));
            this.pnl_developreCard.Location = new System.Drawing.Point(59, 35);
            this.pnl_developreCard.Name = "pnl_developreCard";
            this.pnl_developreCard.Size = new System.Drawing.Size(339, 325);
            this.pnl_developreCard.TabIndex = 0;
            // 
            // btn_github
            // 
            this.btn_github.Animated = true;
            this.btn_github.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_github.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_github.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_github.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_github.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_github.FillColor = System.Drawing.Color.Transparent;
            this.btn_github.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_github.ForeColor = System.Drawing.Color.White;
            this.btn_github.Image = global::ForceConnect.Properties.Resources.github48;
            this.btn_github.ImageSize = new System.Drawing.Size(35, 35);
            this.btn_github.Location = new System.Drawing.Point(205, 253);
            this.btn_github.Name = "btn_github";
            this.btn_github.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_github.Size = new System.Drawing.Size(48, 46);
            this.btn_github.TabIndex = 2;
            this.btn_github.Click += new System.EventHandler(this.btn_github_Click);
            this.btn_github.MouseLeave += new System.EventHandler(this.unFocusSocialMedia);
            this.btn_github.MouseMove += new System.Windows.Forms.MouseEventHandler(this.focusSocialMedia);
            // 
            // btn_instagram
            // 
            this.btn_instagram.Animated = true;
            this.btn_instagram.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_instagram.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_instagram.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_instagram.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_instagram.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_instagram.FillColor = System.Drawing.Color.Transparent;
            this.btn_instagram.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_instagram.ForeColor = System.Drawing.Color.White;
            this.btn_instagram.Image = global::ForceConnect.Properties.Resources.instagram48;
            this.btn_instagram.ImageSize = new System.Drawing.Size(35, 35);
            this.btn_instagram.Location = new System.Drawing.Point(147, 253);
            this.btn_instagram.Name = "btn_instagram";
            this.btn_instagram.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_instagram.Size = new System.Drawing.Size(48, 46);
            this.btn_instagram.TabIndex = 2;
            this.btn_instagram.Click += new System.EventHandler(this.btn_instagram_Click);
            this.btn_instagram.MouseLeave += new System.EventHandler(this.unFocusSocialMedia);
            this.btn_instagram.MouseMove += new System.Windows.Forms.MouseEventHandler(this.focusSocialMedia);
            // 
            // btn_telegram
            // 
            this.btn_telegram.Animated = true;
            this.btn_telegram.Cursor = System.Windows.Forms.Cursors.Hand;
            this.btn_telegram.DisabledState.BorderColor = System.Drawing.Color.DarkGray;
            this.btn_telegram.DisabledState.CustomBorderColor = System.Drawing.Color.DarkGray;
            this.btn_telegram.DisabledState.FillColor = System.Drawing.Color.FromArgb(((int)(((byte)(169)))), ((int)(((byte)(169)))), ((int)(((byte)(169)))));
            this.btn_telegram.DisabledState.ForeColor = System.Drawing.Color.FromArgb(((int)(((byte)(141)))), ((int)(((byte)(141)))), ((int)(((byte)(141)))));
            this.btn_telegram.FillColor = System.Drawing.Color.Transparent;
            this.btn_telegram.Font = new System.Drawing.Font("Segoe UI", 9F);
            this.btn_telegram.ForeColor = System.Drawing.Color.White;
            this.btn_telegram.Image = global::ForceConnect.Properties.Resources.telegram48;
            this.btn_telegram.ImageSize = new System.Drawing.Size(35, 35);
            this.btn_telegram.Location = new System.Drawing.Point(89, 253);
            this.btn_telegram.Name = "btn_telegram";
            this.btn_telegram.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.btn_telegram.Size = new System.Drawing.Size(48, 46);
            this.btn_telegram.TabIndex = 2;
            this.btn_telegram.Click += new System.EventHandler(this.btn_telegram_Click);
            this.btn_telegram.MouseLeave += new System.EventHandler(this.unFocusSocialMedia);
            this.btn_telegram.MouseMove += new System.Windows.Forms.MouseEventHandler(this.focusSocialMedia);
            // 
            // lbl_descriptionDeveloper
            // 
            this.lbl_descriptionDeveloper.Font = new System.Drawing.Font("Rubik SemiBold", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_descriptionDeveloper.Location = new System.Drawing.Point(66, 195);
            this.lbl_descriptionDeveloper.Name = "lbl_descriptionDeveloper";
            this.lbl_descriptionDeveloper.Size = new System.Drawing.Size(209, 15);
            this.lbl_descriptionDeveloper.TabIndex = 1;
            this.lbl_descriptionDeveloper.Text = "Founder , Lead Developer";
            this.lbl_descriptionDeveloper.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // lbl_nameDeveloper
            // 
            this.lbl_nameDeveloper.Font = new System.Drawing.Font("Rubik", 13.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.lbl_nameDeveloper.Location = new System.Drawing.Point(94, 157);
            this.lbl_nameDeveloper.Name = "lbl_nameDeveloper";
            this.lbl_nameDeveloper.Size = new System.Drawing.Size(147, 28);
            this.lbl_nameDeveloper.TabIndex = 1;
            this.lbl_nameDeveloper.Text = "MXQIUS";
            this.lbl_nameDeveloper.TextAlign = System.Drawing.ContentAlignment.MiddleCenter;
            // 
            // pb_aboutPicture
            // 
            this.pb_aboutPicture.Cursor = System.Windows.Forms.Cursors.Default;
            this.pb_aboutPicture.FillColor = System.Drawing.Color.Transparent;
            this.pb_aboutPicture.Image = global::ForceConnect.Properties.Resources.mxqius;
            this.pb_aboutPicture.ImageRotate = 0F;
            this.pb_aboutPicture.Location = new System.Drawing.Point(111, 29);
            this.pb_aboutPicture.Name = "pb_aboutPicture";
            this.pb_aboutPicture.ShadowDecoration.Mode = Guna.UI2.WinForms.Enums.ShadowMode.Circle;
            this.pb_aboutPicture.Size = new System.Drawing.Size(109, 111);
            this.pb_aboutPicture.SizeMode = System.Windows.Forms.PictureBoxSizeMode.Zoom;
            this.pb_aboutPicture.TabIndex = 0;
            this.pb_aboutPicture.TabStop = false;
            this.pb_aboutPicture.Click += new System.EventHandler(this.pb_aboutPicture_Click);
            // 
            // frm_about
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(8F, 15F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.BackColor = System.Drawing.Color.FromArgb(((int)(((byte)(33)))), ((int)(((byte)(33)))), ((int)(((byte)(33)))));
            this.ClientSize = new System.Drawing.Size(884, 408);
            this.Controls.Add(this.pnl_developreCard);
            this.Controls.Add(this.pnl_descriptionAbout);
            this.Font = new System.Drawing.Font("Museo Sans Cyrl 900", 7.8F, System.Drawing.FontStyle.Bold, System.Drawing.GraphicsUnit.Point, ((byte)(0)));
            this.FormBorderStyle = System.Windows.Forms.FormBorderStyle.None;
            this.MaximizeBox = false;
            this.Name = "frm_about";
            this.StartPosition = System.Windows.Forms.FormStartPosition.CenterScreen;
            this.Text = "About";
            this.pnl_descriptionAbout.ResumeLayout(false);
            this.pnl_descriptionAbout.PerformLayout();
            this.pnl_developreCard.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.pb_aboutPicture)).EndInit();
            this.ResumeLayout(false);

        }

        #endregion

        private Guna.UI2.WinForms.Guna2Panel pnl_descriptionAbout;
        private Guna.UI2.WinForms.Guna2Panel pnl_developreCard;
        private Guna.UI2.WinForms.Guna2CirclePictureBox pb_aboutPicture;
        private System.Windows.Forms.Label lbl_nameDeveloper;
        private System.Windows.Forms.Label lbl_descriptionDeveloper;
        private Guna.UI2.WinForms.Guna2CircleButton btn_telegram;
        private Guna.UI2.WinForms.Guna2CircleButton btn_github;
        private Guna.UI2.WinForms.Guna2CircleButton btn_instagram;
        private System.Windows.Forms.Label lbl_titleDescription;
        private System.Windows.Forms.Label lbl_description;
        private Guna.UI2.WinForms.Guna2Shapes guna2Shapes1;
    }
}