# Vencord Installer

The Vencord Installer allows you to install [Vencord, the cutest Discord Desktop client mod](https://github.com/Vendicated/Vencord)

![image](https://user-images.githubusercontent.com/45497981/226734476-5fb42420-844d-4e27-ae06-4799118e086e.png)

## Usage

### Windows

> **Warning**
**Do not** run the installer as Admin

Download [VencordInstaller.exe](https://github.com/Vencord/Installer/releases/latest/download/VencordInstaller.exe) and run it

If the above doesn't work/open, for example because you're using Windows 7, 32 bit, or have a bad GPU, you can instead use our terminal based installer.

To do so, open Powershell, run the following command, then follow along with the instructions/prompts

```ps1
iwr "https://raw.githubusercontent.com/Vencord/Installer/main/install.ps1" -UseBasicParsing | iex
```

### Linux

Run the following command in your terminal and follow along with the instructions/prompts

```sh
sh -c "$(curl -sS https://raw.githubusercontent.com/Vendicated/VencordInstaller/main/install.sh)"
```

### MacOS

Download the latest [MacOs build](https://github.com/Vencord/Installer/releases/latest/download/VencordInstaller.MacOS.zip), unzip it, and run `VencordInstaller.app` 

If you get a `VencordInstaller can't be opened` warning, right-click `VencordInstaller.app` and click open.

This warning shows because the app isn't signed since I'm not willing to pay 100 bucks a year for an Apple Developer license.

___

## Building from source

### Prerequisites 

You need to install the [nightly Rust toolchain](https://www.rust-lang.org/tools/install).

### Building

TODO
