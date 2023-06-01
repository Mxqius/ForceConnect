using ForceConnect.API;
using ForceConnect.Services;
using Guna.UI2.WinForms;
using System;
using System.Drawing;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_main : Form
    {
        private bool dragging = false;
        private Point dragCursorPoint;
        private Point dragFormPoint;

        public DnsAddress Shecan, Electro, Online403, Google, Cloudflare, RadarGame;
        public DnsAddress currentDNS;
        private Guna2Button currentSelectedMenuOption;

        private bool _connected;
        private bool _internetConnection = true;
        public frm_main()
        {
            InitializeComponent();
            currentSelectedMenuOption = btn_home;
            btn_home.ImageSize = new Size(45, 45);
            btn_home.FillColor = Color.FromArgb(32, 32, 32);
            btn_home.Text = "HOME";
            // Setup default servers
            Shecan = new DnsAddress()
            {
                dnsAddress = new string[] { "178.22.122.100", "185.51.200.2" },
                Latency = 170,
                Name = "Shecan",
                Picture = Properties.Resources.shecan
            };
            Electro = new DnsAddress()
            {
                dnsAddress = new string[] { "78.157.42.101", "78.157.42.100" },
                Latency = 100,
                Name = "Electro",
                Picture = Properties.Resources.electro
            };
            RadarGame = new DnsAddress()
            {
                dnsAddress = new string[] { "10.202.10.10", "10.202.10.11" },
                Latency = 70,
                Name = "RadarGame",
                Picture = Properties.Resources.radargame
            };
            Online403 = new DnsAddress()
            {
                dnsAddress = new string[] { "10.202.10.202", "10.202.10.102" },
                Latency = 80,
                Name = "403.Online",
                Picture = Properties.Resources.online403
            };
            Google = new DnsAddress()
            {
                dnsAddress = new string[] { "8.8.8.8", "8.8.4.4" },
                Latency = 24,
                Name = "Google",
                Picture = Properties.Resources.google
            };
            Cloudflare = new DnsAddress()
            {
                dnsAddress = new string[] { "1.1.1.1", "1.0.0.1" },
                Latency = 50,
                Name = "Cloudflare",
                Picture = Properties.Resources.cloudflare
            };

        }
        private void checkInternetConnection()
        {
            if (Latency.MeasureLatency("google.com") == -1)
                changeAppStatus(false);
            else
                changeAppStatus(true);
        }
        private void changeAppStatus(bool internetConnection)
        {
            _internetConnection = internetConnection;
            lbl_status.Text = internetConnection ? "CLICK TO CONNECT" : "NO CONNECTION";
        }
        private async Task<bool> delay(int milisecound)
        {
            return await Task<bool>.Run(() =>
            {
                Thread.Sleep(milisecound);
                return true;
            });
        }
        private void btn_exit_Click(object sender, EventArgs e)
        {
            Application.Exit();
        }

        private void btn_minimize_Click(object sender, EventArgs e)
        {
            this.WindowState = FormWindowState.Minimized;
        }

        private void moveControls(object sender, MouseEventArgs e)
        {
            ((Guna2Button)sender).ImageSize = new Size(40, 40);
        }

        private void leaveControls(object sender, EventArgs e)
        {
            ((Guna2Button)sender).ImageSize = new Size(35, 35);
        }

        private void cb_selectDns_SelectedIndexChanged(object sender, EventArgs e)
        {
            changeServer();
        }

        private void changeServer()
        {
            switch (cb_selectDns.Text)
            {
                case "Shecan":
                    currentDNS = Shecan;
                    break;

                case "Electro":
                    currentDNS = Electro;
                    break;

                case "Google":
                    currentDNS = Google;
                    break;

                case "Radar Game":
                    currentDNS = RadarGame;
                    break;

                case "Cloudflare":
                    currentDNS = Cloudflare;
                    break;

                case "403.online":
                    currentDNS = Online403;
                    break;
            }
            showInformation();
        }
        private async void showInformation()
        {
            pb_dnsPicture.Image = currentDNS.Picture;
            lbl_latency.Text = currentDNS.Latency.ToString() + " ms";
            lbl_name.Text = currentDNS.Name;
            lbl_previewAddress.Text = currentDNS.dnsAddress[0] + " " + currentDNS.dnsAddress[1];
            if (_internetConnection)
                await syncLatency();
            else
                lbl_latency.Text = "-1 ms";
        }

        private void pnl_control_MouseDown(object sender, MouseEventArgs e)
        {
            dragging = true;
            dragCursorPoint = Cursor.Position;
            dragFormPoint = this.Location;
        }

        private void pnl_control_MouseMove(object sender, MouseEventArgs e)
        {
            if (dragging)
            {
                Point dif = Point.Subtract(Cursor.Position, new Size(dragCursorPoint));
                this.Location = Point.Add(dragFormPoint, new Size(dif));
            }
        }

        private void pnl_control_MouseUp(object sender, MouseEventArgs e)
        {
            dragging = false;
        }

        private async void lbl_previewAddress_Click(object sender, EventArgs e)
        {
            Clipboard.SetText(lbl_previewAddress.Text);
            lbl_previewAddress.Text = "Address copied";
            await delay(1000);
            lbl_previewAddress.Text = currentDNS.dnsAddress[0] + " " + currentDNS.dnsAddress[1];
        }
        private async Task<bool> syncLatency()
        {
            return await Task.Run(() =>
            {
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_latency.Text = Latency.MeasureLatency("google.com").ToString() + " ms";
                }));
                return true;
            });

        }
        private async void btn_sync_Click(object sender, EventArgs e)
        {
            await syncLatency();
            checkInternetConnection();
        }

        private void frm_main_Load(object sender, EventArgs e)
        {
            changeServer();
            checkInternetConnection();
        }

        private async void connectEvent(object sender, EventArgs e)
        {
            if (!_internetConnection) return;
            if (!_connected)
            {
                shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                lbl_dnsStatus.Text = "Connecting";
                lbl_status.Text = "APPLIYNG DNS";
                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                await delay(3000);
                DnsManager.setDNS(currentDNS.dnsAddress);
                shapeStatus.FillColor = Color.FromArgb(3, 201, 136);
                lbl_dnsStatus.Text = "Connected";
                lbl_status.Text = "CLICK TO DISCONNECT";
                wp_dnsProgress.Visible = false;
                wp_dnsProgress.Stop();
                // Sync Latency
                lbl_latency.Text = "Loading..";
                await delay(2000);
                await syncLatency();
            }
            else
            {
                shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                lbl_dnsStatus.Text = "Disconnecting";
                lbl_status.Text = "RESTORING";
                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                await delay(3000);
                DnsManager.clearDNS();
                shapeStatus.FillColor = Color.FromArgb(248, 114, 114);
                lbl_dnsStatus.Text = "Disconnected";
                lbl_status.Text = "CLICK TO CONNECT";
                wp_dnsProgress.Visible = false;
                wp_dnsProgress.Stop();
                // Sync Latency
                lbl_latency.Text = "Loading..";
                await delay(2000);
                await syncLatency();
            }
            _connected = !_connected;
        }
        private void selectMenuOption(object sender, bool clickEvent)
        {
            if (((Guna2Button)sender) == currentSelectedMenuOption && !clickEvent) return;
            ((Guna2Button)sender).ImageSize = new Size(45, 45);
            ((Guna2Button)sender).FillColor = Color.FromArgb(32, 32, 32);
            switch (((Guna2Button)sender).Name)
            {
                case "btn_settings":
                    ((Guna2Button)sender).Text = "SETTINGS";
                    break;
                case "btn_explore":
                    ((Guna2Button)sender).Text = "EXPLORE";
                    break;
                case "btn_home":
                    ((Guna2Button)sender).Text = "HOME";
                    break;
            }
        }
        private void moveControlMenu(object sender, MouseEventArgs e)
        {
            selectMenuOption(sender, false);
        }

        private void leaveControlMenu(object sender, EventArgs e)
        {
            if (((Guna2Button)sender) == currentSelectedMenuOption) return;
            ((Guna2Button)sender).ImageSize = new Size(40, 40);
            ((Guna2Button)sender).FillColor = Color.Transparent;
            ((Guna2Button)sender).Text = string.Empty;
        }

        private void clickControlMenu(object sender, EventArgs e)
        {
            currentSelectedMenuOption.ImageSize = new Size(40, 40);
            currentSelectedMenuOption.FillColor = Color.Transparent;
            currentSelectedMenuOption.Text = string.Empty;
            currentSelectedMenuOption = ((Guna2Button)sender);
            selectMenuOption(sender, true);
        }
    }
}
