cargo run --release peaks -s 1.001 -e 3.0 -d 0.001 -i 1e-7 > data/ipeaks-1e-7.txt
cargo run --release peaks -s 1.001 -e 3.0 -d 0.001 -i 1e-4 > data/ipeaks-1e-4.txt

gnuplot << __EOF__
set terminal pdfcairo mono enhanced
set output 'sir-ipeaks.pdf'

set ylabel 'Peak of Infectious'
set xlabel '{/Symbol l}'

set xr [1:3]
set format y "%1.2f"
plot 'data/ipeaks-1e-7.txt' u 1:2 with lines title 'I(0) = 1e-7'

set xr [1:1.1]
set format y "%1.4f"
plot 'data/ipeaks-1e-7.txt' u 1:2 with lines title 'I(0) = 1e-7'

set xr [1:1.01]
set format y "%1.1tx10^{%T}"
plot 'data/ipeaks-1e-7.txt' u 1:2 with lines title 'I(0) = 1e-7'

set xr [1:3]
set format y "%1.2f"
plot 'data/ipeaks-1e-4.txt' u 1:2 with lines title 'I(0) = 1e-4'

set xr [1:1.1]
set format y "%1.4f"
plot 'data/ipeaks-1e-4.txt' u 1:2 with lines title 'I(0) = 1e-4'

set xr [1:1.01]
set format y "%1.2tx10^{%T}"
plot 'data/ipeaks-1e-4.txt' u 1:2 with lines title 'I(0) = 1e-4'

__EOF__
