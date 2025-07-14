using ForceConnect.API;
using ForceConnect.Launch;
using ForceConnect.Utility;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Reflection;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_settings : Form
    {
        private bool _launchOnWindows, initilizeWait, checkingUpdate;
        private string _isAutoUpdate, _isMinimizeTray;
        private readonly string _repositoryOwner = "Mxqius", _repositoryName = "ForceConnect";
        private readonly string _discordOptionText;
        private static frm_settings _instance;
        private frm_main frm_mainObject;
        public frm_settings(frm_main formMain)
        {
            InitializeComponent();
            _launchOnWindows = LaunchProgram.IsAppSetToRunAtStartup();
            _isAutoUpdate = RegistryApplication.RetrieveData("AutoUpdate");
            _isMinimizeTray = RegistryApplication.RetrieveData("MinimizeTray");
            _discordOptionText = lbl_discordDescription.Text;
            frm_mainObject = formMain;
        }
        public static frm_settings GetInstance()
        {
            if (_instance == null)
                _instance = new frm_settings(null);
            return _instance;
        }

        private void cb_launchOnWindows_CheckedChanged(object sender, EventArgs e)
        {
            if (initilizeWait) return;
            if (!_launchOnWindows)
                LaunchProgram.AddAppToStartup();
            else
                LaunchProgram.RemoveAppFromStartup();
            _launchOnWindows = !_launchOnWindows;
        }

        private async Task<string> getLastestVersionApplication()
        {
            return await Task.Run(() =>
            {
                return LaunchUpdate.GetLatestVersionFromGitHub(_repositoryOwner, _repositoryName); ;
            });
        }
        private async Task<string> getLastestVersionDownlaodUrl()
        {
            return await Task.Run(() =>
            {
                return LaunchUpdate.GetLastestVersionDownloadUrl(_repositoryOwner, _repositoryName);
            });
        }
        public async Task<bool> delay(int milisecound)
        {
            return await Task.Run(() =>
            {
                Thread.Sleep(milisecound);
                return true;
            });
        }

        private void cb_autoUpdate_CheckedChanged(object sender, EventArgs e)
        {
            if (initilizeWait) return;
            string isAutoUpdate = RegistryApplication.RetrieveData("AutoUpdate");
            if (isAutoUpdate == null || isAutoUpdate == "false")
                RegistryApplication.SaveData("AutoUpdate", "true");
            else
                RegistryApplication.SaveData("AutoUpdate", "false");
        }

        private void lbl_automaticUpdate_Click(object sender, EventArgs e)
        {
            cb_autoUpdate.Checked = !cb_autoUpdate.Checked;
        }

        private void lbl_minimizeInTray_Click(object sender, EventArgs e)
        {
            cb_minimizeInTray.Checked = !cb_minimizeInTray.Checked;
        }

        private void btn_networkInformation_Click(object sender, EventArgs e)
        {
            new frm_network().ShowDialog();
        }

        private async void ts_discordRPC_CheckedChanged(object sender, EventArgs e)
        {
            frm_mainObject.discordRPCStatus = !frm_mainObject.discordRPCStatus;
            if (!frm_mainObject.discordRPCStatus)
                DiscordRPCManager.GetInstance().Dispose();
            else
                DiscordRPCManager.GetInstance();
            lbl_discordDescription.Text = _discordOptionText + ", Wait for changes..";
            await Task.Delay(5000);
            lbl_discordDescription.Text = _discordOptionText;
        }

        private void lbl_theme_Click(object sender, EventArgs e)
        {
            ts_discordRPC.Checked = !ts_discordRPC.Checked;
        }

        private void btn_languagePanel_Click(object sender, EventArgs e)
        {
            new frm_language().ShowDialog();
        }

        private void cb_minimizeInTray_CheckedChanged(object sender, EventArgs e)
        {
            if (initilizeWait) return;
            string isMinimizeTray = RegistryApplication.RetrieveData("MinimizeTray");
            if (isMinimizeTray == null || isMinimizeTray == "false")
                RegistryApplication.SaveData("MinimizeTray", "true");
            else
                RegistryApplication.SaveData("MinimizeTray", "false");
        }

        private void lbl_launchOnWindows_Click(object sender, EventArgs e)
        {
            cb_launchOnWindows.Checked = !cb_launchOnWindows.Checked;
        }
        private async Task<long> getLatencyDNS(string address)
        {
            return await Task.Run(() =>
            {
                return Latency.MeasureLatency(address);
            });
        }
        private async void btn_updateSofware_Click(object sender, EventArgs e)
        {
            if (await getLatencyDNS("google.com") == -1)
            {
                new frm_messageBox()
                {
                    MessageText = $"Please ensure that your device is connected to the internet and try again.",
                    MessageCaption = "Network Error",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Error
                }.ShowMessage();
                return;
            }
            if (checkingUpdate) return;
            checkingUpdate = true;
            btn_updateSofware.Text = "Checking Updates..";
            await delay(1500);
            string lastestVersion = await getLastestVersionApplication();
            checkingUpdate = false;
            btn_updateSofware.Text = "Check Updates";
            bool updateAvailable = LaunchUpdate.IsUpdateAvailable(lastestVersion, LaunchUpdate.getVersionApplication().ToString());
            if (updateAvailable)
            {
                DialogResult result = new frm_messageBox()
                {
                    MessageText = $"Update is available, Would you like to download and install it? v{lastestVersion}",
                    MessageCaption = "Update Result",
                    MessageButtons = frm_messageBox.Buttons.YesNo,
                    MessageIcon = frm_messageBox.Icon.Info
                }.ShowMessage();
                if (result == DialogResult.Yes)
                {
                    btn_updateSofware.Text = "Downloading..";
                    string download_url = await getLastestVersionDownlaodUrl();
                    string savePath = Application.StartupPath + "\\ForceConnect_Update.exe";
                    LaunchUpdate.DownloadUpdate(download_url, savePath);
                    btn_updateSofware.Text = "Completed";
                    await delay(1000);
                    btn_updateSofware.Text = "Check Updates";
                }
            }
            else
            {
                new frm_messageBox()
                {
                    MessageText = $"You are updated to the latest version of the program. Enjoy",
                    MessageCaption = "Update Result",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Success
                }.ShowMessage();
            }
        }

        private void frm_setttings_Load(object sender, EventArgs e)
        {
            initilizeWait = true;
            cb_launchOnWindows.Checked = _launchOnWindows;
            cb_autoUpdate.Checked = (_isAutoUpdate == "true" && _isAutoUpdate != null) ? true : false;
            cb_minimizeInTray.Checked = (_isMinimizeTray == "true" && _isMinimizeTray != null) ? true : false;
            initilizeWait = false;

        }
    }
}
