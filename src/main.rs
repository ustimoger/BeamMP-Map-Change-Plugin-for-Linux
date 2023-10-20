use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::process::Command;



fn main() {
let path = match env::current_dir(){
    Ok(exe_path) => 
    exe_path.display().to_string(),
Err(_) => panic!("Could not get Path"),

};


let arguments:[&str;6] =["help","trackselect","reboot","install", "init", "start"]; 
let mut tracks: Vec<String> = Vec::new();
init_tracks(& mut tracks, &path);
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
let strng: &str = &args[1];
match strng {

"help" => println!("help: shows this menu, trackselect: use this to select a track, reboot: reboots the server, install: installs the given mod, init: initializes the plugin and start skript(only run once), start: starts the server in screen session if off."),
"trackselect" => if args.len() == 2 {println!("Choose one of the following tracks: ");
let mut counter: u32 = 0; 
for strng in tracks{
    println!("{0}: {1}", counter, strng);
    counter +=1 ;
}

} else if args.len() == 3{ 
  trackselect(&args[2], tracks, &path).expect("Something went Wrong there Buckeroo.");
},
"reboot" => reboot(&path) ,
"install" => if args.len() < 3 {println!("Usage: install 'link here' ");} else if args.len() == 3{install(&args[2], &path);}, 
    "init" => init(&path),
    "start" => start_server(&path),
    _ =>  println!("Valid arguments are: {:?} \n You entered: {}", arguments, &args[1]),

}

    }
    else{
        println!("Valid arguments are: {:?}",arguments );
    }

}



fn reboot(curr_path: &str){
  let outp = Command::new("screen").arg("-ls").output().expect("screen -ls didn't work, terminating.");
 let kprocess = { 
    let mut retval:String = String::from("BeamMP"); 
    for line in String::from_utf8_lossy(&outp.stdout).lines(){
   if line.contains("BeamMP"){

retval = String::from(line);
retval = retval[0..retval.find("(").unwrap()].to_string();
retval = retval.replace("	", "");
break; 
   }

    }
  retval
 };
    Command::new("screen")
    .arg("-S")
    .arg("BeamMP")
    .arg("-d")
    .arg("-m")
    .arg("sh")
    .arg(format!("{}/BeamMPStart.sh", curr_path)) //todo: change before uploading
    .spawn()
    .expect("Command Failed, idk why tho");

    Command::new("screen")
        .arg("-XS")
        .arg(&kprocess)
        .arg("quit")
        .spawn()
        .expect("operation failed, program probably needs to be executed with sudo");

}

fn trackselect(ag: &str, tracks: Vec<String>, curr_path: &str) -> Result<(), Error>  {

    let path = &format!("{}/ServerConfig.toml", curr_path); 

  

    let input = File::open(path)?;
    let buffered = BufReader::new(input);
    let mut outstr: Vec<String> = Vec::new();
    let map = get_map(ag, tracks);
    for line in buffered.lines() {
      outstr.push(line.expect("OopsieWoopsie"));
    }
    let mut acout: String = String::from("");
    
    for strng in outstr{
        if strng.contains("Map =") {
acout = acout + "Map = '"+ &map+"'\n";  

        }else{
acout+=&strng; 
acout+= "\n";
        }
    }
    let mut output = File::create(path)?;
    write!(output,"{}", acout )?;
    reboot(curr_path);
    Ok(())
   
    
}

fn get_map(map_arg: &str, all_tracks: Vec<String> )-> String{

String::from("/levels/") + &String::from(match all_tracks.get(map_arg.parse::<usize>().expect("Please give a proper int value.")) {
Some(str) => str,
_  => "",
})+&String::from("/info.json")

}
fn init_tracks(trackstor: & mut Vec<String>,curr_path :&str ){
    
    let default_tracks: [String; 15] = [ String::from("automation_test_track"), String::from("cliff"), String::from("derby"), String::from("driver_training"), String::from("east_coast_usa"), String::from("gridmap_v2"), String::from("hirochi_raceway"), String::from("industrial"), String::from("italy"), String::from("johnson_valley"), String::from("jungle_rock_island"), String::from("small_island"), String::from("smallgrid"), String::from("utah"), String::from("west_coast_usa") ];
    for strng in default_tracks {
        trackstor.push(strng);

    }
    
    let paths = std::fs::read_dir(format!("{}/Resources/Client", curr_path)).unwrap();

    for path in paths {
         for strng in zip_helper( &path.unwrap().path().display().to_string()){
            trackstor.push(strng);
         }
    }

}
fn install( link : &str, curr_path: &str ){

Command:: new("wget").arg("-P").arg(format!("{}/Resources/Client", curr_path)).arg(link).spawn().expect("Something went Wrong with the download");



}
fn zip_helper(path: &str) -> Vec<String>{
let mut ret :Vec<String> = Vec::new();
let outputs = Command::new("unzip").arg("-l").arg(path).output().expect("Reading content of Unzip went wrong");
for line in String::from_utf8_lossy(&outputs.stdout).lines(){
if line.contains("levels/") {
    let mut start_bytes = line.find("levels/").unwrap_or(line.len()); //index where "pattern" starts
        if start_bytes != line.len() {
start_bytes += String::from("levels/").len();
        }                                             // or beginning of line if 
                                                     // "pattern" not found
let end_bytes = line[start_bytes..].find("/").unwrap_or(0); //index where "<" is found
                                                      // or end of line

let result = &line[start_bytes..end_bytes+start_bytes];
if result != ""&& !ret.contains(&String::from(result)){
    ret.push(String::from(result));
}

} 

}

ret
}
fn init(curr_path : &str){
    
let lua_script = {
    let path = &format!("{}/rawSkript", curr_path); 
let input = File::open(path).expect("error finding rawSkript file");
let buffered = BufReader::new(input);
let mut outstr: Vec<String> = Vec::new();
let mut inte : u8 = 1; 
for line in buffered.lines() {
    if  inte == 26{
        outstr.push(   format!( "local command = {0}screen -d -m {1}{2}",r#"""#, curr_path, r#"/BeamNGEdit {r}""#)); //extremely ugly but does the job , screen has to be terminated after usage 

    }else{
  outstr.push(line.expect("OopsieWoopsie"));}
inte +=1;
}
let mut acout: String = String::from("");
    
    for strng in outstr{
        
acout+=&strng; 
acout+= "\n";
        
    }

acout 
};
let pathstr = &format!("{}/Resources/Server/MapVotePlugin/main.lua",curr_path);
let path = std::path::Path::new(pathstr);
let prefix = path.parent().unwrap();
std::fs::create_dir_all(prefix).unwrap();
let mut write = File::create(path).expect("Couldn't create lua file"); //create dict for the plugin
write!(write, "{}", lua_script).expect("Writing to lua file went wrong");
let mut write = File::create(format!("{}/BeamMPStart.sh", curr_path)).expect("Couldn't create Start file.");
write!(write , "cd {} \n sleep 5\n ./BeamMP-for-your-distro-.22.04", curr_path).expect("Couldn't Write to StartSkript file");

}
fn start_server(curr_path: &str){
    Command::new("screen")
    .arg("-S")
    .arg("BeamMP")
    .arg("-d")
    .arg("-m")
    .arg("sh")
    .arg(format!("{}/BeamMPStart.sh", curr_path)) //todo: change before uploading
    .spawn()
    .expect("Command Failed, idk why tho");
}