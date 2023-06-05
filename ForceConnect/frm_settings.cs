using ForceConnect.Launch;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_settings : Form
    {
        private bool _launchOnWindows, initilizeWait, checkingUpdate;
        private string _isAutoUpdate;
        public frm_settings()
        {
            InitializeComponent();
            _launchOnWindows = LaunchProgram.IsAppSetToRunAtStartup();
            _isAutoUpdate = RegistryApplication.RetrieveData("AutoUpdate");
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
                return LaunchUpdate.GetLatestVersionFromGitHub("Mxqius", "ForceConnect"); ;
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

        private async void btn_updateSofware_Click(object sender, EventArgs e)
        {
            if (checkingUpdate) return;
            checkingUpdate = true;
            btn_updateSofware.Text = "Checking Update..";
            await delay(1500);
            string lastestVersion = await getLastestVersionApplication();
            checkingUpdate = false;
            btn_updateSofware.Text = "Check Update";
            bool updateAvailable = LaunchUpdate.IsUpdateAvailable(lastestVersion, LaunchUpdate.getVersionApplication().ToString());
            if (updateAvailable)
            {
                new frm_messageBox()
                {
                    MessageText = $"Update is available v{lastestVersion}",
                    MessageCaption = "Update Log",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Success
                }.ShowMessage();
            }
            else
            {
                new frm_messageBox()
                {
                    MessageText = $"Update is not available v{lastestVersion}",
                    MessageCaption = "Update Log",
                    MessageButtons = frm_messageBox.Buttons.OK,
                    MessageIcon = frm_messageBox.Icon.Error
                }.ShowMessage();
            }
        }

        private void frm_setttings_Load(object sender, EventArgs e)
        {
            initilizeWait = true;
            cb_launchOnWindows.Checked = _launchOnWindows;
            cb_autoUpdate.Checked = (_isAutoUpdate == "true" || _isAutoUpdate != null) ? true : false;
            initilizeWait = false;

        }
    }
}
