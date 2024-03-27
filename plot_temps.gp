set title "CPU Temperature Over Time"
set xlabel "Time"
set ylabel "Temperature (°C)"
set xdata time
set timefmt "%s"
set format x "%H:%M:%S"
plot "~/cpu_temps/temp_log.txt" using 1:2 with lines title "Core 0"

