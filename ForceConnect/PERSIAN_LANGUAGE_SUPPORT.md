# Persian Language Support for ForceConnect

## Overview

ForceConnect now includes comprehensive Persian (Farsi) language support with proper Right-to-Left (RTL) text rendering, Persian font management, and complete localization.

## Features

### 1. Complete Persian Localization
- **Full UI Translation**: All user interface elements are translated to Persian
- **Error Messages**: All error messages and notifications are localized
- **Context Menus**: Right-click context menus are fully translated
- **Status Messages**: Connection status and progress messages are localized

### 2. Persian Font Support
- **Automatic Font Detection**: Automatically detects and uses the best available Persian fonts
- **Font Installation**: Can install Persian fonts from the Fonts directory
- **Fallback Support**: Uses Tahoma, Arial, or Segoe UI as fallback fonts
- **RTL Support**: Proper Right-to-Left text rendering for Persian text

### 3. Persian Number Conversion
- **Automatic Conversion**: English numbers are automatically converted to Persian numbers
- **Format Support**: Supports string formatting with Persian numbers
- **Consistent Display**: Numbers appear in Persian format throughout the application

### 4. Language Management
- **Language Selection**: Easy language switching through the settings menu
- **Persistent Settings**: Language preference is saved and restored
- **Real-time Switching**: Language changes are applied immediately
- **Preview Mode**: Preview language changes before applying

## Supported Languages

- **English (en)**: Default language
- **Persian/Farsi (fa)**: Complete Persian localization

## File Structure

```
ForceConnect/
├── Languages/
│   └── languages.json          # Translation strings
├── Utilities/
│   ├── LocalizationManager.cs  # Main localization system
│   ├── PersianFontManager.cs   # Persian font management
│   └── LanguageTest.cs         # Testing utilities
└── Forms/
    └── frm_language.cs         # Language selection form
```

## Usage

### Changing Language

1. Open the application
2. Go to Settings (تنظیمات)
3. Click on "Language Selection" (انتخاب زبان)
4. Select "فارسی" for Persian or "English" for English
5. Click "Confirm" (تایید)

### Programmatic Usage

```csharp
// Set Persian language
LocalizationManager.SetCurrentLanguage("fa");

// Get localized string
string welcomeText = LocalizationManager.GetString("welcomeText", "Welcome");

// Format with Persian numbers
string formatted = LocalizationManager.FormatString("Test {0}", 123);
// Result: "Test ۱۲۳"

// Apply Persian settings to a form
LocalizationManager.ApplyPersianSettings(this);
```

## Translation Keys

The application uses the following main translation keys:

### UI Elements
- `home`: Home button text
- `explore`: Explore button text
- `settings`: Settings button text
- `about`: About button text
- `latency`: Latency label
- `address`: Address label

### Status Messages
- `connected`: Connected status
- `disconnected`: Disconnected status
- `connecting`: Connecting status
- `disconnecting`: Disconnecting status

### Error Messages
- `error`: Error title
- `warning`: Warning title
- `info`: Information title
- `success`: Success title

### Actions
- `confirm`: Confirm button
- `cancel`: Cancel button
- `yes`: Yes button
- `no`: No button

## Font Support

### Available Persian Fonts
1. **Tahoma**: Primary Persian font (Windows default)
2. **Arial**: Alternative Persian font
3. **Segoe UI**: Modern Persian font
4. **Microsoft Sans Serif**: Legacy Persian font
5. **Verdana**: Web-safe Persian font

### Font Installation
The application can install additional Persian fonts from the `Fonts/` directory:
- Place `.ttf` font files in the `Fonts/` directory
- Fonts are automatically installed on application startup
- Only fonts with Persian character support are used

## Testing

Use the `LanguageTest` utility to verify Persian language support:

```csharp
// Run comprehensive test
bool allTestsPass = LanguageTest.TestPersianLanguageSupport();

// Get detailed test report
string report = LanguageTest.GetTestReport();
```

## Technical Details

### RTL Implementation
- Uses `RightToLeft.Yes` for Persian language
- Uses `RightToLeftLayout = true` for proper layout
- Handles both RTL and LTR text in the same application

### Culture Settings
- Sets `CultureInfo` to `fa-IR` for Persian
- Sets `CultureInfo` to `en-US` for English
- Applies culture settings to the current thread

### JSON Configuration
- Uses `Newtonsoft.Json` for parsing language files
- Supports UTF-8 encoding for Persian characters
- Fallback to English if Persian translation is missing

## Troubleshooting

### Common Issues

1. **Persian text not displaying correctly**
   - Check if Persian fonts are installed
   - Verify the language file is properly encoded in UTF-8
   - Ensure the application has font installation permissions

2. **RTL layout issues**
   - Verify `RightToLeft` and `RightToLeftLayout` are set correctly
   - Check if the form is properly initialized with Persian settings

3. **Language not switching**
   - Check if the language file exists and is valid JSON
   - Verify the language code is correct ("fa" for Persian)
   - Check application permissions for writing settings

### Debug Information

Use the following methods to get debug information:

```csharp
// Get font information
string fontInfo = PersianFontManager.GetFontInformation();

// Get test report
string testReport = LanguageTest.GetTestReport();

// Check current language
string currentLang = LocalizationManager.GetCurrentLanguage();
```

## Future Enhancements

- Support for additional languages (Arabic, Turkish, etc.)
- Dynamic font loading from external sources
- Advanced RTL layout management
- Custom font embedding
- Language-specific UI layouts

## Contributing

To add new translations or improve existing ones:

1. Edit `Languages/languages.json`
2. Add new translation keys and values
3. Test with `LanguageTest.TestPersianLanguageSupport()`
4. Update this documentation

## License

This Persian language support is part of the ForceConnect project and follows the same license terms. 