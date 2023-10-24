using ForceConnect.API;
using ForceConnect.Launch;
using ForceConnect.Services;
using ForceConnect.User_Controls;
using ForceConnect.Utility;
using Guna.UI2.WinForms;
using System;
using System.Collections.Generic;
using System.Diagnostics;
using System.Drawing;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;
using Control = System.Windows.Forms.Control;

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

        private long _currentLatency = 0;
        private bool _connected, pendingRequest, _internetConnection = true;
        private readonly Version version = Version.Parse(Application.ProductVersion);
        private readonly string _repositoryOwner = "Mxqius", _repositoryName = "ForceConnect";
        private string _statusConnectionService, _previewAddress;

        private List<ServiceControl> serviceControls = new List<ServiceControl>();

        private Thread serviceThread;

        private sbyte serviceComboBoxSelectedIndex = -1;

        public frm_main()
        {
            InitializeComponent();
            updateVersion();
            currentSelectedMenuOption = btn_home;

            btn_home.ImageSize = new Size(35, 35);
            btn_home.FillColor = Color.FromArgb(44, 45, 52);
            btn_home.Text = "HOME";

            servicesUser = DnsAddressItems.GetServicesUser();

            // Just for test
            //currentDNS = servicesUser[0];
        }

        private void updateServices()
        {
            servicesUser.Clear();
            servicesUser = DnsAddressItems.GetServicesUser();

        }
        private async Task checkInternetConnection()
        {
            await Task.Run(async () =>
            {
                if (await getLatencyDNS("google.com") == -1)
                    changeAppStatus(false);
                else
                    changeAppStatus(true);
            });
        }
        private void changeAppStatus(bool internetConnection)
        {
            _internetConnection = internetConnection;

            this.Invoke(new MethodInvoker(delegate
            {
                if (internetConnection && _connected)
                {
                    lbl_status.Text = "CLICK TO DISCONNECT";
                    iconConnect.Image = Properties.Resources.turn_on;
                    iconConnect.ImageRotate = 180;
                    return;
                }
                else if (internetConnection && !_connected)
                {
                    lbl_status.Text = "CLICK TO CONNECT";
                    iconConnect.Image = Properties.Resources.turn_on;
                    iconConnect.ImageRotate = 0;
                    return;
                }
                else
                {
                    lbl_status.Text = "NO CONNECTION";
                    iconConnect.Image = Properties.Resources.no_wifi;
                    iconConnect.ImageRotate = 0;
                }
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

        private async void welcomeAction()
        {
            lbl_appName_wlc.Text = string.Empty;
            string wlcName = "";
            foreach (char letter in "ForceConnect")
            {
                wlcName += letter;
                lbl_appName_wlc.Text = wlcName + "|";
                await Task.Delay(200);
            }
            lbl_appName_wlc.Text = wlcName;
        }
        private void hideWelcomePanel()
        {
            //transitionEffect.HideSync(pnl_welcome);
            pnl_welcome.Visible = lbl_hintSelectDNS.Visible = false;
            //transitionEffect.ShowSync(pnl_information);
            lbl_version.Visible = pnl_latencySection.Visible = pnl_cardDns.Visible = pnl_addressSection.Visible = true;
        }
        private void cb_selectDNS_SelectedIndexChanged(object sender, EventArgs e)
        {
            hideWelcomePanel();
            if (connectedDNS != null)
            {
                if (connectedDNS.Name.ToLower() != cb_selectDNS.Text.ToLower() && _connected)
                    new frm_messageBox()
                    {
                        MessageCaption = "Warning",
                        MessageText = $"You Are still connect to perviosly dns ({connectedDNS.Name})",
                        MessageIcon = frm_messageBox.Icon.Warning,
                        MessageButtons = frm_messageBox.Buttons.OK
                    }.ShowMessage();
                if (connectedDNS.Name.ToLower() == cb_selectDNS.Text.ToLower())
                    updateVersion();
            }
            changeServer();
            serviceComboBoxSelectedIndex = (sbyte)cb_selectDNS.SelectedIndex;
        }

        private void changeServer()
        {
            if (servicesUser.Exists(x => x.Name == cb_selectDNS.Text))
            {
                currentDNS = servicesUser.Find(item => item.Name == cb_selectDNS.Text);
                showInformation();
            }
        }
        private async void showInformation()
        {
            lbl_previewAddress.Text = currentDNS.Name;
            if (currentDNS.dnsAddress.Length > 1)
            {
                _previewAddress = currentDNS.dnsAddress[0] + " - " + currentDNS.dnsAddress[1];
                lbl_previewAddress.Text = _previewAddress;
            }
            else
            {
                _previewAddress = currentDNS.dnsAddress[0];
                lbl_previewAddress.Text = _previewAddress;
            }
            pb_dnsPicture.Image = currentDNS.Picture;
            if (_internetConnection)
                await syncLatencyDNS();
            else
                lbl_latency.Text = "Latnecy: " + "-1 ms";
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

        private async void lbl_serviceAddress_Click(object sender, EventArgs e)
        {
            Clipboard.SetText(lbl_previewAddress.Text);
            lbl_previewAddress.Text = "Address copied";
            await delay(1000);
            lbl_previewAddress.Text = currentDNS.dnsAddress[0] + " - " + currentDNS.dnsAddress[1];
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
                    lbl_latency.Text = "Latnecy: " + _currentLatency.ToString() + " ms";
                }));
                updateLatencyStatusColor();
            });

        }
        private async Task syncLatencyDNS()
        {
            await Task.Run(async () =>
            {
                _currentLatency = await getLatencyDNS(currentDNS.dnsAddress[0]);
                this.Invoke(new MethodInvoker(delegate
                {
                    lbl_latency.Text = "Latnecy: " + _currentLatency.ToString() + " ms";
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
                //statusProgressColor.FillColor = Color.FromArgb(255, 176, 0);
                _statusConnectionService = "Disconnecting";
                lbl_status.Text = "RESTORING";
                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                await delay(3000);
                DnsManager.clearDNS();
                //statusProgressColor.FillColor = Color.FromArgb(216, 0, 50);
                updateVersion();
                _statusConnectionService = "Disconnected";
                wp_dnsProgress.Visible = false;
                wp_dnsProgress.Stop();
                iconConnect.Image = Properties.Resources.turn_on;
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
            return lbl_version.Text = "VERSION " + version.Major + "." + version.Minor;
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
        private async void frm_main_Load(object sender, EventArgs e)
        {
            loadServices();
            await syncLatency();
            welcomeAction();
            registrySync();
            tsm_exit.Click += Tsm_exit_Click;
            currentFormLoaded = this;
            //changeServer();
            checkAutoUpdate();
            // Update Notify Text
            notifyForm.Text = $"Version {version.Major}.{version.Minor}.{version.Build}";
            //  Initialize Discord RPC
            DiscordRPCManager.GetInstance();
        }
        private void updateLatencyStatusColor()
        {

            this.Invoke(new MethodInvoker(delegate
            {
                if (_currentLatency > 1 && _currentLatency <= 120)
                    //shapeStatus.FillColor = Color.FromArgb(60, 207, 78);
                    pictureLatency.Image = Properties.Resources.signalGreen;
                else if (_currentLatency > 120 && _currentLatency <= 180)
                    //shapeStatus.FillColor = Color.FromArgb(255, 176, 0);
                    pictureLatency.Image = Properties.Resources.signalYellow;
                else
                    //shapeStatus.FillColor = Color.FromArgb(216, 0, 50);
                    pictureLatency.Image = Properties.Resources.signalRed;
            }));
        }
        private async void loadServices()
        {
            await Task.Run(() =>
            {
                foreach (DnsAddress dns in servicesUser)
                {
                    this.Invoke(new MethodInvoker(delegate
                    {
                        cb_selectDNS.Items.Add(dns.Name);
                    }));
                }
            });
        }
        private void PerformTaskConnection(byte step, Color progressColor)
        {
            if (step >= 4)
            {
                step = 4;
                progressBarConnection.Animated = false;
            }
            else progressBarConnection.Animated = true;

            if (step == 0)
                // Rest the progress bar
                progressBarConnection.Value = 0;

            // Update the progress bar.            
            progressBarConnection.Value = step * 25;
            progressBarConnection.ProgressColor = progressColor;
            progressBarConnection.ProgressColor2 = progressColor;
        }
        private async void connectEvent(object sender, EventArgs e)
        {
            if (pendingRequest) return;
            await checkInternetConnection();
            await syncLatency();
            if (!_internetConnection) return;
            DnsAddress connectingDNS = currentDNS;
            if (!_connected)
            {
                if (currentDNS == null)
                {
                    new frm_messageBox()
                    {
                        MessageText = "Please first pick your favorite service to connect !",
                        MessageCaption = "ERROR",
                        MessageButtons = frm_messageBox.Buttons.OK,
                        MessageIcon = frm_messageBox.Icon.Error
                    }.ShowMessage(); return;
                }
                Action onCompleted = () =>
                {
                    this.Invoke(new MethodInvoker(async delegate
                    {
                        connectedDNS = connectingDNS;

                        //statusProgressColor.FillColor = Color.FromArgb(60, 207, 78);

                        wp_dnsProgress.Visible = false;
                        wp_dnsProgress.Stop();

                        _statusConnectionService = "Connected";
                        tsm_status.Text = "Connected";
                        lbl_status.Text = "CLICK TO DISCONNECT";

                        new NotificationForm().showAlert($"{connectedDNS.Name} Connected", NotificationForm.enmType.Success);

                        iconConnect.ImageRotate = 180;

                        PerformTaskConnection(4, Color.FromArgb(60, 207, 78));

                        // Sync Latency
                        await syncLatency();

                        // Update Discord RPC
                        DiscordRPCManager.GetInstance().UpdatePresence(details: $"Connected to {connectedDNS.Name}", state: "", largeImage: "force", largeImageText: "Powerful DnsChanger", smallImage: $"{connectedDNS.Name.ToLower()}", smallImageText: $"{connectedDNS.Name} Service:\n {_previewAddress}");
                        pendingRequest = false;
                        cb_selectDNS.Enabled = true;
                    }));
                };
                // Start Connecting 
                pendingRequest = true;
                cb_selectDNS.Enabled = false;


                //statusProgressColor.FillColor = Color.FromArgb(255, 176, 0);

                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                _statusConnectionService = "Connecting";
                lbl_status.Text = "APPLIYNG DNS";

                PerformTaskConnection(1, Color.FromArgb(255, 176, 0));

                //await delay(3000);
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
                        //statusProgressColor.FillColor = Color.FromArgb(216, 0, 50);

                        updateVersion();

                        wp_dnsProgress.Visible = false;
                        wp_dnsProgress.Stop();

                        _statusConnectionService = "Disconnected";
                        tsm_status.Text = "Disconnected";
                        lbl_status.Text = "CLICK TO CONNECT";

                        new NotificationForm().showAlert($"{connectedDNS.Name} Disconnected", NotificationForm.enmType.Error);

                        iconConnect.ImageRotate = 0;
                        // Sync Latency           
                        await syncLatency();

                        //Update Discord RPC
                        DiscordRPCManager.GetInstance().UpdatePresence(details: $"Ideal", state: "", largeImage: "force", largeImageText: "Powerful DnsChanger");

                        pendingRequest = false;

                        PerformTaskConnection(0, Color.FromArgb(200, 213, 218, 223));
                    }));
                };
                // Start Disconnecting
                pendingRequest = true;

                //statusProgressColor.FillColor = Color.FromArgb(255, 176, 0);

                wp_dnsProgress.Visible = true;
                wp_dnsProgress.Start();
                _statusConnectionService = "Disconnecting";
                lbl_status.Text = "RESTORING";

                PerformTaskConnection(1, Color.FromArgb(255, 176, 0));

                //await delay(3000);
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

        //private void btn_nextServices_Click(object sender, EventArgs e)
        //{
        //    if (serviceControls.Count > 3)
        //    {
        //        List<ServiceControl> itemsToMove = serviceControls.GetRange(0, 3); // Get the three items starting from index 2.

        //        // Remove the three items from their current positions.
        //        serviceControls.RemoveRange(0, 3);

        //        // Add the three items to the end of the list.
        //        serviceControls.AddRange(itemsToMove);

        //        pnl_information.Controls.Clear();
        //        pnl_information.Controls.AddRange(new[] { serviceControls[0], serviceControls[1], serviceControls[2] });
        //    }
        //}

        private async void timerLatency_Tick(object sender, EventArgs e)
        {
            await syncLatency();
            await checkInternetConnection();
        }

        private void ToolFooterEnter(object sender, EventArgs e)
        {
            Guna2Button button = ((Guna2Button)sender);
            switch (button.Name)
            {
                case "btn_flushDNS":
                    button.Text = "Flush DNS";

                    break;
            }
            button.Size = new Size(160, 36);
        }

        private void ToolFooterLeave(object sender, EventArgs e)
        {
            Guna2Button button = ((Guna2Button)sender);
            button.Size = new Size(69, 36);
            button.Text = string.Empty;
        }

        private void tsm_donateUs_Click(object sender, EventArgs e)
        {
            Process.Start("https://www.coffeebede.com/mxqius");
        }

        private async void btn_flushDNS_Click(object sender, EventArgs e)
        {
            if (pendingRequest) return;
            await checkInternetConnection();
            await syncLatency();
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
                    wp_dnsProgress.Visible = true;
                    wp_dnsProgress.Start();
                    lbl_status.Text = "FLUSHING DNS SYSTEM";
                    await delay(2000);
                    // Sync Latency
                    await syncLatency();
                    wp_dnsProgress.Visible = false;
                    wp_dnsProgress.Stop();
                    lbl_status.Text = "SUCCESSFULLY FLUSHED";
                    await delay(1500);
                    if (_connected)
                        lbl_status.Text = "CLICK TO DISCONNECT";
                    else
                        lbl_status.Text = "CLICK TO CONNECT";

                    pendingRequest = false;
                }));
            };
            pendingRequest = true;
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
            ((Guna2Button)sender).ImageSize = new Size(35, 35);
            ((Guna2Button)sender).FillColor = Color.FromArgb(44, 45, 52);
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
            ((Guna2Button)sender).ImageSize = new Size(30, 30);
            ((Guna2Button)sender).FillColor = Color.Transparent;
            ((Guna2Button)sender).Text = string.Empty;
        }

        private void hiddenHomeForm(bool visible)
        {
            wp_dnsProgress.Visible = !visible;
            if (!pnl_welcome.Visible)
            {
                pnl_latencySection.Visible = pnl_cardDns.Visible = pnl_addressSection.Visible = lbl_hintSelectDNS.Visible = lbl_version.Visible = iconConnect.Visible = lbl_status.Visible = shape_connect.Visible = btn_flushDNS.Visible = visible;
            }
        }
        private void clickControlMenu(object sender, EventArgs e)
        {
            if (currentSelectedMenuOption == ((Guna2Button)sender)) return;
            currentSelectedMenuOption.ImageSize = new Size(35, 35);
            currentSelectedMenuOption.FillColor = Color.Transparent;
            currentSelectedMenuOption.Text = string.Empty;
            currentSelectedMenuOption = ((Guna2Button)sender);
            selectMenuOption(sender, true);
            if (!_connected)
            {
                pnl_welcome.Visible = lbl_hintSelectDNS.Visible = true;
                currentDNS = null;
            }
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
                    hiddenHomeForm(true);
                    cb_selectDNS.Items.Clear();
                    loadServices();
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
