# Updating the app icons

## 1. Flutter

### 1. Android, iOS, MacOS, Windows
1. update the icon source files at `qaul_ui/assets/logo`:
    * `icon_android.png` - It's advised for the source icon to have padding around it to avoid having the resulting 
      image too big or too small. 
      See [this issue](https://github.com/fluttercommunity/flutter_launcher_icons/issues/96) for more information.
    * `icon_ios.png` - iOS icons should [fill the entire image](https://stackoverflow.com/questions/26014461/black-border-on-my-ios-icon) 
      and not contain transparent borders.
    * `icon_desktop.png` - The icon will, effectively, be a resized copy of the source. If you want it to be rounded, 
      circular or something of the sort, edit the image accordingly.

2. Run the following command:
```sh
flutter pub get
flutter pub run flutter_launcher_icons:main
```

### 2. Linux
Replace the image `qaul_ui/snap/gui/qaul.png` with a 512x512 png of same name.

## 2. Installers
### 1. Macos
1. Update the icon source over at `utilities/icon/Icon1024.png`.
2. Run the following command:
```sh
cd utilities/installers/macos
sh bin/geticns
```

### 2. Windows
Replace the image `utilities/installers/windows/assets/app_icon.ico` with the result of the generated icon from
the step *1.1.2* - which is found at `qaul_ui/windows/runner/resources/app_icon.ico`.
