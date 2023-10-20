# BeamMP-Map-Change-Plugin-for-Linux

Command line based Map selector, Mod downloader and Auto Restart tool, and BeamNG plugin for chat based map votes.

For this Program to work the Executable needs to be in the linux sub-folder for the BeamMP server and "screen" needs to be installed on your linux server.

"./BeamNGEdit init" will automatically initialize the program and create nessecary files for the Plugin, "./BeamNGEdit help" will explain any other important commands 

You will also need to edit the "Launch Skript" for BeamMP called "BeamMPStart.sh", it will be automatically created in the same dictionary as the Command line tool. you will just need to change the "/BeamMP-for-your-distro-.22.04" line to the correct executable name for your distro. 

The Plugin isn't nearly done yet so there will be a bunch of issues with the voting system (if used wrong), but the general purpose of switching maps by vote and restarting the server after 60 seconds should be fulfilled.



If the release executable doesn't work on your system, you will need to download the source files, open the folder in your terminal, build the executable with "cargo build" (you might have to install the rust programming kit beforehand, tutorials for which are plenty online. ) and move the executable from ./BeamNGEditor/target/debug to the linux folder of your server. 