set N 1000

echo "bench oursh"
time fish scripts/bench_loop.fish $N scripts/hello_world.sh oursh > /dev/null
echo "bench oursh ruby"
time fish scripts/bench_loop.fish $N scripts/ruby.sh oursh > /dev/null
echo "bench oursh ruby piped"
time fish scripts/bench_loop.fish $N scripts/ruby_piped.sh oursh > /dev/null
echo "bench oursh ruby bridge"
time fish scripts/bench_loop.fish $N scripts/ruby_bridge.sh oursh > /dev/null
