set N 500

if test "$argv[1]" = "oursh"
    echo "bench oursh"
    time fish scripts/bench_loop.fish $N scripts/hello_world.sh oursh > /dev/null
    echo "bench oursh ruby"
    time fish scripts/bench_loop.fish $N scripts/ruby.sh oursh > /dev/null
    echo "bench oursh ruby piped"
    time fish scripts/bench_loop.fish $N scripts/ruby_piped.sh oursh > /dev/null
    echo "bench oursh ruby bridge"
    time fish scripts/bench_loop.fish $N scripts/ruby_bridge.sh oursh > /dev/null

else
    echo "bench sh"
    time fish scripts/bench_loop.fish $N scripts/hello_world.sh sh > /dev/null
    echo "bench oursh"
    time fish scripts/bench_loop.fish $N scripts/hello_world.sh oursh > /dev/null
    echo "bench zsh"
    time fish scripts/bench_loop.fish $N scripts/hello_world.sh zsh > /dev/null
    echo "bench fish"
    time fish scripts/bench_loop.fish $N scripts/hello_world.sh fish > /dev/null
end
