use std::env;
use std::thread;
use std::time::Duration;
use std::process::Command;

fn getsong() -> String {
    let output = Command::new("sh").arg("-c").arg("mocp -Q %file | awk -F'/' '{print $NF}' | cut -d'[' -f1").output().expect("bad mocp");
    let outtext = String::from_utf8_lossy(&output.stdout); 
    return outtext.trim().to_string();
    //return outtext.trim().chars().filter(|&s| s.is_ascii()).collect::<String>();
}

fn getplay() -> bool {
    let output = Command::new("sh").arg("-c").arg("mocp -Q %state").output().expect("bad mocp");
    return String::from_utf8_lossy(&output.stdout) == "PAUSE\n";
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let limit = args.get(1).map_or(30, |arg| arg.parse().unwrap_or(30));
    let padlen = args.get(2).map_or(5, |arg| arg.parse().unwrap_or(5));
    let movems = args.get(3).map_or(300, |arg| arg.parse().unwrap_or(300));
    let holdms = args.get(4).map_or(1000, |arg| arg.parse().unwrap_or(1000));
    let revr = args.get(5).map_or(false, |arg| arg.to_lowercase() == "true");

    let mut text = getsong();
    let mut originaltext = text.clone();

    text.push_str(&" ".repeat(padlen));
    println!("{}",text);
    let mut elapsed = 0;
    
    loop {
        if !revr {
            text = format!("{}{}",text.chars().skip(1).collect::<String>(),text.chars().take(1).collect::<String>());
        } else {
            text = format!("{}{}",text.chars().skip(text.chars().count()-1).collect::<String>(),text.chars().take(text.chars().count()-1).collect::<String>());
        }
        if originaltext.chars().count() < limit {
            println!("{}", originaltext);
        } else {
            println!("{}", text.chars().take(limit).collect::<String>());
        }
        thread::sleep(Duration::from_millis(movems));
        elapsed += movems;
        if text.chars().take(originaltext.chars().count()).collect::<String>() == originaltext {
            thread::sleep(Duration::from_millis(holdms));
            elapsed += holdms;
            while getplay() {
                thread::sleep(Duration::from_millis(1000));
                elapsed += 1000;
            }
        }
        
        if elapsed >= 1000 {
            let cursong = getsong();
            if cursong != originaltext {
                text = cursong.clone();
                originaltext = text.clone();
                text.push_str(&" ".repeat(padlen))
            }
            elapsed = 0
        }
    }
}
