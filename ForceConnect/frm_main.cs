using ForceConnect.API;
using ForceConnect.DNS;
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
        private Point dragCursorPoint, dragFormPoint;

        private DnsAddressItems DnsAddress;
        public DnsAddress currentDNS, connectedDNS;
        private Guna2Button currentSelectedMenuOption;
        public Form currentFormLoaded;

        private bool _connected, pendingRequest, _internetConnection = true;
        public frm_main()
        {
            InitializeComponent();
            currentSelectedMenuOption = btn_home;

            btn_home.ImageSize = new Size(45, 45);
            btn_home.FillColor = Color.FromArgb(32, 32, 32);
            btn_home.Text = "HOME";

            DnsAddress = new DnsAddressItems();

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
            if (_internetConnection && _connected) return;

            lbl_status.Text = (internetConnection) ? "CLICK TO CONNECT" : "NO CONNECTION";
            iconConnect.Image = (internetConnection) ? Properties.Resources.connectIcon : Properties.Resources.no_internet;

        }
        public async Task<bool> delay(int milisecound)
        {
            return await Task<bool>.Run(() =>
            {
                Thread.Sleep(milisecound);
                return true;
            });
        }
        private void btn_exit_Click(object sender, EventArgs e)
        {
            disconnectFromApp();
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
            if (connectedDNS != null)
            {
                if (connectedDNS.Name.ToLower() != cb_selectDns.Text.ToLower() && _connected)
                    lbl_message.Text = $"YOU ARE STILL CONNECT TO PERVIOSLY DNS ({connectedDNS.Name})";
                if (connectedDNS.Name.ToLower() == cb_selectDns.Text.ToLower())
                    lbl_message.Text = "BETA VERSION";
            }
            changeServer();
        }

        private void changeServer()
        {
            switch (cb_selectDns.Text)
            {
                case "Shecan":
                    currentDNS = DnsAddress.Shecan;
                    break;

                case "Electro":
                    currentDNS = DnsAddress.Electro;
                    break;

                case "Google":
                    currentDNS = DnsAddress.Google;
                    break;

                case "Radar Game":
                    currentDNS = DnsAddress.RadarGame;
                    break;

                case "Cloudflare":
                    currentDNS = DnsAddress.Cloudflare;
                    break;

                case "403.online":
                    currentDNS = DnsAddress.Online403;
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
                await syncLatencyDNS();
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
        private async Task<bool> syncLatencyDNS()
        {
            return await Task.Run(() =>
            {
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_latency.Text = Latency.MeasureLatency(currentDNS.dnsAddress[0]).ToString() + " ms";
                }));
                return true;
            });

        }
        private async void disconnectFromApp()
        {
            Guna2MessageDialog message = new Guna2MessageDialog()
            {
                Buttons = MessageDialogButtons.YesNo,
                Icon = MessageDialogIcon.Warning,
                Style = MessageDialogStyle.Dark,
                Text = "If you leave the program, your DNS will be disabled. Are you sure?",
                Caption = "Exit Program"
            };
            if (message.Show() == DialogResult.No)
                return;

            if (_connected)
            {
                btn_sync.Enabled = false;
                shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                lbl_dnsStatus.Text = "Disconnecting";
                lbl_status.Text = "RESTORING";
                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                await delay(3000);
                DnsManager.clearDNS();
                shapeStatus.FillColor = Color.FromArgb(248, 114, 114);
                lbl_message.Text = "BETA VERSION";
                lbl_dnsStatus.Text = "Disconnected";
                wp_dnsProgress.Visible = false;
                wp_dnsProgress.Stop();
                iconConnect.Image = Properties.Resources.connectIcon;
            }
            lbl_status.Text = "CLOSING THE PROGRAM";
            await delay(2000);
            this.Close();


        }

        private async void btn_sync_Click(object sender, EventArgs e)
        {
            await syncLatency();
            checkInternetConnection();
        }

        private void frm_main_Load(object sender, EventArgs e)
        {
            currentFormLoaded = this;
            changeServer();
            checkInternetConnection();
        }

        private async void connectEvent(object sender, EventArgs e)
        {
            if (!_internetConnection || pendingRequest) return;
            if (!_connected)
            {
                pendingRequest = true;
                btn_sync.Enabled = false;
                shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                wp_dnsProgress.Invoke(new MethodInvoker(delegate
                {
                    wp_dnsProgress.Visible = true;
                    wp_dnsProgress.Start();
                }));
                lbl_dnsStatus.Text = "Connecting";
                lbl_status.Text = "APPLIYNG DNS";
                await delay(3000);
                DnsManager.setDNS(currentDNS.dnsAddress);
                connectedDNS = currentDNS;
                shapeStatus.FillColor = Color.FromArgb(3, 201, 136);
                lbl_dnsStatus.Text = "Connected";
                lbl_status.Text = "DISCONNECT";
                wp_dnsProgress.Invoke(new MethodInvoker(delegate
                {
                    wp_dnsProgress.Visible = false;
                    wp_dnsProgress.Stop();
                }));
                // Sync Latency
                await syncLatency();
                btn_sync.Enabled = true;
                new NotificationForm().showAlert($"{connectedDNS.Name} Connected", NotificationForm.enmType.Success);
                iconConnect.Image = Properties.Resources.connectedIcon;
                pendingRequest = false;
            }
            else
            {
                pendingRequest = true;
                btn_sync.Enabled = false;
                shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                wp_dnsProgress.Invoke(new MethodInvoker(delegate
                {
                    wp_dnsProgress.Visible = true;
                    wp_dnsProgress.Start();
                }));
                lbl_dnsStatus.Text = "Disconnecting";
                lbl_status.Text = "RESTORING";
                await delay(3000);
                DnsManager.clearDNS();
                shapeStatus.FillColor = Color.FromArgb(248, 114, 114);
                lbl_message.Text = "BETA VERSION";
                lbl_dnsStatus.Text = "Disconnected";
                lbl_status.Text = "CLICK TO CONNECT";
                wp_dnsProgress.Invoke(new MethodInvoker(delegate
                {
                    wp_dnsProgress.Visible = false;
                    wp_dnsProgress.Stop();
                }));
                // Sync Latency           
                await syncLatency();
                btn_sync.Enabled = true;
                new NotificationForm().showAlert($"{connectedDNS.Name} Disconnected", NotificationForm.enmType.Error);
                iconConnect.Image = Properties.Resources.connectIcon;
                pendingRequest = false;
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

        private void hiddenHomeForm(bool visible)
        {
            wp_dnsProgress.Visible = !visible;
            pnl_information.Visible = lbl_message.Visible = iconConnect.Visible = lbl_status.Visible = shape_connect.Visible = cb_selectDns.Visible = visible;
        }
        private void clickControlMenu(object sender, EventArgs e)
        {
            if (currentSelectedMenuOption == ((Guna2Button)sender)) return;
            currentSelectedMenuOption.ImageSize = new Size(40, 40);
            currentSelectedMenuOption.FillColor = Color.Transparent;
            currentSelectedMenuOption.Text = string.Empty;
            currentSelectedMenuOption = ((Guna2Button)sender);
            selectMenuOption(sender, true);
            switch (currentSelectedMenuOption.Name)
            {
                case "btn_settings":
                    hiddenHomeForm(false);
                    if (pnl_container.Controls.ContainsKey("frm_explore"))
                        pnl_container.Controls.Remove(currentFormLoaded);
                    currentFormLoaded = FormManager.openChildFormInPanel(new frm_settings(), pnl_container);
                    break;
                case "btn_home":
                    hiddenHomeForm(true);
                    pnl_container.Controls.Remove(currentFormLoaded);
                    break;
                case "btn_explore":
                    hiddenHomeForm(false);
                    if (pnl_container.Controls.ContainsKey("frm_settings"))
                        pnl_container.Controls.Remove(currentFormLoaded);
                    currentFormLoaded = FormManager.openChildFormInPanel(new frm_explore(this), pnl_container);
                    break;
            }
        }
    }
}
