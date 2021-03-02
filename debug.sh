#!/bin/sh

openocd -f board/nordic_nrf52_dk.cfg -c 'init' -c 'targets' -c 'reset halt'
