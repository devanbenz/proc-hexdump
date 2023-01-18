# proc-hexdump
hexdump running processes

## Usage 
Build project binary

`cargo build`

MUST run as sudo as this project uses nix::sysio::process_vm_readv() which is libc
and requires root. Also data must be read from /proc/xxxxx/smaps which requires root as well.

`sudo ./target/debug/proc-hexdump --process-name PROCESS`
