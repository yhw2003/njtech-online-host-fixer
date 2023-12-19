use std::fs;
use std::io;
use std::path::Path;
use std::io::Write;

static HOST_CONTEXT :&[u8]= "\n202.119.245.10 online.njtech.edu.cn".as_bytes();

#[cfg(target_os = "windows")]
fn handle() -> Result<(), io::Error>{
    //Backup host files
    let host_path = Path::new("C:\\Windows\\System32\\drivers\\etc\\hosts");
    println!("Backing up os old host file from {} to C:\\host_old",host_path.to_str().unwrap());
    let backup_path = Path::new("C:\\host_old");
    if backup_path.exists() {
        println!("Warning: your host file won't be backed up, maybe host backup is existed")
    } else {
        fs::copy(host_path, backup_path)?;
    }
    println!("Try to open host file");
    let mut host_fd = fs::OpenOptions::new().append(true).open(host_path)?;
    println!("Writing new lines");
    host_fd.write(HOST_CONTEXT)?;
    println!("Successe, press any key to continue\n You can also try to clear your browser cookies");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}
fn main() -> Result<(), io::Error>{
    let os_str: &'static str = std::env::consts::OS;
    println!("Please ensure your os is {os_str} [Y/n]");
    let input = std::io::stdin();
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    let (i , _) = buf.split_at(1);
    if i != "y" && i != "Y" {
        panic!("Stopped")
    }
    handle()
}




