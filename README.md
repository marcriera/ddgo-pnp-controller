# Controller mod for Densha de GO! Plug & Play

This mod allows you to use your Densha de GO! Plug & Play as a USB controller for other games.

## Requirements

- USB flash drive
- Powered micro USB OTG adapter
- Micro USB data cable

## Installation

1. Prepare the USB flash drive by formatting it with a single FAT32 partition.
2. Download the latest release from the [Releases](https://github.com/MarcRiera/ddgo-pnp-controller/releases) section.
3. Extract the content to the root of the USB drive.
4. Plug the USB drive to the OTG adapter, plug the adapter to the Plug & Play and plug the data cable to the adapter and a power source.
5. Turn on the Plug & Play. The door lamp will turn on to show the installation has begun. Once it has finished, the lamp will turn off. Turn off the unit and remove everything.

## Uninstallation

To completely remove the mod, follow these steps. **You need the backup of the original kernel created during installation**. 

1. Prepare the USB flash drive by formatting it with a single FAT32 partition.
2. Download the latest release from the [Releases](https://github.com/MarcRiera/ddgo-pnp-controller/releases) section.
3. Extract the content to the root of the USB drive.
4. Create an empty file named `revert` on the root of the USB drive.
5. Put the original (backup) files `uImage` and `mali.ko` into a folder named `BACKUP` on the root of the USB drive.
6. Plug the USB drive to the OTG adapter, plug the adapter to the Plug & Play and plug the data cable to the adapter and a power source.
7. Turn on the Plug & Play. The door lamp will turn on to show the uninstallation has begun. Once it has finished, the lamp will turn off. Turn off the unit and remove everything.

## Usage

Connect the Plug & Play to a PC or console using the data cable. Press one of the following button combinations to select the controller you want to emulate:

| Controller                              | Button combination       | Notes                                            |
|-----------------------------------------|--------------------------|--------------------------------------------------|
| One handle controller (Nintendo Switch) | UP                       | SELECT+START=HOME, SELECT+LEFT=L, SELECT+RIGHT=R |
| Two handle controller (PC)              | RIGHT                    | D-Pad is mapped to SELECT+ABCD                   |
| Two handle controller (PS1)             | DOWN + Power handle at 0 | Hold D to disable handles and enable D-Pad       |
| Two handle controller (Nintendo 64)     | DOWN + Power handle at 1 |                                                  |
| Two handle controller (Sega Saturn)     | DOWN + Power handle at 2 |                                                  |
| Two handle controller (Sega Dreamcast)  | DOWN + Power handle at 3 |                                                  |
| Two handle controller "Type 2" (PS2)    | D                        |                                                  |
| Shinkansen controller (PS2)             | B                        | Power notches are mapped to P2-P4-P7-P10-P13     |
| Multi Train Controller (PS2) - P4/B7    | C + Power handle at 0    | SELECT+A=A2, SELECT+D=ATS, SELECT+D-Pad=Reverser |
| Multi Train Controller (PS2) - P4/B2-B7 | C + Power handle at 1    | SELECT+A=A2, SELECT+D=ATS, SELECT+D-Pad=Reverser |
| Multi Train Controller (PS2) - P5/B5    | C + Power handle at 2    | SELECT+A=A2, SELECT+D=ATS, SELECT+D-Pad=Reverser |
| Multi Train Controller (PS2) - P5/B7    | C + Power handle at 3    | SELECT+A=A2, SELECT+D=ATS, SELECT+D-Pad=Reverser |
| Generic Train Controller                | A                        |                                                  |

Hold the buttons until the controller vibrates to confirm selection. If no button is pressed, you can play with the Plug & Play as usual.

If you need more information regarding each controller and supported software, please check the [Densha de GO! controller documentation](https://marcriera.github.io/ddgo-controller-docs).

## Usage with emulators

The mod can be used with certain emulators and compatible games. Instructions are provided per system.

### Nintendo 64 (RetroArch)

1. Use mode *Two handle controller (Nintendo 64)*.
2. In the emulator's settings, assign the controller to **port 3**. The controller should map automatically.
3. Enable the setting **Independent C-Buttons controls** in the core's settings.
4. Select the correct controller type in-game.

### Nintendo Switch (any)

1. Use mode *One handle controller (Nintendo Switch)*.
2. In the emulator's settings, set the controller type to **Pro Controller**. The controller should map automatically.
3. Select the correct controller type in-game.

### Sega Dreamcast (Flycast)

1. Use mode *Two handle controller (Sega Dreamcast)*.
2. In the emulator's settings, set the controller type to **Densha de GO!**. The controller should map automatically.

### Sega Dreamcast (RetroArch)

1. Use mode *Two handle controller (Sega Dreamcast)*.
2. In the emulator's settings, set the controller type to **Densha de GO!**. The controller should map automatically.

### Sega Saturn (RetroArch)

1. Use mode *Two handle controller (Sega Saturn)*. The controller should map automatically.
2. Select the correct controller type in-game.

### Sony PlayStation (DuckStation)

1. Use mode *Two handle controller (PS1)*.
2. Add the [SDL controller mappings](SDL_mappings.md) to your PC.
3. In the emulator's settings, configure a regular digital controller (**not a Densha de GO! controller**). The controller should map automatically.
4. If needed, hold **D** to temporarily disable the controller and enable the D-Pad for navigation.

### Sony PlayStation (RetroArch)

1. Use mode *Two handle controller (PS1)*.
2. In the emulator's settings, configure a regular digital controller (**not a Densha de GO! controller**). The controller should map automatically.
3. If needed, hold **D** to temporarily disable the controller and enable the D-Pad for navigation.

### Sony PlayStation 2 (PCSX2)

1. Use mode *Generic Train Controller*.
2. In the emulator's settings, configure a USB Type 2 Train Controller. Map the buttons/axes manually.

### Sony PlayStation 3 (RPCS3)

1. Use mode *Two handle controller "Type 2" (PS2)*.
2. Follow the OS-specific instructions in the [RPCS3 wiki](https://wiki.rpcs3.net/index.php?title=Help:Peripherals_and_accessories) to make sure the emulator can see the controller.
3. No further configuration required, the emulator will communicate with the controller directly via USB.

## RNDIS access (advanced users)

When no controller is selected, RNDIS access is enabled in the device. You can access SSH on the Plug & Play at 169.254.215.100. SFTP is not supported out of the box, but SCP is available. Keep in mind the root filesystem is mounted read-only by default.

## Notes

- During the first installation, if no previous mods are detected, the device's original kernel is backed up to a folder named *BACKUP* in the root of the USB drive. Copy its contents to a safe location.
- If detected, the [original mod by GMMan](https://github.com/GMMan/dengo-plug-and-play-controller) will be uninstalled to avoid conflicts.

## Compilation

To compile the program yourself, you will need Rust and toolchain for Armv7-A. The easiest way is to install [cross](https://github.com/cross-rs/cross) and run:

```cross build --target arm-unknown-linux-musleabi --release```

The Linux kernel source can be found [here](https://github.com/MarcRiera/dengo-plug-and-play-kernel).
