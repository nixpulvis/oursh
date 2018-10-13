#!/usr/bin/env python
# -*- coding: utf-8 -*-

import os
import sys
import platform
import signal
import readline

## BUILTINS
 # Global variable holding a dictionary of builtin names
 # and the functions they're associated with.
 #
 # { String => [List -> Boolean], ... }
 #
BUILTINS = { }

## {BUILTIN} cd : List -> Boolean
 # Attempts to change directories to the given path.
 # Or prints a reasonable error message.
 #
def cd(a):
  home = os.getenv("HOME")
  try:
    if a: os.chdir(a[0].replace('~', home))
    else: os.chdir(home)
    return True
  except Exception, e:
    print(e.args[1])
    return False
BUILTINS['cd'] = cd

## {BUILTIN} exit : List ->
 # Exits from this shell :)
 # We don't need to return here, because the program is done.
 #
def exit(a):
  sys.exit()
BUILTINS['exit'] = exit

## {BUILTIN} alias : List ->
 # Delegates to add_alias with the right number of arguments.
 #
def alias(a):
  if   len(a) == 0: print(ALIASES)
  elif len(a) >= 2: add_alias(a[0], " ".join(a[1:]))
  else: print("Wrong number of arguments for `alias`")
BUILTINS['alias'] = alias

## {BUILTIN} source : List ->
 # Opens a script file and executes every line of it,
 # then continues on prompting the user for input.
 #
def source(a):
  with open(a[0], 'r') as f:
    for x in f:
      execute(x.rstrip())
BUILTINS['source'] = source

## ALIASES
 # Global variable holding a dictionary of aliases for commands.
 # If a command matches an alias first it gets expanded then
 # re-executed.
 #
 # { String => String, ... }
ALIASES = { }

## add_alias : String String -> void
 # adds an alias to the given command with the given name.
 #
def add_alias( name, command ):
  ALIASES[name] = command

## is_interactive : -> Boolean
 # Returns True when this shell is intended
 # to be interacted with via terminal commands.
 #
def is_interactive():
  return len(sys.argv) == 1

## prompt : -> String
 # Returns a string to be placed before user input.
 #
def prompt():
  pwd = os.getcwd().split('/').pop()
  return pwd + " $ "

## get_command : File or False -> String
 # Returns a string of the current command to be executed
 #
 # From STDIN:
 #   when shell is interactive commands come from STDIN.
 #   To make users lives better we'll place a prompt before
 #   the input region.
 #
 # From File:
 #   when shell is executing a script file, each command
 #   is read from the file, per line.
 #
def get_command( script ):
  if script:
    command = script.readline()
    if command == "": sys.exit() # EOF
    return command
  else: return raw_input(prompt())

## execute : String -> Boolean
 # Executes the given system call or builtin, and returns
 # true if execution was successful.
 #
 # NOTE: Builtins override system calls.
 #
def execute( command ):
  argv = command.split(" ")
  if argv[0] in ALIASES:
    # Substitute the aliased command
    argv = ALIASES[argv[0]].split(" ") + argv[1:]
  if argv[0] in BUILTINS:
    # Always pass a list to this function, even if it's empty
    return BUILTINS[argv.pop(0)](argv)
  else:
    # Handles comments and environment variables
    return os.system(" ".join(argv)) == 0

# Trap SIGINT if is_interactive()
def sigint_handler( signal, frame ):
  print '\nExit this shell with `exit`'
  sys.stdout.write(prompt())
if is_interactive(): signal.signal(signal.SIGINT, sigint_handler)

# Open the given file if there was one
script = False if is_interactive() else open(sys.argv[1], 'r')

# Easy as that, tab completion making good use of the
# readline module.
def completer( text, state ):
  incomplete = text.split('/').pop()
  path = text.strip(incomplete)
  r_path = '.' if path == "" else path
  r_path = r_path.replace("~", os.getenv("HOME"))
  matches = [f for f in os.listdir(r_path) if f.startswith(incomplete)]
  if state < len(matches): return path + matches[state]
  else: return None

# NOTE: In versions of python < 2.7 a trailing
# whitespace character will be added upon tab
# completion, this is a bug in python's readline
# module.
readline.set_completer(completer)
readline.set_completer_delims(' ')
if platform.system() == "Darwin":
  readline.parse_and_bind("bind ^I rl_complete")
elif platform.system() == "Linux":
  readline.parse_and_bind("tab: complete")

# Source our profile
try: source(["./profile.myshell"])
except IOError: print 'No profile.myshell file found in current directory'

# Execute every command
while True: execute(get_command(script))