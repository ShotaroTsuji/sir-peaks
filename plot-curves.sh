cargo run --release simulate -l 1.1 > data/sir-11.txt
cargo run --release simulate -l 1.2 > data/sir-12.txt
cargo run --release simulate -l 1.5 > data/sir-15.txt
cargo run --release simulate -l 2.0 > data/sir-20.txt

gnuplot << __EOF__
set terminal pdfcairo mono enhanced
set output 'sir-icurves.pdf'

set format y '%.2f'

set ylabel 'Infectious'
set xlabel 'steps'

plot 'data/sir-11.txt' u 0:2 with lines title '{/Symbol l} = 1.1', \
     'data/sir-12.txt' u 0:2 with lines title '{/Symbol l} = 1.2', \
     'data/sir-15.txt' u 0:2 with lines title '{/Symbol l} = 1.5', \
     'data/sir-20.txt' u 0:2 with lines title '{/Symbol l} = 2.0'

set format y '10^{%L}'
set logscale y

plot 'data/sir-11.txt' u 0:2 with lines title '{/Symbol l} = 1.1', \
     'data/sir-12.txt' u 0:2 with lines title '{/Symbol l} = 1.2', \
     'data/sir-15.txt' u 0:2 with lines title '{/Symbol l} = 1.5', \
     'data/sir-20.txt' u 0:2 with lines title '{/Symbol l} = 2.0'

__EOF__
