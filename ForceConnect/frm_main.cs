using ForceConnect.API;
using ForceConnect.Launch;
using ForceConnect.Services;
using ForceConnect.Utility;
using Guna.UI2.WinForms;
using System;
using System.Collections.Generic;
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

        public DnsAddress currentDNS, connectedDNS;
        private List<DnsAddress> servicesUser;
        private Guna2Button currentSelectedMenuOption;
        public Form currentFormLoaded;

        private byte currentSelectedIndexComboBox = 0;
        private long _currentLatency = 0;
        private bool _connected, pendingRequest, _internetConnection = true;
        private readonly Version version = Version.Parse(Application.ProductVersion);
        private readonly string _repositoryOwner = "Mxqius", _repositoryName = "ForceConnect";

        private Thread serviceThread;

        public frm_main()
        {
            InitializeComponent();
            updateVersion();
            currentSelectedMenuOption = btn_home;

            btn_home.ImageSize = new Size(45, 45);
            btn_home.FillColor = Color.FromArgb(32, 32, 32);
            btn_home.Text = "HOME";

            servicesUser = DnsAddressItems.GetServicesUser();
            currentDNS = servicesUser[0];

        }
        private void updateServices()
        {
            servicesUser.Clear();
            servicesUser = DnsAddressItems.GetServicesUser();

        }
        private void updateDNSBox()
        {
            cb_selectDns.Items.Clear();
            foreach (DnsAddress dns in servicesUser)
            {
                cb_selectDns.Items.Add(dns.Name);
            }
        }
        private async Task checkInternetConnection()
        {
            await Task.Run(async () =>
            {
                if (await getLatencyDNS("google.com") == -1)
                    changeAppStatus(false);
                else
                    changeAppStatus(true);

                updateLatencyPicture();
            });
        }
        private void changeAppStatus(bool internetConnection)
        {
            _internetConnection = internetConnection;
            if (_internetConnection && _connected) return;

            this.Invoke(new MethodInvoker(delegate
            {
                lbl_status.Text = (internetConnection) ? "CLICK TO CONNECT" : "NO CONNECTION";
                iconConnect.Image = (internetConnection) ? Properties.Resources.connectIcon : Properties.Resources.no_internet;
            }));

        }
        public async Task<bool> delay(int milisecound)
        {
            return await Task.Run(() =>
            {
                Thread.Sleep(milisecound);
                return true;
            });
        }
        private void btn_exit_Click(object sender, EventArgs e)
        {
            if (!IsMinimizeTrayCheck())
            { disconnectFromApp(); return; }
            Hide();
            notifyForm.Visible = true;
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
            currentSelectedIndexComboBox = (byte)cb_selectDns.SelectedIndex;
            if (connectedDNS != null)
            {
                if (connectedDNS.Name.ToLower() != cb_selectDns.Text.ToLower() && _connected)
                    lbl_message.Text = $"YOU ARE STILL CONNECT TO PERVIOSLY DNS ({connectedDNS.Name})";
                if (connectedDNS.Name.ToLower() == cb_selectDns.Text.ToLower())
                    updateVersion();
            }
            changeServer();
        }

        private void changeServer()
        {
            if (servicesUser.Exists(x => x.Name == cb_selectDns.Text))
                currentDNS = servicesUser.Find(item => item.Name == cb_selectDns.Text);

            showInformation();
        }
        private async void showInformation()
        {
            pb_dnsPicture.Image = currentDNS.Picture;
            lbl_name.Text = currentDNS.Name;
            if (currentDNS.dnsAddress.Length > 1)
                lbl_previewAddress.Text = currentDNS.dnsAddress[0] + " " + currentDNS.dnsAddress[1];
            else
                lbl_previewAddress.Text = currentDNS.dnsAddress[0];

            if (_internetConnection)
                await syncLatencyDNS();
            else
                lbl_latency.Text = "-1 ms";
            await checkInternetConnection();
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
        private async Task<long> getLatencyDNS(string address)
        {
            return await Task.Run(() =>
            {
                return Latency.MeasureLatency(address);
            });
        }
        private async Task syncLatency()
        {

            await Task.Run(async () =>
            {
                _currentLatency = await getLatencyDNS("google.com");
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_latency.Text = _currentLatency.ToString() + " ms";
                }));
            });

        }
        private async Task syncLatencyDNS()
        {
            await Task.Run(async () =>
            {
                _currentLatency = await getLatencyDNS(currentDNS.dnsAddress[0]);
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_latency.Text = _currentLatency.ToString() + " ms";
                }));
            });
        }
        private void registrySync()
        {
            string isAutoUpdate = RegistryApplication.RetrieveData("AutoUpdate");
            string isMinimizeTray = RegistryApplication.RetrieveData("MinimizeTray");
            if (isMinimizeTray == null)
                RegistryApplication.SaveData("MinimizeTray", "false");
            if (isAutoUpdate == null)
                RegistryApplication.SaveData("AutoUpdate", "false");
        }
        private async void disconnectFromApp()
        {
            if (pendingRequest)
            {
                new frm_messageBox()
                {
                    MessageText = "It is not possible to leave the program when an operation is in progress!",
                    MessageCaption = "Cannot Exit Program",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Error
                }.ShowMessage(); return;
            }
            pendingRequest = true;
            frm_messageBox message = new frm_messageBox()
            {
                MessageText = "If you leave the program, your DNS will be disabled. Are you sure?",
                MessageCaption = "Exit Program",
                MessageButtons = frm_messageBox.Buttons.YesNo,
                MessageIcon = frm_messageBox.Icon.Warning
            };

            if (message.ShowMessage() == DialogResult.No)
            { pendingRequest = false; return; }

            if (!Visible)
                Show();
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
                updateVersion();
                lbl_dnsStatus.Text = "Disconnected";
                wp_dnsProgress.Visible = false;
                wp_dnsProgress.Stop();
                iconConnect.Image = Properties.Resources.connectIcon;
            }
            wp_dnsProgress.Visible = true;
            wp_dnsProgress.Start();
            lbl_status.Text = "CLOSING THE PROGRAM";
            await delay(2000);
            this.Close();


        }

        private async void btn_sync_Click(object sender, EventArgs e)
        {
            if (pendingRequest) return;
            pendingRequest = true;
            await syncLatency();
            await checkInternetConnection();
            pendingRequest = false;
        }

        private string updateVersion()
        {
            return lbl_message.Text = "VERSION " + version.Major + "." + version.Minor;
        }
        private async Task<string> getLastestVersionApplication()
        {
            return await Task.Run(() =>
            {
                return LaunchUpdate.GetLatestVersionFromGitHub("Mxqius", "ForceConnect"); ;
            });
        }
        private async void checkAutoUpdate()
        {
            if (await getLatencyDNS("google.com") == -1) return;
            string isAutoUpdate = RegistryApplication.RetrieveData("AutoUpdate");
            if (isAutoUpdate == "false" || isAutoUpdate == null) return;
            string lastestVersion = await getLastestVersionApplication();
            bool updateAvailable = LaunchUpdate.IsUpdateAvailable(lastestVersion, LaunchUpdate.getVersionApplication().ToString());
            if (updateAvailable)
            {
                //timer_updateCheck.Enabled = true;
                DialogResult result = new frm_messageBox()
                {
                    MessageText = $"Update is available, Would you like to download and install it? v{lastestVersion}",
                    MessageCaption = "Update Check",
                    MessageButtons = frm_messageBox.Buttons.YesNo,
                    MessageIcon = frm_messageBox.Icon.Info
                }.ShowMessage();
                if (result == DialogResult.Yes)
                {
                    pendingRequest = true;
                    wp_dnsProgress.Visible = true;
                    wp_dnsProgress.Start();
                    lbl_status.Text = "DOWNLOADING UPDATE";
                    string download_url = await getLastestVersionDownlaodUrl();
                    string savePath = Application.StartupPath + "\\ForceConnect_Update.exe";
                    await DownloadUpdateTask(download_url, savePath);
                    wp_dnsProgress.Visible = false;
                    wp_dnsProgress.Stop();
                    lbl_status.Text = "UPDATING COMPLETED";
                    await delay(1000);
                    lbl_status.Text = (_connected) ? "CLICK TO DISCONNECT" : "CLICK TO CONNECT";
                    pendingRequest = false;
                }
            }
        }
        private async Task DownloadUpdateTask(string download_url, string savePath)
        {
            await Task.Run(() =>
           {
               LaunchUpdate.DownloadUpdate(download_url, savePath);
           });
        }

        private async Task<string> getLastestVersionDownlaodUrl()
        {
            return await Task.Run(() =>
            {
                return LaunchUpdate.GetLastestVersionDownloadUrl(_repositoryOwner, _repositoryName);
            });
        }
        private void frm_main_Load(object sender, EventArgs e)
        {
            registrySync();
            tsm_exit.Click += Tsm_exit_Click;
            updateDNSBox();
            cb_selectDns.SelectedIndexChanged -= cb_selectDns_SelectedIndexChanged;
            cb_selectDns.SelectedIndex = 0;
            cb_selectDns.SelectedIndexChanged += cb_selectDns_SelectedIndexChanged;
            currentFormLoaded = this;
            changeServer();
            checkAutoUpdate();
            // Update Notify Text
            notifyForm.Text = $"Version {version.Major}.{version.Minor}.{version.Build}";
            //  Initialize Discord RPC
            DiscordRPCManager.GetInstance();
        }
        private void updateLatencyPicture()
        {

            this.Invoke(new MethodInvoker(delegate
            {
                if (_currentLatency > 1 && _currentLatency <= 120)
                    pb_latencyPicture.Image = Properties.Resources.signalGreen;
                else if (_currentLatency > 120 && _currentLatency <= 180)
                    pb_latencyPicture.Image = Properties.Resources.signalYellow;
                else
                    pb_latencyPicture.Image = Properties.Resources.signalRed;
            }));
        }
        private async void connectEvent(object sender, EventArgs e)
        {
            if (pendingRequest) return;
            await checkInternetConnection();
            await syncLatency();
            updateLatencyPicture();
            if (!_internetConnection) return;
            DnsAddress connectingDNS = currentDNS;
            if (!_connected)
            {
                Action onCompleted = () =>
                {
                    this.Invoke(new MethodInvoker(async delegate
                    {
                        connectedDNS = connectingDNS;

                        shapeStatus.FillColor = Color.FromArgb(27, 94, 32);
                        pnl_statusColor.FillColor = Color.FromArgb(42, 61, 43);
                        lbl_dnsStatus.ForeColor = Color.FromArgb(63, 160, 79);
                        lbl_dnsStatus.Text = "Connected";
                        tsm_status.Text = "Connected";
                        lbl_status.Text = "CLICK TO DISCONNECT";
                        wp_dnsProgress.Visible = false;
                        wp_dnsProgress.Stop();

                        new NotificationForm().showAlert($"{connectedDNS.Name} Connected", NotificationForm.enmType.Success);

                        iconConnect.Image = Properties.Resources.connectedIcon;
                        // Sync Latency
                        await syncLatency();
                        updateLatencyPicture();

                        // Update Discord RPC
                        DiscordRPCManager.GetInstance().UpdatePresence(details: $"Connected to {connectedDNS.Name}", state: "", largeImage: "force", largeImageText: "Powerful DnsChanger", smallImage: $"{connectedDNS.Name.ToLower()}", smallImageText: $"{connectedDNS.Name} Service:\n{lbl_previewAddress.Text}");

                        btn_sync.Enabled = true;
                        cb_selectDns.Enabled = true;
                        pendingRequest = false;
                    }));
                };
                // Start Connecting 
                pendingRequest = true;
                cb_selectDns.Enabled = false;
                btn_sync.Enabled = false;
                //shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                //pnl_statusColor.FillColor = Color.FromArgb(234, 144, 108);
                //lbl_dnsStatus.ForeColor = Color.FromArgb(244, 209, 96);
                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                lbl_dnsStatus.Text = "Connecting";
                lbl_status.Text = "APPLIYNG DNS";
                await delay(2000);
                // Connect Thread 
                serviceThread = new Thread(() =>
                {
                    try
                    { DnsManager.setDNS(connectingDNS.dnsAddress); }
                    finally
                    { onCompleted(); }
                })
                { IsBackground = true };
                serviceThread.Start();
            }
            else
            {
                Action onCompleted = () =>
                {
                    this.Invoke(new MethodInvoker(async delegate
                    {
                        shapeStatus.FillColor = Color.FromArgb(183, 28, 28);
                        pnl_statusColor.FillColor = Color.FromArgb(79, 43, 41);
                        lbl_dnsStatus.ForeColor = Color.FromArgb(180, 61, 61);
                        updateVersion();
                        lbl_dnsStatus.Text = "Disconnected";
                        tsm_status.Text = "Disconnected";
                        lbl_status.Text = "CLICK TO CONNECT";
                        wp_dnsProgress.Visible = false;
                        wp_dnsProgress.Stop();
                        new NotificationForm().showAlert($"{connectedDNS.Name} Disconnected", NotificationForm.enmType.Error);
                        iconConnect.Image = Properties.Resources.connectIcon;
                        // Sync Latency           
                        await syncLatency();
                        updateLatencyPicture();

                        //Update Discord RPC
                        DiscordRPCManager.GetInstance().UpdatePresence(details: $"Ideal", state: "", largeImage: "force", largeImageText: "Powerful DnsChanger");

                        btn_sync.Enabled = true;
                        pendingRequest = false;
                    }));
                };
                // Start Disconnecting
                pendingRequest = true;
                btn_sync.Enabled = false;
                //shapeStatus.FillColor = Color.FromArgb(255, 221, 131);
                //pnl_statusColor.FillColor = Color.FromArgb(234, 144, 108);
                //lbl_dnsStatus.ForeColor = Color.FromArgb(244, 209, 96);
                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                lbl_dnsStatus.Text = "Disconnecting";
                lbl_status.Text = "RESTORING";
                await delay(2000);
                // Disconnect Thread
                serviceThread = new Thread(() =>
                {
                    try { DnsManager.clearDNS(); }
                    finally { onCompleted(); }
                })
                { IsBackground = true };
                serviceThread.Start();
            }
            _connected = !_connected;
        }

        private bool IsMinimizeTrayCheck()
        {
            string isMinimizeTray = RegistryApplication.RetrieveData("MinimizeTray");
            return (isMinimizeTray == "true" && isMinimizeTray != null) ? true : false;
        }

        private void frm_main_Resize(object sender, EventArgs e)
        {
            if (this.WindowState == FormWindowState.Minimized && IsMinimizeTrayCheck())
            {
                Hide();
                notifyForm.Visible = true;
            }
        }

        private void Tsm_exit_Click(object sender, EventArgs e)
        {
            disconnectFromApp();
        }

        private void notifyForm_DoubleClick(object sender, EventArgs e)
        {
            Show();
            this.WindowState = FormWindowState.Normal;
            notifyForm.Visible = false;
        }

        private async void btn_flushDNS_Click(object sender, EventArgs e)
        {
            if (pendingRequest) return;
            await checkInternetConnection();
            await syncLatency();
            updateLatencyPicture();
            if (!_internetConnection) return;
            if (_connected)
            {
                new frm_messageBox()
                {
                    MessageText = "This cannot be done when you are connected to a DNS service. Please try again",
                    MessageCaption = "Error",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Error
                }.ShowMessage(); return;
            };
            DialogResult result = new frm_messageBox()
            {
                MessageText = "By doing this, all the DNS cache of your system will be cleared. Are you sure?",
                MessageCaption = "Warning",
                MessageButtons = frm_messageBox.Buttons.YesNo,
                MessageIcon = frm_messageBox.Icon.Warning
            }.ShowMessage();
            if (result == DialogResult.No) return;
            Action onCompleted = () =>
            {
                this.Invoke(new MethodInvoker(async delegate
                {
                    btn_sync.Enabled = false;
                    wp_dnsProgress.Visible = true;
                    wp_dnsProgress.Start();
                    lbl_status.Text = "FLUSHING DNS SYSTEM";
                    await delay(2000);
                    // Sync Latency
                    await syncLatency();
                    updateLatencyPicture();
                    wp_dnsProgress.Visible = false;
                    wp_dnsProgress.Stop();
                    lbl_status.Text = "SUCCESSFULLY FLUSHED";
                    await delay(1500);
                    if (_connected)
                        lbl_status.Text = "CLICK TO DISCONNECT";
                    else
                        lbl_status.Text = "CLICK TO CONNECT";
                    cb_selectDns.Enabled = true;
                    pendingRequest = false;
                }));
            };
            pendingRequest = true;
            cb_selectDns.Enabled = false;
            // Flush DNS Thread
            serviceThread = new Thread(() =>
            {
                try { DnsManager.flushDNS(); }
                finally { onCompleted(); }
            })
            { IsBackground = true };
            serviceThread.Start();
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
                case "btn_about":
                    ((Guna2Button)sender).Text = "ABOUT";
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
            pnl_information.Visible = lbl_message.Visible = iconConnect.Visible = lbl_status.Visible = shape_connect.Visible = btn_flushDNS.Visible = cb_selectDns.Visible = visible;
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
                    if (pnl_container.Controls.ContainsKey("frm_explore") || pnl_container.Controls.ContainsKey("frm_about"))
                        pnl_container.Controls.Remove(currentFormLoaded);
                    currentFormLoaded = FormManager.openChildFormInPanel(new frm_settings(), pnl_container);
                    break;
                case "btn_home":
                    updateServices();
                    updateDNSBox();
                    cb_selectDns.SelectedIndex = currentSelectedIndexComboBox;
                    hiddenHomeForm(true);
                    pnl_container.Controls.Remove(currentFormLoaded);
                    break;
                case "btn_explore":
                    hiddenHomeForm(false);
                    if (pnl_container.Controls.ContainsKey("frm_settings") || pnl_container.Controls.ContainsKey("frm_about"))
                        pnl_container.Controls.Remove(currentFormLoaded);
                    currentFormLoaded = FormManager.openChildFormInPanel(new frm_explore(this), pnl_container);
                    break;
                case "btn_about":
                    hiddenHomeForm(false);
                    if (pnl_container.Controls.ContainsKey("frm_settings") || pnl_container.Controls.ContainsKey("frm_explore"))
                        pnl_container.Controls.Remove(currentFormLoaded);
                    currentFormLoaded = FormManager.openChildFormInPanel(new frm_about(), pnl_container);
                    break;
            }
        }
    }
}
