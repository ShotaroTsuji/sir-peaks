gamma=0.05
beta=0.00016
cargo run --release simulate -g $gamma -i 1 -l $beta -n 2300 > data/cdg.txt

gnuplot << __EOF__
set terminal pdfcairo mono enhanced
set output 'sir-cdg.pdf'

set ylabel 'Infectious Population'
set xlabel 'Days'

plot 'data/cdg.txt' u (\$0*0.1):2 with lines title '{/Symbol b} = $beta, {/Symbol g} = $gamma'
__EOF__
