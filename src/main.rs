use std::ffi::OsStr;
use std::os::windows::process::CommandExt;
use std::path::PathBuf;
use std::time::Duration;
use tokio::process::Command;
use lazy_static::lazy_static;
use sysinfo;

lazy_static! {
static ref PYTHON_DIR: PathBuf = std::env::current_dir().unwrap().join("bin");
static ref PYTHON_EXE: PathBuf = std::env::current_dir().unwrap().join("bin").join("python.exe");
static ref PYTHON_INSTALLER: PathBuf = std::env::current_dir().unwrap().join("assets").join("python-3.8.8-amd64.exe");
static ref PYTHON_URL: &'static str = "https://www.python.org/ftp/python/3.8.8/python-3.8.8-amd64.exe";
}


#[cfg(windows)]
fn main() {
    // println!("{:?}", get_python_executable_path());
    let rt = tokio::runtime::Builder::new_multi_thread().enable_all().build().unwrap();

    // rt.spawn(async {
    execute_command(
        PYTHON_EXE.as_path(),
        vec!["-m pip install --force-reinstall -r requirements.txt"],
    );
    // });

    rt.spawn(async {
        execute_command_async(
            PYTHON_EXE.as_path(),
            vec!["main.py"],
        ).await;
    });

    std::thread::sleep(Duration::from_secs(5));
    check_python_process();
    std::thread::sleep(Duration::from_secs(2));
    let count = kill_python_process();
    println!("[info] killed {} process", count);
    rt.shutdown_background();
}

fn check_python_process() -> () {
    let system = sysinfo::System::new_all();
    for (_, process) in system.processes() {
        if process.name().eq("python.exe") {
            let executable = &process.cmd()[0].clone().to_string();
            println!("[info] python process: {}", executable);
        }
    }
}

fn kill_python_process() -> usize {
    let mut count = 0;
    let system = sysinfo::System::new_all();
    for (pid, process) in system.processes() {
        if process.name().eq("python.exe") {
            let executable = &process.cmd()[0].clone().to_string();
            if executable.eq(PYTHON_EXE.as_path().to_str().unwrap()) {
                println!("[info] killed PID: {}", pid.as_u32());
                count += 1;
            }
        }
    }
    count
}

async fn execute_command_async<C, S>(cmd: C, args: Vec<S>) -> () where C: AsRef<OsStr>, S: AsRef<str> {
    let cmd = cmd.as_ref().to_str().unwrap().to_string();
    let args: Vec<String> = args.iter().map(|s| s.as_ref().to_string()).collect();
    let arg = args.join(" ");

    Command::new(cmd).raw_arg(arg).spawn().unwrap();
}

fn execute_command<C, S>(cmd: C, args: Vec<S>) -> () where C: AsRef<OsStr>, S: AsRef<str> {
    let cmd = cmd.as_ref().to_str().unwrap().to_string();
    let args: Vec<String> = args.iter().map(|s| s.as_ref().to_string()).collect();
    let arg = args.join(" ");

    std::process::Command::new(cmd).raw_arg(arg).spawn().unwrap().wait().unwrap();
}