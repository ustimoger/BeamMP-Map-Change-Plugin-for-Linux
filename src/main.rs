use std::env;
use std::fs::File;
use std::io::{Write, BufReader, BufRead, Error};
use std::process::Command;



fn main() {
let path = match env::current_dir(){
    Ok(exe_path) => 
    exe_path.display().to_string(),
Err(e) => panic!("Could not get Path"),

};


let arguments:[&str;4] =["help","trackselect","reboot","install"]; 
let mut tracks: Vec<String> = Vec::new();
init_tracks(& mut tracks, &path);
    let args: Vec<_> = env::args().collect();
    if args.len() > 1 {
let strng: &str = &args[1];
match strng {

"help" => println!("help: shows this menu, trackselect: use this to select a track, reboot: reboots the server, install: installs the given mod"),
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
    _ =>  println!("Valid arguments are: {:?} \n You entered: {}", arguments, &args[1]),

}

    }
    else{
        println!("Valid arguments are: {:?}",arguments );
    }

}



fn reboot(curr_path: &str){
    Command::new("screen")
        .arg("-XS")
        .arg("BeamMP")
        .arg("quit")
        .spawn()
        .expect("operation failed, program probably needs to be executed with sudo");

        Command::new("screen")
        .arg("-S")
        .arg("BeamMP")
        .arg("-d")
        .arg("-m")
        .arg("sh")
        .arg(format!("{curr_path}/BeamMPStart.sh")) //todo: change before uploading
        .spawn()
        .expect("Command Failed, idk why tho");
}

fn trackselect(ag: &str, tracks: Vec<String>, curr_path: &str) -> Result<(), Error>  {

    let path = &format!("{curr_path}/ServerConfig.toml"); 

  

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
    
    let paths = std::fs::read_dir(format!("{curr_path}/Resources/Client")).unwrap();

    for path in paths {
         for strng in zip_helper( &path.unwrap().path().display().to_string()){
            trackstor.push(strng);
         }
    }

}
fn install( link : &str, curr_path: &str ){

Command:: new("wget").arg("-P").arg(format!("{curr_path}/Resources/Client")).arg(link).spawn().expect("Something went Wrong with the download");



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


