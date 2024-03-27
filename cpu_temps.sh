#!/bin/bash

mkdir -p ~/cpu_temps
while true; do
	date +%s >> ~/cpu_temps/temp_log.txt
	sensors | grep 'Core 0' | awk '{print $3}' >> ~/cpu_temps/temp_log.txt
	sleep 5
done

