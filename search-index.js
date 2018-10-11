var N = null;var searchIndex = {};
searchIndex["oursh"]={"doc":"This shell should be both POSIX compatible and yet modern and exciting. Fancy features should not be prevented by POSIX compatibility. This will effect the design of the shell.","items":[[0,"job","oursh","Subprocess execution management.",N,N],[3,"Job","oursh::job","A job to be executed by various means.",N,N],[11,"new","","Create a new job from the given command.",0,[[["command"]],["self"]]],[11,"run","","Run a shell job, waiting for the command to finish.",0,[[["self"]]]],[0,"program","oursh","Parsing and handling program syntax(es) of the shell.",N,N],[5,"parse_default","oursh::program","Parse a program of the default type.",N,[[["r"]],["defaultprogram"]]],[5,"parse","","Parse a program of the given type.",N,[[["r"]],["p"]]],[0,"basic","","Single command programs with no features.",N,N],[3,"BasicProgram","oursh::program::basic","A basic program with only a single command.",N,N],[3,"BasicCommand","","A single poorly parsed command.",N,N],[0,"posix","oursh::program","The ubiquitous POSIX shell command language.",N,N],[3,"PosixProgram","oursh::program::posix","",N,N],[6,"DefaultProgram","oursh::program","The default program type, used for unannotated blocks.",N,N],[8,"Command","","A command is a task given by the user as part of a `Program`.",N,N],[10,"argv","","Return the command's arguments (including it's name).",1,[[["self"]],["vec",["cstring"]]]],[11,"name","","Return the name of this command.",1,[[["self"]],["cstring"]]],[8,"Program","","A program is as large as a file or as small as a line.",N,N],[16,"Command","","The type of each of this program's commands.",2,N],[10,"parse","","Parse a whole program from the given `reader`.",2,[[["r"]],["self"]]],[10,"commands","","Return a list of all the commands in this program.",2,[[["self"]],["vec"]]],[0,"repl","oursh","",N,N],[5,"is_tty","oursh::repl","Is this stream a TTY?",N,[[["t"]],["bool"]]],[5,"prompt","","",N,[[["stdout"]]]],[5,"trap_sigint","","",N,[[],["result",["sigaction"]]]],[11,"from","oursh::job","",0,[[["t"]],["t"]]],[11,"into","","",0,[[["self"]],["u"]]],[11,"try_from","","",0,[[["u"]],["result"]]],[11,"borrow","","",0,[[["self"]],["t"]]],[11,"get_type_id","","",0,[[["self"]],["typeid"]]],[11,"try_into","","",0,[[["self"]],["result"]]],[11,"borrow_mut","","",0,[[["self"]],["t"]]],[11,"from","oursh::program::basic","",3,[[["t"]],["t"]]],[11,"into","","",3,[[["self"]],["u"]]],[11,"try_from","","",3,[[["u"]],["result"]]],[11,"borrow","","",3,[[["self"]],["t"]]],[11,"get_type_id","","",3,[[["self"]],["typeid"]]],[11,"try_into","","",3,[[["self"]],["result"]]],[11,"borrow_mut","","",3,[[["self"]],["t"]]],[11,"from","","",4,[[["t"]],["t"]]],[11,"into","","",4,[[["self"]],["u"]]],[11,"to_owned","","",4,[[["self"]],["t"]]],[11,"clone_into","","",4,N],[11,"try_from","","",4,[[["u"]],["result"]]],[11,"borrow","","",4,[[["self"]],["t"]]],[11,"get_type_id","","",4,[[["self"]],["typeid"]]],[11,"try_into","","",4,[[["self"]],["result"]]],[11,"borrow_mut","","",4,[[["self"]],["t"]]],[11,"from","oursh::program::posix","",5,[[["t"]],["t"]]],[11,"into","","",5,[[["self"]],["u"]]],[11,"try_from","","",5,[[["u"]],["result"]]],[11,"borrow","","",5,[[["self"]],["t"]]],[11,"get_type_id","","",5,[[["self"]],["typeid"]]],[11,"try_into","","",5,[[["self"]],["result"]]],[11,"borrow_mut","","",5,[[["self"]],["t"]]],[11,"argv","oursh::program::basic","Treat each space blindly as an argument delimiter.",4,[[["self"]],["vec",["cstring"]]]],[11,"parse","","Create a new program from the given reader.",3,[[["r"]],["self"]]],[11,"commands","","Return the single parsed command.",3,[[["self"]],["vec"]]],[11,"parse","oursh::program::posix","",5,[[["r"]],["self"]]],[11,"commands","","",5,[[["self"]],["vec"]]],[11,"clone","oursh::program::basic","",4,[[["self"]],["basiccommand"]]]],"paths":[[3,"Job"],[8,"Command"],[8,"Program"],[3,"BasicProgram"],[3,"BasicCommand"],[3,"PosixProgram"]]};
initSearch(searchIndex);
