set N $argv[1]
set script $argv[2]
set program $argv[3]

if test $program = "oursh"
    for i in (seq $N)
        oursh $script
    end
end

if test $program = "sh"
    for i in (seq $N)
        sh $script
    end
end

if test $program = "zsh"
    for i in (seq $N)
        zsh $script
    end
end

if test $program = "fish"
    for i in (seq $N)
        fish $script
    end
end

