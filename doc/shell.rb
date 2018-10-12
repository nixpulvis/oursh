# Commands to be executed directly by this shell.
BUILTINS = {}

# The builtin `cd` for changing the shell's working directory.
BUILTINS['cd'] = lambda do |args|
  # Change to the home directory by default.
  args << ENV['HOME'] if args.empty?

  # Read the destination path, doing very basic path expansion.
  dest = args.pop.gsub(/~/, ENV['HOME'])

  # Try to change this shell's working directory.
  begin
    Dir.chdir(dest)
  rescue Exception
    puts("no such directory: #{dest}")
  end
end

# The builtin to exit the shell.
BUILTINS['exit'] = lambda do |args|
  # Exit with a status of 0 by default.
  args << 0 if args.empty?

  # Exit the shell.
  exit args.pop.to_i
end

# Print a prompt, then read a line from STDIN.
def read
  print('$ ')

  # Try to read a line, exiting if there's no more lines.
  line = STDIN.gets
  if line
    line.chomp.split(' ')
  else
    exit
  end
end

# Run the given command in a subprocess.
def evaluate(argv)
  if BUILTINS[argv[0]]
    BUILTINS[argv[0]].call(argv[1..-1])
  else
    success = system(*argv)
    unless success
      puts("unknown command '#{argv[0]}'")
    end
  end
end

# Don't exit on `SIGINT`, instead simply return a new prompt.
trap('INT') { print("\n$ ") }

# The glorious REPL itself.
loop { evaluate(read) }
