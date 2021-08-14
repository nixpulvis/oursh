
/// Every process group is in a unique session.
///
/// (When the process is created, it becomes a member of the session of its parent.) By convention,
/// the session ID of a session equals the process ID of the first member of the session, called
/// the session leader. A process finds the ID of its session using the system call getsid().
///
/// Every session may have a controlling tty, that then also is called the controlling tty of each
/// of its member processes. A file descriptor for the controlling tty is obtained by opening
/// /dev/tty. (And when that fails, there was no controlling tty.) Given a file descriptor for the
/// controlling tty, one may obtain the SID using tcgetsid(fd).
pub struct Session;
