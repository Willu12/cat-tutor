# cat-tutor

A Chrome extension that puts a pixel cat on LeetCode problem pages, written in Rust + WebAssembly.

## Requirements

- Rust with the `wasm32-unknown-unknown` target (`rustup target add wasm32-unknown-unknown`)
- [`wasm-pack`](https://rustwasm.github.io/wasm-pack/)
- Chrome (or another Chromium-based browser)

## Build

```bash
./run.sh           # dev build (faster, readable function names in stack traces)
./run.sh release   # release build (optimized)
```

The output goes to `extension/pkg/`, which git ignores.

## Install

1. Open `chrome://extensions` and turn on **Developer mode**.
2. Click **Load unpacked** and select the `extension/` folder.
3. Open any problem at `https://leetcode.com/problems/...`. Logs show up in the DevTools console.

## Reload routine

| What you changed               | What to do                                     |
| ------------------------------ | ---------------------------------------------- |
| Rust code (`src/`)             | `./run.sh` → refresh the tab                   |
| `content.js` / `manifest.json` | ⟳ in `chrome://extensions` → refresh the tab   |

If you still see an old version after rebuilding, hard-refresh the tab (Ctrl+Shift+R) or reload the extension.
