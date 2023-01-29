# erichVK5-suzy-Q-cable-v1

A simple closed debugging cable breakout PCB to allow reflashing of firmware on Chromebooks using signed firmware/CR50 systems to allow alternative operating systems to be installed more easily.

The prototype PCB in the FOSS layout editor pcb-rnd: 

![prototype layout in pcb-rnd](images/PrototypeLayout-V1.png)

The schematic in the FOSS schematic editor sch-rnd:

![prototype schematic in sch-rnd](erichVK5-suzy-Q-v1-1.svg)

The board is 48mm wide and 44mm high. Manufacture of the boards and testing is a work in progress.

The board is designed to accommodate a readily available and inexpensive USB type C 3.1 breakout board with a male connector, and two standard USB type B connectors with kinked solder tabs (i.e. Molex 670687041) to allow assembly with simple through hole soldering, and avoiding the need for an expensive USB 3.1 cable.

The 5V pins on the Type B USB cables are not connected to the USB type C VCC pins by default, in case different machines (with slightly different USB "5V" voltages) might be used for monitoring ttyUSB0 and ttyUSB1. The rails can be commoned up by putting links across JMP0 and/or JMP1. A link placed across JMP3 can be used to common the USB type C cable shield to the ground pins/ground network of the circuit.
