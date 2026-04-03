# XOS Wallet - Chrome Extension

A simple Web3 wallet Chrome extension for XOS with basic wallet functionality.

## Features

✅ **Create Wallet** - Generates a new 12-word seed phrase  
✅ **Import Wallet** - Import existing wallet using seed phrase  
✅ **Lock/Unlock** - Password protection (test password: 123456)  
✅ **Dashboard** - View dummy address and balance  
✅ **Send Transaction** - Dummy transaction with alert confirmation  

## Files Included

1. `manifest.json` - Extension configuration (Manifest V3)
2. `popup.html` - UI with all screens (welcome, create, import, lock, dashboard)
3. `popup.js` - All wallet logic using chrome.storage.local

## How to Install

1. Open Chrome and go to `chrome://extensions/`
2. Enable **Developer mode** (toggle in top right)
3. Click **Load unpacked**
4. Select the `browser-extension` folder
5. Extension icon will appear in toolbar

## Usage Flow

### First Time Setup
1. Click extension icon → See welcome screen
2. Choose "Create New Wallet" or "Import Existing Wallet"

### Create New Wallet
1. Click "Create New Wallet"
2. Copy the generated 12-word seed phrase
3. Set a password (min 6 characters)
4. Confirm password
5. Click "Create Wallet"
6. Wallet is created with dummy balance of 1000.00 XOS

### Import Existing Wallet
1. Click "Import Existing Wallet"
2. Enter 12-word seed phrase (space-separated)
3. Set a password
4. Click "Import Wallet"

### Lock/Unlock
- Click "Lock" button on dashboard to lock wallet
- Enter password to unlock (default test: 123456)

### Send Transaction
1. On dashboard, enter recipient address
2. Enter amount in XOS
3. Click "Send XOS"
4. Alert confirms transaction (dummy)

## Technical Details

- **No WASM** - Pure JavaScript
- **No External APIs** - All data stored locally
- **Storage** - Uses `chrome.storage.local` for persistence
- **Seed Generation** - Simplified BIP39 word list (2048 words)
- **Address Generation** - Simple hash-based dummy addresses

## Security Note

⚠️ **This is a demo/test wallet only!**
- Passwords are stored in plain text (for demo purposes)
- No real cryptographic operations
- No blockchain connectivity
- Do NOT use with real funds or seed phrases

## Testing

Test password: `123456`

## Troubleshooting

If extension doesn't load:
1. Check `chrome://extensions/` for error messages
2. Verify all files are in the folder
3. Check console for JavaScript errors (F12 → Console)

## License

MIT License
