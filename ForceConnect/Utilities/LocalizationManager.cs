using System;
using System.Collections.Generic;
using System.Drawing;
using System.Globalization;
using System.IO;
using System.Linq;
using System.Text;
using System.Threading.Tasks;
using System.Windows.Forms;
using Newtonsoft.Json;

namespace ForceConnect.Utilities
{
    /// <summary>
    /// Manages application localization and language support including Persian (Farsi).
    /// </summary>
    public static class LocalizationManager
    {
        private static Dictionary<string, Dictionary<string, string>> _translations;
        private static string _currentLanguage = "en";
        private static readonly string _languageFilePath = Path.Combine(Application.StartupPath, "Languages", "languages.json");
        private static readonly object _lockObject = new object();

        /// <summary>
        /// Supported languages in the application.
        /// </summary>
        public static readonly Dictionary<string, string> SupportedLanguages = new Dictionary<string, string>
        {
            { "en", "English" },
            { "fa", "فارسی" }
        };

        /// <summary>
        /// Initializes the localization manager.
        /// </summary>
        public static void Initialize()
        {
            LoadTranslations();
            SetCurrentLanguage(GetSavedLanguage());
        }

        /// <summary>
        /// Loads translations from the language file.
        /// </summary>
        private static void LoadTranslations()
        {
            try
            {
                if (File.Exists(_languageFilePath))
                {
                    var jsonContent = File.ReadAllText(_languageFilePath, Encoding.UTF8);
                    _translations = JsonConvert.DeserializeObject<Dictionary<string, Dictionary<string, string>>>(jsonContent);
                }
                else
                {
                    _translations = new Dictionary<string, Dictionary<string, string>>();
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to load translations: {ex.Message}");
                _translations = new Dictionary<string, Dictionary<string, string>>();
            }
        }

        /// <summary>
        /// Gets a localized string for the current language.
        /// </summary>
        /// <param name="key">The translation key.</param>
        /// <param name="defaultValue">Default value if translation is not found.</param>
        /// <returns>The localized string.</returns>
        public static string GetString(string key, string defaultValue = null)
        {
            try
            {
                if (_translations != null && 
                    _translations.ContainsKey(_currentLanguage) && 
                    _translations[_currentLanguage].ContainsKey(key))
                {
                    return _translations[_currentLanguage][key];
                }

                // Fallback to English if current language doesn't have the key
                if (_translations != null && 
                    _translations.ContainsKey("en") && 
                    _translations["en"].ContainsKey(key))
                {
                    return _translations["en"][key];
                }

                return defaultValue ?? key;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get localized string for key '{key}': {ex.Message}");
                return defaultValue ?? key;
            }
        }

        /// <summary>
        /// Sets the current language and applies it to the application.
        /// </summary>
        /// <param name="languageCode">The language code (e.g., "en", "fa").</param>
        public static void SetCurrentLanguage(string languageCode)
        {
            if (string.IsNullOrWhiteSpace(languageCode) || !SupportedLanguages.ContainsKey(languageCode))
            {
                languageCode = "en";
            }

            lock (_lockObject)
            {
                _currentLanguage = languageCode;
                SaveLanguagePreference(languageCode);
                ApplyLanguageToApplication();
            }
        }

        /// <summary>
        /// Gets the current language code.
        /// </summary>
        /// <returns>The current language code.</returns>
        public static string GetCurrentLanguage()
        {
            return _currentLanguage;
        }

        /// <summary>
        /// Gets the current language display name.
        /// </summary>
        /// <returns>The current language display name.</returns>
        public static string GetCurrentLanguageDisplayName()
        {
            return SupportedLanguages.ContainsKey(_currentLanguage) 
                ? SupportedLanguages[_currentLanguage] 
                : "English";
        }

        /// <summary>
        /// Checks if the current language is Persian (Farsi).
        /// </summary>
        /// <returns>True if current language is Persian, false otherwise.</returns>
        public static bool IsPersianLanguage()
        {
            return _currentLanguage == "fa";
        }

        /// <summary>
        /// Applies the current language settings to the application.
        /// </summary>
        private static void ApplyLanguageToApplication()
        {
            try
            {
                // Set culture for Persian language
                if (IsPersianLanguage())
                {
                    var persianCulture = new CultureInfo("fa-IR");
                    CultureInfo.DefaultThreadCurrentCulture = persianCulture;
                    CultureInfo.DefaultThreadCurrentUICulture = persianCulture;
                }
                else
                {
                    var englishCulture = new CultureInfo("en-US");
                    CultureInfo.DefaultThreadCurrentCulture = englishCulture;
                    CultureInfo.DefaultThreadCurrentUICulture = englishCulture;
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to apply language settings: {ex.Message}");
            }
        }

        /// <summary>
        /// Applies Persian font and RTL settings to a control.
        /// </summary>
        /// <param name="control">The control to apply Persian settings to.</param>
        public static void ApplyPersianSettings(Control control)
        {
            if (control == null) return;

            try
            {
                if (IsPersianLanguage())
                {
                    // Apply RTL for Persian
                    control.RightToLeft = RightToLeft.Yes;
                    control.RightToLeftLayout = true;

                    // Apply Persian font if available
                    ApplyPersianFont(control);
                }
                else
                {
                    // Reset to LTR for English
                    control.RightToLeft = RightToLeft.No;
                    control.RightToLeftLayout = false;
                }

                // Apply to child controls
                foreach (Control childControl in control.Controls)
                {
                    ApplyPersianSettings(childControl);
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to apply Persian settings to control: {ex.Message}");
            }
        }

        /// <summary>
        /// Applies Persian font to a control if available.
        /// </summary>
        /// <param name="control">The control to apply font to.</param>
        private static void ApplyPersianFont(Control control)
        {
            try
            {
                PersianFontManager.ApplyPersianFont(control);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to apply Persian font: {ex.Message}");
            }
        }

        /// <summary>
        /// Saves the language preference to registry or settings.
        /// </summary>
        /// <param name="languageCode">The language code to save.</param>
        private static void SaveLanguagePreference(string languageCode)
        {
            try
            {
                // Save to registry or settings file
                var settingsPath = Path.Combine(Application.StartupPath, "settings.json");
                var settings = new Dictionary<string, object>
                {
                    { "Language", languageCode }
                };

                var jsonSettings = JsonConvert.SerializeObject(settings, Formatting.Indented);
                File.WriteAllText(settingsPath, jsonSettings, Encoding.UTF8);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to save language preference: {ex.Message}");
            }
        }

        /// <summary>
        /// Gets the saved language preference.
        /// </summary>
        /// <returns>The saved language code or "en" as default.</returns>
        private static string GetSavedLanguage()
        {
            try
            {
                var settingsPath = Path.Combine(Application.StartupPath, "settings.json");
                if (File.Exists(settingsPath))
                {
                    var jsonSettings = File.ReadAllText(settingsPath, Encoding.UTF8);
                    var settings = JsonConvert.DeserializeObject<Dictionary<string, object>>(jsonSettings);
                    
                    if (settings.ContainsKey("Language"))
                    {
                        var language = settings["Language"].ToString();
                        if (SupportedLanguages.ContainsKey(language))
                        {
                            return language;
                        }
                    }
                }
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to get saved language: {ex.Message}");
            }

            return "en";
        }

        /// <summary>
        /// Formats a string with Persian number conversion if needed.
        /// </summary>
        /// <param name="format">The format string.</param>
        /// <param name="args">The format arguments.</param>
        /// <returns>The formatted string.</returns>
        public static string FormatString(string format, params object[] args)
        {
            try
            {
                var result = string.Format(format, args);
                
                if (IsPersianLanguage())
                {
                    // Convert English numbers to Persian numbers
                    result = ConvertToPersianNumbers(result);
                }
                
                return result;
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Failed to format string: {ex.Message}");
                return format;
            }
        }

        /// <summary>
        /// Converts English numbers to Persian numbers.
        /// </summary>
        /// <param name="text">The text to convert.</param>
        /// <returns>The text with Persian numbers.</returns>
        private static string ConvertToPersianNumbers(string text)
        {
            if (string.IsNullOrEmpty(text)) return text;

            var persianNumbers = new Dictionary<char, char>
            {
                {'0', '۰'}, {'1', '۱'}, {'2', '۲'}, {'3', '۳'}, {'4', '۴'},
                {'5', '۵'}, {'6', '۶'}, {'7', '۷'}, {'8', '۸'}, {'9', '۹'}
            };

            var result = new StringBuilder();
            foreach (char c in text)
            {
                result.Append(persianNumbers.ContainsKey(c) ? persianNumbers[c] : c);
            }

            return result.ToString();
        }

        /// <summary>
        /// Gets all available languages with their display names.
        /// </summary>
        /// <returns>Dictionary of language codes and display names.</returns>
        public static Dictionary<string, string> GetAvailableLanguages()
        {
            return new Dictionary<string, string>(SupportedLanguages);
        }

        /// <summary>
        /// Reloads translations from the language file.
        /// </summary>
        public static void ReloadTranslations()
        {
            LoadTranslations();
        }
    }
} 