/*



CODE IN GO:



package main

import (
	"fmt"
	"os"
	"os/exec"
	"syscall"
)

func main() {
	switch os.Args[1] {
	case "run":
		parent()
	case "child":
		child()
	default:
		panic("wat should I do")
	}
}

func parent() {
	cmd := exec.Command("/proc/self/exe", append([]string{"child"}, os.Args[2:]...)...)
	cmd.SysProcAttr = &syscall.SysProcAttr{
		Cloneflags: syscall.CLONE_NEWUTS | syscall.CLONE_NEWPID | syscall.CLONE_NEWNS,
	}
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		fmt.Println("ERROR", err)
		os.Exit(1)
	}
}

func child() {
	must(syscall.Mount("rootfs", "rootfs", "", syscall.MS_BIND, ""))
	must(os.MkdirAll("rootfs/oldrootfs", 0700))
	must(syscall.PivotRoot("rootfs", "rootfs/oldrootfs"))
	must(os.Chdir("/"))

	cmd := exec.Command(os.Args[2], os.Args[3:]...)
	cmd.Stdin = os.Stdin
	cmd.Stdout = os.Stdout
	cmd.Stderr = os.Stderr

	if err := cmd.Run(); err != nil {
		fmt.Println("ERROR", err)
		os.Exit(1)
	}
}

func must(err error) {
	if err != nil {
		panic(err)
	}
}

*/



use {
    std::{
        process::{
            Command,
            Stdio
        }
    }
};

/*
#[proc_macro_attribute]
pub fn ENV_ARGS(attr: TokenStream, item: TokenStream) -> TokenStream {

    //where attr: matches the provided args in #[ENV_ARGS()]
    //
    //and
    //
    //item matches the function that gets called like this:
    //
    // #[ENV_ARGS]
    // #my_function(3);
    
    // how to print:    println!("item: \"{}\"", item.to_string());

    item
}
*/


// TODO determine by argv[0] what called
// be able to detach/interact with container when not BitCommit
//
// BitCommit        (commit-container)
// #########        ##################
//      |                   |
//      |--------------------  <--- Symlink
//      |
//  call library



pub fn run() {
    Command::new("/proc/self/exe")
        .env("Cloneflags", "syscall.CLONE_NEWUTS | syscall.CLONE_NEWPID | syscall.CLONE_NEWNS,")
// implement this in furure        .cuurent_dir("")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .arg("child")
        .arg(std::env::args().nth(1).expect("could not use argv[1]"))
        .arg(std::env::args().nth(2).expect("could not use argv[2]"));
}


