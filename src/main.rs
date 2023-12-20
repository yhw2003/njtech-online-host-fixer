use std::fs;
use std::io;
use std::path::Path;
use std::io::Write;
use home::home_dir;
use std::path::PathBuf;
use sqlx;

static HOST_CONTEXT :&[u8]= "\n202.119.245.10 online.njtech.edu.cn".as_bytes();

#[cfg(target_os = "windows")]
fn host_handle() -> Result<(), io::Error>{
    println!("Editing host");
    let os_str: &'static str = std::env::consts::OS;
    println!("Please ensure your os is {os_str} [Y/n]");
    let input = std::io::stdin();
    let mut buf = String::new();
    input.read_line(&mut buf)?;
    let (i , _) = buf.split_at(1);
    if i != "y" && i != "Y" {
        panic!("Stopped")
    }
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
    println!("Successe, press enter to continue\n You can also try to clear your browser cookies and restart it");
    std::io::stdin().read_line(&mut String::new())?;
    Ok(())
}

#[cfg(target_os = "windows")]
async fn clear_cookie() -> Result<(), io::Error>{


    let prefix = home_dir().unwrap();
    let fire_fox_prefix = prefix.join("AppData\\Roaming\\Mozilla\\Firefox\\Profiles\\");
    let chrome_prefix = prefix.join("AppData\\Local\\Google\\Chrome\\User Data\\Default\\Network\\Cookies");
    // find cookie file 
    let mut base_path_firefox = Box::new(fire_fox_prefix.as_path());
    let base_path_chrome = Box::new(chrome_prefix.as_path());
    let bufpath = base_path_firefox.read_dir()?;
    let prc:Box<PathBuf>;
    for path in bufpath {
        match path {
            Ok(path) => {
                if path.path().join("cookies.sqlite").exists() {
                    prc = Box::new(path.path().join("cookies.sqlite"));
                    base_path_firefox = Box::new(Path::new(prc.to_str().unwrap()));
                    break;
                }
            }
            Err(_) => {continue;}
        }
        base_path_firefox = Box::new(Path::new("Not Found"));
    }
    println!("select your browser\n  1. Google Chrome (cookie at {})\n  2. FireFox (cookie at {})",
        base_path_chrome.to_str().unwrap(),
        base_path_firefox.to_str().unwrap()
    );
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let (input, _) = input.split_at(1);
    println!("Open sqlite file");
    if input == "1" {
        println!("Clearing cookies in Google Chrome of online.njtech.edu.cn");
        let db_uri = format!("sqlite://{}", base_path_chrome.to_str().unwrap());
        let pool = sqlx::SqlitePool::connect(&db_uri).await.unwrap();
        let _ = sqlx::query("
            DELETE FROM cookies WHERE name = 'mars_token'
        ").execute(&pool).await.map_err(|e| panic!("Sqlite Error: {:?}", e));
        return Ok(());
    };
    if input == "2" {
        println!("Clearing cookies in FireFox of online.njtech.edu.cn");
        println!("Open sqlite file");
        if input == "1" {
            println!("Clearing cookies in FireFox of online.njtech.edu.cn");
            let db_uri = format!("sqlite://{}", base_path_firefox.to_str().unwrap());
            let pool = sqlx::SqlitePool::connect(&db_uri).await.unwrap();
            let _ = sqlx::query("
                DELETE FROM moz_cookies WHERE name = 'mars_token'
            ").execute(&pool).await.map_err(|e| panic!("Sqlite Error: {:?}", e));
        return Ok(());
        };
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), io::Error>{
    println!("Add host[H] or clear cookies[C]? [H/C/A(both)/N]");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;
    let (input, _) = input.split_at(1);
    if input == "H" || input == "h" {
        host_handle()?;
        return Ok(());
    }
    if input == "C" || input == "c" {
        clear_cookie().await?;
        return Ok(());
    }
    if input == "A" || input == "a" {
        host_handle()?;
        clear_cookie().await?;
        return Ok(());
    }
    panic!("You inputed {input}, Stopped");
}