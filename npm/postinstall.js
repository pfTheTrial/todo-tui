#!/usr/bin/env node
const os = require('os');
const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

const VERSION = '1.0.0';
const REPO = 'pfTheTrial/todo-tui';
const BIN_NAME = process.platform === 'win32' ? 'tdt.exe' : 'tdt';
const INSTALL_DIR = path.join(__dirname, '..', 'bin');

function getPlatformAsset() {
  const platform = os.platform();
  const arch = os.arch();

  if (platform === 'win32' && arch === 'x64') return `todo-tui-windows-amd64.exe`;
  if (platform === 'linux' && arch === 'x64') return `todo-tui-linux-amd64`;
  if (platform === 'darwin' && arch === 'arm64') return `todo-tui-macos-arm64`;
  if (platform === 'darwin' && arch === 'x64') return `todo-tui-macos-amd64`;
  return null;
}

function downloadFile(url, dest, redirects = 0) {
  if (redirects > 5) {
    console.error('Too many redirects');
    process.exit(1);
  }
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, { headers: { 'User-Agent': 'todo-tui-installer' } }, (res) => {
      if (res.statusCode === 301 || res.statusCode === 302) {
        file.close();
        fs.unlinkSync(dest);
        downloadFile(res.headers.location, dest, redirects + 1).then(resolve).catch(reject);
        return;
      }
      if (res.statusCode !== 200) {
        reject(new Error(`HTTP ${res.statusCode}`));
        return;
      }
      res.pipe(file);
      file.on('finish', () => { file.close(); resolve(); });
    }).on('error', (err) => {
      fs.unlinkSync(dest);
      reject(err);
    });
  });
}

async function install() {
  const asset = getPlatformAsset();
  if (!asset) {
    console.error(`❌ Unsupported platform: ${os.platform()} ${os.arch()}`);
    console.log('Please install Rust and run: cargo install --git https://github.com/pfTheTrial/todo-tui');
    process.exit(1);
  }

  const url = `https://github.com/${REPO}/releases/download/v${VERSION}/${asset}`;
  const dest = path.join(INSTALL_DIR, BIN_NAME);

  if (!fs.existsSync(INSTALL_DIR)) {
    fs.mkdirSync(INSTALL_DIR, { recursive: true });
  }

  console.log(`📦 Downloading tdt v${VERSION} for ${os.platform()}/${os.arch()}...`);
  console.log(`   URL: ${url}`);

  try {
    await downloadFile(url, dest);
    if (process.platform !== 'win32') {
      fs.chmodSync(dest, '755');
    }
    console.log(`✅ tdt installed successfully! Run: tdt`);
  } catch (err) {
    console.error(`❌ Download failed: ${err.message}`);
    console.log('Fallback: Please install Rust and run: cargo install --git https://github.com/pfTheTrial/todo-tui');
    process.exit(1);
  }
}

install();
