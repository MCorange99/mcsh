use std::{
    io::{
        Write
    },
    env,
    path::{
        Path,
    },
    process::{
        Command
    }
};
use std::sync::mpsc::channel;
use ctrlc;
use whoami;
use chrono::{Utc};


// use std::io;
use substring::{Substring};

fn parse_input(inp: String) -> Vec<String> {
    let arr: Vec<&str> = inp.split_ascii_whitespace().collect::<Vec<&str>>();
    let arr: Vec<String> = arr.iter().map(|i| i.to_string()).collect();
    let mut out = Vec::new();

    let mut str_buff = String::new();
    let mut is_str = false;
    for arg in arr {

        if is_str {
            str_buff = str_buff + " " + arg.as_str();
            if arg.ends_with("\"") {
                is_str = false;
                out.push(str_buff.substring(1, str_buff.len() - 1).to_string());
                continue;
            }
        }

        if arg.starts_with("\"") {
            is_str = true;
            str_buff = arg;
            continue;    
        }

        if arg.starts_with("$") {
            out.push(env::var(arg.substring(1, arg.len())).unwrap());
            continue;    
        }

        out.push(arg);
    }
    out
}


fn find_program_path(prog: String) -> std::io::Result<String> {
    let env_path = env::var("PATH").unwrap_or(String::new());
    let paths = env_path.split(":");

    for path in paths {
        // println!("{}", path);
        let files = std::fs::read_dir(path);
        if files.is_err() {
            continue;
        }
        for file in files? {
            let fp = file?.path().display().to_string();
            if fp == (path.to_string() + "/" + prog.as_str()) {
                return Ok(fp);
            }
        }
    }


    Ok(String::new())
}

fn init() {
    env::set_var("LS_COLORS", "rs=0:di=01;34:ln=01;36:mh=00:pi=40;33:so=01;35:do=01;35:bd=40;33;01:cd=40;33;01:or=40;31;01:mi=00:su=37;41:sg=30;43:ca=00:tw=30;42:ow=34;42:st=37;44:ex=01;32:*.tar=01;31:*.tgz=01;31:*.arc=01;31:*.arj=01;31:*.taz=01;31:*.lha=01;31:*.lz4=01;31:*.lzh=01;31:*.lzma=01;31:*.tlz=01;31:*.txz=01;31:*.tzo=01;31:*.t7z=01;31:*.zip=01;31:*.z=01;31:*.dz=01;31:*.gz=01;31:*.lrz=01;31:*.lz=01;31:*.lzo=01;31:*.xz=01;31:*.zst=01;31:*.tzst=01;31:*.bz2=01;31:*.bz=01;31:*.tbz=01;31:*.tbz2=01;31:*.tz=01;31:*.deb=01;31:*.rpm=01;31:*.jar=01;31:*.war=01;31:*.ear=01;31:*.sar=01;31:*.rar=01;31:*.alz=01;31:*.ace=01;31:*.zoo=01;31:*.cpio=01;31:*.7z=01;31:*.rz=01;31:*.cab=01;31:*.wim=01;31:*.swm=01;31:*.dwm=01;31:*.esd=01;31:*.avif=01;35:*.jpg=01;35:*.jpeg=01;35:*.mjpg=01;35:*.mjpeg=01;35:*.gif=01;35:*.bmp=01;35:*.pbm=01;35:*.pgm=01;35:*.ppm=01;35:*.tga=01;35:*.xbm=01;35:*.xpm=01;35:*.tif=01;35:*.tiff=01;35:*.png=01;35:*.svg=01;35:*.svgz=01;35:*.mng=01;35:*.pcx=01;35:*.mov=01;35:*.mpg=01;35:*.mpeg=01;35:*.m2v=01;35:*.mkv=01;35:*.webm=01;35:*.webp=01;35:*.ogm=01;35:*.mp4=01;35:*.m4v=01;35:*.mp4v=01;35:*.vob=01;35:*.qt=01;35:*.nuv=01;35:*.wmv=01;35:*.asf=01;35:*.rm=01;35:*.rmvb=01;35:*.flc=01;35:*.avi=01;35:*.fli=01;35:*.flv=01;35:*.gl=01;35:*.dl=01;35:*.xcf=01;35:*.xwd=01;35:*.yuv=01;35:*.cgm=01;35:*.emf=01;35:*.ogv=01;35:*.ogx=01;35:*.aac=00;36:*.au=00;36:*.flac=00;36:*.m4a=00;36:*.mid=00;36:*.midi=00;36:*.mka=00;36:*.mp3=00;36:*.mpc=00;36:*.ogg=00;36:*.ra=00;36:*.wav=00;36:*.oga=00;36:*.opus=00;36:*.spx=00;36:*.xspf=00;36:*~=00;90:*#=00;90:*.bak=00;90:*.old=00;90:*.orig=00;90:*.part=00;90:*.rej=00;90:*.swp=00;90:*.tmp=00;90:*.dpkg-dist=00;90:*.dpkg-old=00;90:*.ucf-dist=00;90:*.ucf-new=00;90:*.ucf-old=00;90:*.rpmnew=00;90:*.rpmorig=00;90:*.rpmsave=00;90:")
}

