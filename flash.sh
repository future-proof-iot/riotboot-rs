#!/bin/sh

openocd -f board/nordic_nrf52_dk.cfg -c 'init' -c 'targets' -c 'reset halt' -c "flash write_image erase \"$1\" 0 elf" -c "verify_image \"$1\" 0 elf" -c 'reset run' -c     'shutdown'
