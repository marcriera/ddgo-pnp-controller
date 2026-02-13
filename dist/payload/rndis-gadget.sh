#!/bin/sh -e

echo 1209 > "/sys/class/android_usb/android0/idVendor"
echo d500 > "/sys/class/android_usb/android0/idProduct"
echo -n "TAITO" > "/sys/class/android_usb/android0/iManufacturer"
echo -n "Densha de Go! Plug & Play (RNDIS mode)" > "/sys/class/android_usb/android0/iProduct"
echo "rndis" > "/sys/class/android_usb/android0/functions"
echo 1 > "/sys/class/android_usb/android0/f_rndis/wceis"
echo 1 > "/sys/class/android_usb/android0/enable"
sleep 1
ifconfig rndis0 169.254.215.100 netmask 255.255.0.0