fn build_prompt(p: String, exit_code: i32) -> String {
    let mut p = p.clone();
    p = p.replace("{user}", &env!("USER"));
    p = p.replace("{host}", &whoami::hostname());
    p = p.replace("{pwd}", &env::current_dir().unwrap().to_string_lossy().replace(env!("HOME"), "~"));
    p = p.replace("{exit_code}", format!("{}", exit_code).as_str());
    p = p.replace("{c_reset}",       "\u{001b}[0m");
    p = p.replace("{c_fg_black}",    "\u{001b}[30m");
    p = p.replace("{c_fg_red}",      "\u{001b}[31m");
    p = p.replace("{c_fg_green}",    "\u{001b}[32m");
    p = p.replace("{c_fg_yellow}",   "\u{001b}[33m");
    p = p.replace("{c_fg_blue}",     "\u{001b}[34m");
    p = p.replace("{c_fg_magenta}",  "\u{001b}[35m");
    p = p.replace("{c_fg_cyan}",     "\u{001b}[36m");
    p = p.replace("{c_fg_white}",    "\u{001b}[37m");
    p = p.replace("{c_fg_br_black}",    "\u{001b}[30;1m");
    p = p.replace("{c_fg_br_red}",      "\u{001b}[31;1m");
    p = p.replace("{c_fg_br_green}",    "\u{001b}[32;1m");
    p = p.replace("{c_fg_br_yellow}",   "\u{001b}[33;1m");
    p = p.replace("{c_fg_br_blue}",     "\u{001b}[34;1m");
    p = p.replace("{c_fg_br_magenta}",  "\u{001b}[35;1m");
    p = p.replace("{c_fg_br_cyan}",     "\u{001b}[36;1m");
    p = p.replace("{c_fg_br_white}",    "\u{001b}[37;1m");
    p = p.replace("{time_H}", &Utc::now().format("%H").to_string());
    p = p.replace("{time_M}", &Utc::now().format("%M").to_string());
    p = p.replace("{time_S}", &Utc::now().format("%S").to_string());
    p
}


fn main() -> std::io::Result<()> {
    init();
    let (tx, _rx) = channel();
    
    ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
        .expect("Error setting Ctrl-C handler");


    let mut user_input = String::new();
    let prompt: String = "\n{c_fg_br_green}{user}@{host} {c_fg_blue}[{time_H}:{time_M}:{time_S}]{c_reset} {c_fg_white}[{pwd}] {c_reset}[{exit_code}]\n{c_fg_cyan}-> % {c_reset}".into();
    let mut exit_code = 0;
    loop {
        // println!("{:?}", prompt);
        print!("{}", build_prompt(prompt.clone(), exit_code));
        std::io::stdout().flush()?;
        std::io::stdin().read_line(&mut user_input)?;
        let mut parsed = parse_input(user_input.clone());
        // println!("{:?}", parsed);
        user_input.clear();
        if parsed.len() == 0 {
            continue;
        }
        if parsed[0] == "exit" {
            println!("Exiting!");
            break;
        } else
        if parsed[0] == "cd" {
            let home_dir = env::var("HOME").unwrap();
            let inp = if parsed.len() < 2 {home_dir} else {parsed[1].as_str().to_string()};
            let pwd = env::var("PWD").unwrap();
            let new_pwd = match Path::new(pwd.as_str()).join(inp.as_str()).canonicalize() {
                Ok(p) => p,
                Err(e) => {
                    println!("{}", e);
                    continue;
                }
            };
            // println!("{}", new_pwd.as_os_str().to_string_lossy());
            // env::set_var("PWD", new_pwd.as_os_str().to_str().unwrap_or(&env::var("HOME").unwrap()));
            env::set_current_dir(new_pwd.as_os_str().to_str().unwrap_or(&env::var("HOME").unwrap()))?;
            continue;
        } else
        if parsed[0] == "export" {

        }


        let fpath = find_program_path(parsed[0].clone())?;

        let mut proc = match Command::new(fpath)
                                                        .args(&mut parsed[1..])
                                                        .spawn(){
                                                            Ok(p) => p,
                                                            Err(e) => {
                                                                println!("{}", e);
                                                                exit_code = 1;
                                                                continue;
                                                            }

                                                        };
        exit_code = proc.wait()?.code().unwrap();

        // println!("{}", fpath);
    }

    Ok(())
}
