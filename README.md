# proc-hexdump
hexdump running processes

## Usage 
Build project binary

`cargo build`

MUST run as sudo as this project uses nix::sysio::process_vm_readv() which is libc
and requires root. Data must be read from /proc/xxxxx/smaps which requires root as well.

`sudo ./target/debug/proc-hexdump --process-name PROCESS`

Example output (will be much MUCH longer)
```
[0x563fd2bd6000]
7f 45 4c 46 02 01 01 00 00 00 00 00 00 00 00 00
[0x563fd2bd6010]
03 00 3e 00 01 00 00 00 90 2b 00 00 00 00 00 00
[0x563fd2bd6020]
40 00 00 00 00 00 00 00 40 82 00 00 00 00 00 00
[0x563fd2bd6030]
00 00 00 00 40 00 38 00 0d 00 40 00 1f 00 1e 00
[0x563fd2bd6040]
06 00 00 00 04 00 00 00 40 00 00 00 00 00 00 00
```
