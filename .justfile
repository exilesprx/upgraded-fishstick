processor := "m328p"
board := "ardunio"
port := "/dev/ttyAMC0"
rate := 115200
file := ""


default:
  just --list

flash:
  avrdude -p {{processor}} -c {{board}} -P {{port}} -b {{rate}} -U flash:w:{{file}}
