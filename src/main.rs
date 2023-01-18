use std::{io::IoSliceMut, ffi::OsStr, collections::BTreeMap};

use clap::Parser;
use proc_maps::get_process_maps;
use sysinfo::{ProcessExt, System, SystemExt, PidExt};
use nix::{sys::uio::{RemoteIoVec, process_vm_readv}, unistd::Pid};

const BUFFER_SIZE: usize = 16;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    process_name: String
}


fn read_bytes_from_process(pid: Pid, mut base_address: usize, page_size: usize) {
    for _ in (0..=page_size).step_by(BUFFER_SIZE) {
        let mut data = [0u8; BUFFER_SIZE];
        let local_iov = IoSliceMut::new(&mut data);
        let remote_iov = RemoteIoVec {
            base: base_address,
            len: BUFFER_SIZE,
        };
        let pvm = process_vm_readv(pid, &mut [local_iov], &[remote_iov]);
        match pvm {
            Ok(_) => println!("[0x{:08x}]", base_address),
            Err(err) => println!("{}", err)
        }
        for byte in data {
            print!("{:02x} ", byte);
        }
        println!("");
        base_address += BUFFER_SIZE;
    }
}

fn get_process_id_by_name(process_name: &str) -> u32 {
    let mut sys = System::new_all();
    sys.refresh_all();
    let process = sys.processes_by_exact_name(&process_name)
       .nth(0)
       .expect("could not find process");
    process.pid().as_u32()
}

fn get_virtual_memory_mapping(pid: u32, process_name: &str) -> BTreeMap<usize, usize> {
    let mut addresses = BTreeMap::new();
    let maps = get_process_maps(pid as i32).expect("cannot find virtual memory pages for process");
    for map in maps {
        match map.filename() {
            Some(val) => {
                if val.file_stem() == Some(OsStr::new(process_name)) {
                    addresses.insert(map.start(), map.size());
                }
            },
            None => () 
        }
    }
    addresses
}

fn main() {
    let args = Args::parse();
    let raw_pid = get_process_id_by_name(args.process_name.as_str());
    let virt_mem_maps = get_virtual_memory_mapping(raw_pid, args.process_name.as_str());
    let pid = Pid::from_raw(raw_pid as i32);
    for (memory_addr, page_size) in virt_mem_maps {
        read_bytes_from_process(pid, memory_addr, page_size);
    }
}
