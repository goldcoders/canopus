# Canopus - Desktop Site Manager for Hugo Sites

## Get started

Install the dependencies...

```bash
git clone https://github.com/goldcoders/canopus
cd canopus
yarn
```

Start Devserver

```bash
yarn dev
```

Start Tauri Development Window

```bash
yarn tauri dev
```


## Building and running in production mode

To create an optimised version of the app:

```bash
yarn build
```

Bundle Application with tauri

```js
yarn tauri build
```

## Changelog
- Added Recommeded VScode Settings and Plugins
- Added Stylelint
- Added Example callTauriTestCMD

## WIP Todo for v1.0.0
- CMS
  - Markdown Editor
  - HTML Editor
- Sidebar Manager
  - Sections and Submenu with Collections of Folder & Files
- Workflow
  - Dependency Downloaded (ie. Git, Hugo , Netlify)
  - Custom Script (ie. hugo server , netlify lambda, netlify dev)
  - Deploy Script ( Sync Site)
- Site Creator
  - Theme (Options to Delete content or Use Demo Content)
  - Normal Hugo site
- Account Manager (License / Deploy Keys per Site)
  - Git API Integrations
