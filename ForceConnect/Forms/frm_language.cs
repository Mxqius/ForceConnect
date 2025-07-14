using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using ForceConnect.Utilities;

namespace ForceConnect
{
    public partial class frm_language : Form
    {
        private Dictionary<string, string> _languageMapping;

        public frm_language()
        {
            InitializeComponent();
            InitializeLanguageForm();
        }

        /// <summary>
        /// Initializes the language selection form with proper Persian support.
        /// </summary>
        private void InitializeLanguageForm()
        {
            try
            {
                // Initialize language mapping
                _languageMapping = new Dictionary<string, string>
                {
                    { "English", "en" },
                    { "فارسی", "fa" }
                };

                // Load available languages
                LoadAvailableLanguages();

                // Set current language selection
                SetCurrentLanguageSelection();

                // Apply Persian settings if needed
                if (LocalizationManager.IsPersianLanguage())
                {
                    LocalizationManager.ApplyPersianSettings(this);
                }

                // Update form text
                UpdateFormTexts();
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to initialize language form: {ex.Message}");
            }
        }

        /// <summary>
        /// Loads available languages into the combo box.
        /// </summary>
        private void LoadAvailableLanguages()
        {
            try
            {
                cb_selectLanguage.Items.Clear();
                var availableLanguages = LocalizationManager.GetAvailableLanguages();

                foreach (var language in availableLanguages)
                {
                    cb_selectLanguage.Items.Add(language.Value);
                }

                if (cb_selectLanguage.Items.Count > 0)
                {
                    cb_selectLanguage.SelectedIndex = 0;
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to load available languages: {ex.Message}");
            }
        }

        /// <summary>
        /// Sets the current language selection in the combo box.
        /// </summary>
        private void SetCurrentLanguageSelection()
        {
            try
            {
                var currentLanguage = LocalizationManager.GetCurrentLanguage();
                var currentDisplayName = LocalizationManager.GetCurrentLanguageDisplayName();

                for (int i = 0; i < cb_selectLanguage.Items.Count; i++)
                {
                    if (cb_selectLanguage.Items[i].ToString() == currentDisplayName)
                    {
                        cb_selectLanguage.SelectedIndex = i;
                        break;
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to set current language selection: {ex.Message}");
            }
        }

        /// <summary>
        /// Updates form texts with localized strings.
        /// </summary>
        private void UpdateFormTexts()
        {
            try
            {
                gp_languageSelection.Text = LocalizationManager.GetString("languageSelection", "Choose a language");
                btn_confirm.Text = LocalizationManager.GetString("confirm", "Confirm");
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to update form texts: {ex.Message}");
            }
        }

        private void btn_confirm_Click(object sender, EventArgs e)
        {
            try
            {
                // Get selected language code
                var selectedLanguage = cb_selectLanguage.SelectedItem?.ToString();
                if (!string.IsNullOrEmpty(selectedLanguage) && _languageMapping.ContainsKey(selectedLanguage))
                {
                    var languageCode = _languageMapping[selectedLanguage];
                    LocalizationManager.SetCurrentLanguage(languageCode);
                }

                DialogResult = DialogResult.OK;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to confirm language selection: {ex.Message}");
                DialogResult = DialogResult.Cancel;
            }
        }

        private void cb_selectLanguage_SelectedIndexChanged(object sender, EventArgs e)
        {
            try
            {
                var selectedLanguage = cb_selectLanguage.SelectedItem?.ToString();
                if (string.IsNullOrEmpty(selectedLanguage)) return;

                // Update language image based on selection
                switch (selectedLanguage)
                {
                    case "English":
                        pb_language.Image = Properties.Resources.english;
                        break;
                    case "فارسی":
                        pb_language.Image = Properties.Resources.persian;
                        break;
                    default:
                        pb_language.Image = Properties.Resources.english;
                        break;
                }

                // Preview language change
                PreviewLanguageChange(selectedLanguage);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to handle language selection change: {ex.Message}");
            }
        }

        /// <summary>
        /// Previews the language change without applying it permanently.
        /// </summary>
        /// <param name="selectedLanguage">The selected language display name.</param>
        private void PreviewLanguageChange(string selectedLanguage)
        {
            try
            {
                if (_languageMapping.ContainsKey(selectedLanguage))
                {
                    var languageCode = _languageMapping[selectedLanguage];
                    var originalLanguage = LocalizationManager.GetCurrentLanguage();

                    // Temporarily set the language for preview
                    LocalizationManager.SetCurrentLanguage(languageCode);
                    UpdateFormTexts();

                    // Restore original language
                    LocalizationManager.SetCurrentLanguage(originalLanguage);
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to preview language change: {ex.Message}");
            }
        }
    }
}
