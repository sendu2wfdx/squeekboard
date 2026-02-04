*squeekboard* - a Wayland on-screen keyboard
========================================

[![CI](https://github.com/sendu2wfdx/squeekboard/actions/workflows/ci.yml/badge.svg)](https://github.com/sendu2wfdx/squeekboard/actions/workflows/ci.yml)
[![Build and Release](https://github.com/sendu2wfdx/squeekboard/actions/workflows/build-and-release.yml/badge.svg)](https://github.com/sendu2wfdx/squeekboard/actions/workflows/build-and-release.yml)

*Squeekboard* is the on-screen-keyboard input-method for Phosh. It is primarily designed for smartphones, tablet-PCs, and other devices with touchscreens.

It squeaks because some Rust got inside.

Features
--------

### Present

- GTK3
- Custom keyboard layouts defined in yaml
- Input purpose dependent keyboard layouts
- DBus interface to show and hide
- Use Wayland input method protocol to submit text
- Use Wayland virtual keyboard protocol

### TODO

- [Port to GTK4 / GTK4-Layer-Shell](https://gitlab.gnome.org/World/Phosh/squeekboard/-/issues/64)
- [Text prediction/correction](https://gitlab.gnome.org/World/Phosh/squeekboard/-/issues/54)
- Use preedit
- Submit actions like "next field" using a future Wayland protocol
- Pick up DBus interface files from /usr/share

Creating layouts
-------------------

If you want to work on layouts, check out the [guide](doc/tutorial.md).

GitHub Actions CI/CD
--------------------

This repository includes GitHub Actions workflows for automated building, testing, and releasing:

### Automated Builds
- **Continuous Integration**: Every push triggers a quick build and test
- **Multi-Architecture**: Builds for both AMD64 and ARM64 (Raspberry Pi 5)
- **Artifact Upload**: All builds produce downloadable .deb packages

### Creating Releases
To create a new release with packaged .deb files:

```sh
# Tag your version
git tag -a v1.44.0 -m "Release version 1.44.0"
git push origin v1.44.0
```

GitHub Actions will automatically:
1. Build .deb packages for AMD64 and ARM64
2. Create a GitHub Release with the tag
3. Upload the packages as release assets

### Download Pre-built Packages
- **From Releases**: Visit the [Releases page](../../releases) to download the latest .deb packages
- **From Actions**: Download build artifacts from any workflow run in the [Actions tab](../../actions)

### Documentation
- English guide: [.github/README.md](.github/README.md)
- 中文说明: [.github/README.zh-CN.md](.github/README.zh-CN.md)

Nightly builds
--------------

For testing the latest commits of the `main`-branch, one can install the nightly builds of Squeekboard.
For more information about the nightly builds, read the ["Phosh Nightly Package Builds"-blog-post](https://phosh.mobi/posts/phosh-nightly/).

Building
--------

### Dependencies

See `.gitlab-ci.yml` or run `apt-get build-dep .`

### Build from git repo

```sh
$ git clone https://gitlab.gnome.org/World/Phosh/squeekboard.git
$ cd squeekboard
$ mkdir _build
$ meson setup _build/
$ cd _build
$ ninja
```

To run tests use `ninja test`. To install squeekboard run `ninja install`.

Running
-------

```sh
$ cd ../build/
$ src/squeekboard
```

If no compatible Wayland compositor is running yet, you can use Phoc (after installing it):

```sh
$ phoc --exec 'src/squeekboard'
```

Squeekboard's panel will appear whenever a compatible application requests an input method. Click a text field in any GTK application, like `python3 ./tools/entry.py`.

Squeekboard honors the gnome "screen-keyboard-enabled" setting. Either enable this through gnome-settings under accessibility or run:

```sh
$ gsettings set org.gnome.desktop.a11y.applications screen-keyboard-enabled true
```

Alternatively, force panel visibility manually with:

```sh
$ busctl call --user sm.puri.OSK0 /sm/puri/OSK0 sm.puri.OSK0 SetVisible b true
```

or by using the environment-variable `SQUEEKBOARD_DEBUG=force_show`.

### What the compositor has to support

A compatible compositor has to support the protocols:

- layer-shell
- virtual-keyboard-v1

It's strongly recommended to support:

- input-method-v2

Settings
--------

You can change the height of the panel for the keyboard with:

```sh
$ gsettings set sm.puri.Squeekboard scale-in-horizontal-screen-orientation 1.0
$ gsettings set sm.puri.Squeekboard scale-in-vertical-screen-orientation 1.0
```
and wether or not layouts will stretch to fit the panel with:

```sh
$ gsettings set sm.puri.Squeekboard layout-shape-changes-to-fit-panel true
$ gsettings set sm.puri.Squeekboard layout-shape-changes-to-fit-panel false
```

Note: If the keyboard is open when the settings are changed, the changes will not be visible until the keyboard is opened again, or the layout is changed.
While using Phosh, you can long-click/long-tap the home-bar at the bottom, to open and close the keyboard.

To reset the settings to the default, you can use:

```sh
$ gsettings reset-recursively sm.puri.Squeekboard
```

Developing
----------

See [`doc/hacking.md`](doc/hacking.md) for this copy, or the [official documentation](https://world.pages.gitlab.gnome.org/Phosh/squeekboard) for the current release.
