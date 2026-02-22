# SDL mappings

The special modes for [console controllers](https://traincontrollerdb.marcriera.cat/hardware/index.html#console) need to be mapped correctly to be usable in emulators. If the emulator uses SDL to handle input, you can take advantage of this and map the controller automatically:

**Note**: RetroArch supports SDL but already includes config files to map the controllers automatically and no action is needed. You only need this for standalone emulators.
 
## Windows

1. Open your emulator's installation folder.
2. Look for a file named **gamecontrollerdb.txt**.
3. Add the following lines at the end of the file:
```
030000000912000000d5000000000000,TAITO Densha de Go! Plug & Play (PS1 Two Handle mode),a:b1,b:b2,x:b0,y:b14,back:b8,start:b9,leftshoulder:b10,rightshoulder:b12,dpup:b4,dpleft:b15,dpdown:b5,dpright:b16,lefttrigger:b11,righttrigger:b13,crc:4f5b,platform:Windows,
030000000912000000d5000000000000,TAITO Densha de Go! Plug & Play (N64 Two Handle mode),a:b1,b:b12,x:b0,y:b13,back:b2,start:b9,leftshoulder:b11,rightshoulder:b10,lefttrigger:b16,righttrigger:b8,dpup:b15,dpright:b14,crc:fab2,platform:Windows,
030000000912000000d5000000000000,TAITO Densha de Go! Plug & Play (SAT Two Handle mode),a:b0,b:b1,x:b14,y:b15,back:b8,start:b9,leftshoulder:b16,rightshoulder:b2,dpleft:b13,dpdown:b12,lefttrigger:b10,righttrigger:b11,crc:d4ac,platform:Windows,
030000000912000000d5000000000000,TAITO Densha de Go! Plug & Play (DC Two Handle mode),a:b0,b:b1,x:b16,y:b15,start:b9,leftshoulder:b2,rightshoulder:b14,dpup:b10,dpleft:b12,dpdown:b11,dpright:b13,lefttrigger:b8,crc:2079,platform:Windows,
```
*You may need to edit the file again after updating the emulator, as it may be overwritten.*

## GNU/Linux (Wayland)

1. Create the file **~/.config/environment.d/70-sdl-ddgo-pnp.conf** with the following content:
```
SDL_GAMECONTROLLERCONFIG="
03005b4f0912000000d5000011010000,TAITO Densha de Go! Plug & Play (PS1 Two Handle mode),a:b1,b:b2,x:b0,y:b14,back:b8,start:b9,leftshoulder:b10,rightshoulder:b12,dpup:b4,dpleft:b15,dpdown:b5,dpright:b16,lefttrigger:b11,righttrigger:b13,crc:4f5b,platform:Linux,
0300b2fa0912000000d5000011010000,TAITO Densha de Go! Plug & Play (N64 Two Handle mode),a:b1,b:b12,x:b0,y:b13,back:b2,start:b9,leftshoulder:b11,rightshoulder:b10,lefttrigger:b16,righttrigger:b8,dpup:b15,dpright:b14,crc:fab2,platform:Linux,
0300acd40912000000d5000011010000,TAITO Densha de Go! Plug & Play (SAT Two Handle mode),a:b0,b:b1,x:b14,y:b15,back:b8,start:b9,leftshoulder:b16,rightshoulder:b2,dpleft:b13,dpdown:b12,lefttrigger:b10,righttrigger:b11,crc:d4ac,platform:Linux,
030079200912000000d5000011010000,TAITO Densha de Go! Plug & Play (DC Two Handle mode),a:b0,b:b1,x:b16,y:b15,start:b9,leftshoulder:b2,rightshoulder:b14,dpup:b10,dpleft:b12,dpdown:b11,dpright:b13,lefttrigger:b8,crc:2079,platform:Linux,
"
```
2. Restart the session.

## GNU/Linux (X)

1. Open the file **~/.xprofile** (or create it if it doesn't exist) and add the following content:
```
export SDL_GAMECONTROLLERCONFIG="
03005b4f0912000000d5000011010000,TAITO Densha de Go! Plug & Play (PS1 Two Handle mode),a:b1,b:b2,x:b0,y:b14,back:b8,start:b9,leftshoulder:b10,rightshoulder:b12,dpup:b4,dpleft:b15,dpdown:b5,dpright:b16,lefttrigger:b11,righttrigger:b13,crc:4f5b,platform:Linux,
0300b2fa0912000000d5000011010000,TAITO Densha de Go! Plug & Play (N64 Two Handle mode),a:b1,b:b12,x:b0,y:b13,back:b2,start:b9,leftshoulder:b11,rightshoulder:b10,lefttrigger:b16,righttrigger:b8,dpup:b15,dpright:b14,crc:fab2,platform:Linux,
0300acd40912000000d5000011010000,TAITO Densha de Go! Plug & Play (SAT Two Handle mode),a:b0,b:b1,x:b14,y:b15,back:b8,start:b9,leftshoulder:b16,rightshoulder:b2,dpleft:b13,dpdown:b12,lefttrigger:b10,righttrigger:b11,crc:d4ac,platform:Linux,
030079200912000000d5000011010000,TAITO Densha de Go! Plug & Play (DC Two Handle mode),a:b0,b:b1,x:b16,y:b15,start:b9,leftshoulder:b2,rightshoulder:b14,dpup:b10,dpleft:b12,dpdown:b11,dpright:b13,lefttrigger:b8,crc:2079,platform:Linux,
"
```
2. Restart the session.
