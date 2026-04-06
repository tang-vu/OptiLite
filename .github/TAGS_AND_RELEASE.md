# 🏷️ Tags & Releases Guide

## Current Version
**v1.0.0** - Initial Release

---

## 📋 How to Create a Release

### Method 1: Via GitHub CLI (Recommended)

```bash
# 1. Create and push tag
git tag -a v1.0.0 -m "Initial release - OptiLite v1.0.0"
git push origin v1.0.0

# 2. Create release with assets
gh release create v1.0.0 \
  --title "OptiLite v1.0.0 - Initial Release" \
  --notes-file RELEASE_NOTES.md \
  --draft=false \
  --prerelease=false
```

### Method 2: Via GitHub UI

1. Go to https://github.com/tang-vu/OptiLite/releases
2. Click "Draft a new release"
3. Tag version: `v1.0.0`
4. Target: `main`
5. Release title: `OptiLite v1.0.0 - Initial Release`
6. Description: Copy from `RELEASE_NOTES.md`
7. Attach binaries (if built locally)
8. Click "Publish release"

### Method 3: Automated via CI/CD

The GitHub Actions workflow will automatically:
1. Build on Windows, macOS, and Linux
2. Create release when tag is pushed
3. Upload all binaries as assets

---

## 🏷️ Tag Naming Convention

Follow [Semantic Versioning](https://semver.org/):

```
vMAJOR.MINOR.PATCH

Examples:
v1.0.0  - Initial release
v1.0.1  - Bug fix
v1.1.0  - New features
v2.0.0  - Breaking changes
```

---

## 📦 Build Binaries Locally

### Windows
```bash
cd src-tauri
cargo tauri build
# Output: src-tauri/target/release/bundle/nsis/*.exe
```

### macOS
```bash
cd src-tauri
cargo tauri build --target x86_64-app-darwin    # Intel
cargo tauri build --target aarch64-apple-darwin  # Apple Silicon
# Output: src-tauri/target/*/release/bundle/dmg/*.dmg
```

### Linux
```bash
cd src-tauri
cargo tauri build
# Output: src-tauri/target/release/bundle/deb/*.deb
```

---

## 🚀 Release Checklist

Before publishing:

- [ ] Update version in `package.json`
- [ ] Update version in `src-tauri/Cargo.toml`
- [ ] Update version in `src-tauri/tauri.conf.json`
- [ ] Update `CHANGELOG.md`
- [ ] Update `RELEASE_NOTES.md`
- [ ] Test build locally (`npm run build`)
- [ ] Run all tests
- [ ] Commit all changes
- [ ] Create and push tag
- [ ] Wait for CI/CD to complete
- [ ] Verify release on GitHub
- [ ] Announce release

---

## 📝 Tag Commands

### List all tags
```bash
git tag -l
```

### Create lightweight tag
```bash
git tag v1.0.0
git push origin v1.0.0
```

### Create annotated tag (Recommended)
```bash
git tag -a v1.0.0 -m "Release v1.0.0"
git push origin v1.0.0
```

### Delete tag
```bash
git tag -d v1.0.0
git push --delete origin v1.0.0
```

### Checkout a tag
```bash
git checkout v1.0.0
```

---

## 🎯 Next Release Targets

### v1.1.0 (Planned)
- [ ] GPU optimization
- [ ] Network optimization
- [ ] Gaming mode profile
- [ ] System tray icon
- [ ] Desktop notifications

### v1.2.0 (Future)
- [ ] Auto-updates
- [ ] Multi-language support
- [ ] Export optimization reports
- [ ] Custom optimization profiles

---

## 🔐 Code Signing (Optional)

### Windows
```bash
# Requires code signing certificate
cargo tauri build -- --features custom-protocol
```

### macOS
```bash
# Requires Apple Developer certificate
cargo tauri build --target x86_64-apple-darwin -- --features custom-protocol
```

---

**For CI/CD automation, see `.github/workflows/ci-cd.yml`**
