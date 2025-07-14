using System;
using System.Windows.Forms;

namespace ForceConnect.Utilities
{
    /// <summary>
    /// Test utility for Persian language support functionality.
    /// </summary>
    public static class LanguageTest
    {
        /// <summary>
        /// Tests the Persian language support functionality.
        /// </summary>
        /// <returns>True if all tests pass, false otherwise.</returns>
        public static bool TestPersianLanguageSupport()
        {
            try
            {
                var results = new System.Collections.Generic.List<bool>();

                // Test 1: Check if localization manager initializes
                try
                {
                    LocalizationManager.Initialize();
                    results.Add(true);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Localization manager test failed: {ex.Message}");
                    results.Add(false);
                }

                // Test 2: Check if Persian fonts are available
                try
                {
                    var fontsAvailable = PersianFontManager.ArePersianFontsAvailable();
                    results.Add(fontsAvailable);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Persian fonts test failed: {ex.Message}");
                    results.Add(false);
                }

                // Test 3: Test Persian string retrieval
                try
                {
                    LocalizationManager.SetCurrentLanguage("fa");
                    var persianText = LocalizationManager.GetString("welcomeText", "Welcome");
                    var isPersian = !string.IsNullOrEmpty(persianText) && persianText != "Welcome";
                    results.Add(isPersian);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Persian string test failed: {ex.Message}");
                    results.Add(false);
                }

                // Test 4: Test English string retrieval
                try
                {
                    LocalizationManager.SetCurrentLanguage("en");
                    var englishText = LocalizationManager.GetString("welcomeText", "Welcome");
                    var isEnglish = !string.IsNullOrEmpty(englishText);
                    results.Add(isEnglish);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"English string test failed: {ex.Message}");
                    results.Add(false);
                }

                // Test 5: Test Persian number conversion
                try
                {
                    LocalizationManager.SetCurrentLanguage("fa");
                    var formattedNumber = LocalizationManager.FormatString("Test {0}", 123);
                    var hasPersianNumbers = formattedNumber.Contains("۱۲۳");
                    results.Add(hasPersianNumbers);
                }
                catch (Exception ex)
                {
                    System.Diagnostics.Debug.WriteLine($"Persian number test failed: {ex.Message}");
                    results.Add(false);
                }

                // Return true if all tests pass
                return results.Count > 0 && results.TrueForAll(r => r);
            }
            catch (Exception ex)
            {
                System.Diagnostics.Debug.WriteLine($"Language test failed: {ex.Message}");
                return false;
            }
        }

        /// <summary>
        /// Gets a detailed test report for Persian language support.
        /// </summary>
        /// <returns>Detailed test report string.</returns>
        public static string GetTestReport()
        {
            try
            {
                var report = new System.Text.StringBuilder();
                report.AppendLine("=== Persian Language Support Test Report ===");
                report.AppendLine();

                // Test localization manager
                report.AppendLine("1. Localization Manager Test:");
                try
                {
                    LocalizationManager.Initialize();
                    report.AppendLine("   ✓ Initialized successfully");
                }
                catch (Exception ex)
                {
                    report.AppendLine($"   ✗ Failed: {ex.Message}");
                }

                // Test Persian fonts
                report.AppendLine("\n2. Persian Fonts Test:");
                try
                {
                    var fontsAvailable = PersianFontManager.ArePersianFontsAvailable();
                    var availableFonts = PersianFontManager.GetAvailablePersianFonts();
                    var bestFont = PersianFontManager.GetBestPersianFont();
                    
                    report.AppendLine($"   Available fonts: {string.Join(", ", availableFonts)}");
                    report.AppendLine($"   Best font: {bestFont}");
                    report.AppendLine(fontsAvailable ? "   ✓ Fonts available" : "   ✗ No fonts available");
                }
                catch (Exception ex)
                {
                    report.AppendLine($"   ✗ Failed: {ex.Message}");
                }

                // Test Persian strings
                report.AppendLine("\n3. Persian Strings Test:");
                try
                {
                    LocalizationManager.SetCurrentLanguage("fa");
                    var persianText = LocalizationManager.GetString("welcomeText", "Welcome");
                    report.AppendLine($"   Persian welcome text: {persianText}");
                    report.AppendLine(!string.IsNullOrEmpty(persianText) ? "   ✓ Persian strings loaded" : "   ✗ No Persian strings");
                }
                catch (Exception ex)
                {
                    report.AppendLine($"   ✗ Failed: {ex.Message}");
                }

                // Test English strings
                report.AppendLine("\n4. English Strings Test:");
                try
                {
                    LocalizationManager.SetCurrentLanguage("en");
                    var englishText = LocalizationManager.GetString("welcomeText", "Welcome");
                    report.AppendLine($"   English welcome text: {englishText}");
                    report.AppendLine(!string.IsNullOrEmpty(englishText) ? "   ✓ English strings loaded" : "   ✗ No English strings");
                }
                catch (Exception ex)
                {
                    report.AppendLine($"   ✗ Failed: {ex.Message}");
                }

                // Test Persian numbers
                report.AppendLine("\n5. Persian Numbers Test:");
                try
                {
                    LocalizationManager.SetCurrentLanguage("fa");
                    var formattedNumber = LocalizationManager.FormatString("Test {0}", 123);
                    report.AppendLine($"   Formatted number: {formattedNumber}");
                    report.AppendLine(formattedNumber.Contains("۱۲۳") ? "   ✓ Persian numbers working" : "   ✗ Persian numbers not working");
                }
                catch (Exception ex)
                {
                    report.AppendLine($"   ✗ Failed: {ex.Message}");
                }

                // Overall test result
                report.AppendLine("\n=== Overall Result ===");
                var overallResult = TestPersianLanguageSupport();
                report.AppendLine(overallResult ? "✓ All tests passed" : "✗ Some tests failed");

                return report.ToString();
            }
            catch (Exception ex)
            {
                return $"Test report generation failed: {ex.Message}";
            }
        }
    }
} 