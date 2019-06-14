# OzSecCon 2019 badge design.
designed in kicad v4
FW developed using [embedded rust] (https://rust-embedded.github.io).

# Instructions for use
## Using the badge
Bottom Button plays track
Top Button is to change track

Long press on Top to change card
Long press on Bottom to exit magspoof mode
(i was going to make a pattern thing appear but ran out of time)

## Loading new cards
card program is present at 
    ./ozseccon2019_utility/cardutil.py

Cards are formatted as a JSON file
Badge uses USB HID to communication.
i've only used ubuntu 14.04 to test so YMMV.

python libs used are:
hid
time
argparse
json

1. Leave battery in.
1. CAREFULLY insert a USB micro cable.
1. check that it enumerates
    \#lsusb
    004 Device 001: ID 1d6b:0003 Linux Foundation 3.0 root hub
    Bus 003 Device 107: ID ffff:ffff  
    Bus 003 Device 001: ID 1d6b:0002 Linux Foundation 2.0 root hub
you should see ID ffff:ffff show up.
1. run cardutil as root - this is due to using USB YMMV
```
    ozseccon2019_utility$ sudo python ./cardutil.py --cardfile ./carddata.json 
    ________             _________              _________                   
    \_____  \  ________ /   _____/  ____   ____ \_   ___ \   ____    ____   
     /   |   \ \___   / \_____  \ _/ __ \_/ ___\/    \  \/  /  _ \  /    \  
    /    |    \ /    /  /        \  ___/\  \___\     \____(  <_> )|   |  \ 
    \_______  //_____ \/_______  / \___  >\___  >\______  / \____/ |___|  / 
            \/       \/        \/      \/     \/        \/              \/  
                                                                            
    
    ________ _______   ____  ________  
    \_____  \   _  \ /_   |/   __   \ 
     /  ____//  /_\  \ |   |\____    / 
    /       \  \_/   \|   |   /    /  
    \_______ \_____  /|___|  /____/   
            \/      \/                 
                                       
    
    _________                     .___.____                       .___               
    \_   ___ \ _____  _______   __| _/|    |     ____ _____     __| _/ ____ _______  
    /    \  \/ \__  \ \_  __ \ / __ | |    |    /  _ \__  \   / __ |_/ __ \_  __ \ 
    \     \____ / __ \_|  | \// /_/ | |    |___(  <_> )/ __ \_/ /_/ |\  ___/ |  | \/ 
     \______  /(____  /|__|   \____ | |_______ \____/(____  /\____ | \___  >|__|    
            \/      \/             \/         \/           \/      \/     \/         
    usage: sudo python ./cardutil.py --cardfile [json formatted card file]
    [*] loading cards from ./carddata.json
    [*] loading card 1:
    [*] track1: %B4444444444444444^ABE/LINCOLN^291110100000931?D
    [*] track2: ;4444444444444444=29111010000093100000?9
    [*] track3: 000000000000000000000000000
    [*] loading card 2:
    [*] track1: %B1234567890123445^PADILLA/L.                ^9901120000000000000055123888888?~
    [*] track2: ;1234567890123445=99011200XXXX00000000?*
    [*] track3: ;011234567890123445=72472410000000000003030033040400099010=4320943290432==1=0000000000000000??
    [*] loading card 3:
    [*] track1: %B4888603170607238^Head/Potato^050510100000000001203191805191000000?
    [*] track2: ;4888603170607238=05051011203191805191?
    [*] track3: ;1337?
    Manufacturer: OzSecCon.
    Product: MagSpoof
    Serial No: 1.0.0
    Closing the device
    [*] finished
    
    
    ./ozseccon2019_utility/cardutil.py
```
1. You should now be able to use it.

## reprogramming.
Convert file using the (elf2dfuse utility)[https://github.com/majbthrd/elf2dfuse]

1. Short J3 to get device into bootloader mode
1. reset - you should see the two LEDS light up 
1. check that DFU mode, should see ID 0483:df11
    peter@peter-ThinkPad-X230:/media/peter/VM Images/ozseccon2019/ozseccon2019_badge$ lsusb
    Bus 003 Device 119: ID 0483:df11 STMicroelectronics STM Device in DFU Mode
1. check that dfu-util detects board
```
    ~/src/elf2dfuse$ sudo dfu-util -l
    enter sudo password 
    dfu-util 0.8
    
    Copyright 2005-2009 Weston Schmidt, Harald Welte and OpenMoko Inc.
    Copyright 2010-2014 Tormod Volden and Stefan Schmidt
    This program is Free Software and has ABSOLUTELY NO WARRANTY
    Please report bugs to dfu-util@lists.gnumonks.org
    
    Found DFU: [0483:df11] ver=2200, devnum=123, cfg=1, intf=0, alt=1, name="@Option Bytes  /0x1FFFF800/01*016 e", serial="FFFFFFFEFFFF"
    Found DFU: [0483:df11] ver=2200, devnum=123, cfg=1, intf=0, alt=0, name="@Internal Flash  /0x08000000/032*0001Kg", serial="FFFFFFFEFFFF"
    Found Runtime: [0a5c:21e6] ver=0112, devnum=4, cfg=1, intf=3, alt=0, name="UNKNOWN", serial="083E8EE37645"
```
1. use alt 0 - internal flash and program your DFU file
```
    ~/src/elf2dfuse$ sudo dfu-util -d 0483:df11 -a 0 -D ./final.dfu 
    dfu-util 0.8
    
    Copyright 2005-2009 Weston Schmidt, Harald Welte and OpenMoko Inc.
    Copyright 2010-2014 Tormod Volden and Stefan Schmidt
    This program is Free Software and has ABSOLUTELY NO WARRANTY
    Please report bugs to dfu-util@lists.gnumonks.org
    
    Opening DFU capable USB device...
    ID 0483:df11
    Run-time device DFU version 011a
    Claiming USB DFU Interface...
    Setting Alternate Setting #0 ...
    Determining device status: state = dfuERROR, status = 10
    dfuERROR, clearing status
    Determining device status: state = dfuIDLE, status = 0
    dfuIDLE, continuing
    DFU mode device DFU version 011a
    Device returned transfer size 2048
    DfuSe interface name: "Internal Flash  "
    file contains 1 DFU images
    parsing DFU image 1
    image for alternate setting 0, (2 elements, total size = 12780)
    parsing element 1, address = 0x08000000, size = 12052
    Download    [=========================] 100%        12052 bytes
    Download done.
    parsing element 2, address = 0x08002f20, size = 712
    Download    [=========================] 100%          712 bytes
    Download done.
    done parsing DfuSe file
 ```
 
# Stuff i stole/copied from to make this badge
[Samy Kamkar's Magspoof](http://samy.pl/magspoof/)
[RyscCorp's version](https://github.com/RyscCorp/magspoof_r3)
[Aleh Zasypkin Awesome Kroneum project](https://github.com/azasypkin/kroneum)

