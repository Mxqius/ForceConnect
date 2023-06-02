using ForceConnect.Launch;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;

namespace ForceConnect
{
    public partial class frm_settings : Form
    {
        private bool _launchOnWindows, initilizeWait;
        private Timer timerInitilizeByPass;
        public frm_settings()
        {
            InitializeComponent();
            _launchOnWindows = LaunchProgram.IsAppSetToRunAtStartup();
            initilizeWait = true;

            timerInitilizeByPass = new Timer();
            timerInitilizeByPass.Interval = 1000;
            timerInitilizeByPass.Start();
            timerInitilizeByPass.Tick += TimerInitilizeByPass_Tick;
        }

        private void TimerInitilizeByPass_Tick(object sender, EventArgs e)
        {
            initilizeWait = false;
            timerInitilizeByPass.Stop();
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

        private void frm_setttings_Load(object sender, EventArgs e)
        {
            cb_launchOnWindows.Checked = _launchOnWindows;
        }
    }
}
