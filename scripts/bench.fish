set N 250

echo "bench sh"
time fish scripts/bench_loop.fish $N scripts/hello_world.sh sh    > /dev/null
echo "bench oursh"
time fish scripts/bench_loop.fish $N scripts/hello_world.sh oursh > /dev/null
echo "bench fish"
time fish scripts/bench_loop.fish $N scripts/hello_world.sh fish > /dev/null
