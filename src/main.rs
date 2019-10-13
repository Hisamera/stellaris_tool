#![allow(non_snake_case)]

use std::fs;
use std::io::{self, BufRead, /*BufReader*/};
use std::path;
use std::collections::HashMap;
use std::ffi;
mod Cfs;

// https://users.rust-lang.org/t/trim-string-in-place/15809
trait TrimInPlace { fn trim_in_place (self: &'_ mut Self); }
impl TrimInPlace for String {
    fn trim_in_place (self: &'_ mut Self) {
        let (start, len): (*const u8, usize) = {
            let self_trimmed: &str = self.trim();
            (self_trimmed.as_ptr(), self_trimmed.len())
        };
        unsafe {
            core::ptr::copy(
                start,
                self.as_bytes_mut().as_mut_ptr(), // no str::as_mut_ptr() in std ...
                len,
            );
        }
        self.truncate(len); // no String::set_len() in std ...
    }
}
pub struct Translation {
    origin: String,
    translation: String,
    line: u32,
    version: u16,
}

static ALLOWED_LANGUAGES: [&str; 8] = [
    "l_english",
    "l_braz_por",
    "l_german",
    "l_french",
    "l_spanish",
    "l_polish",
    "l_russian",
    "l_simp_chinese",
];

struct Argument {
    argument: &'static str,
    description: &'static str,
    possibilities: &'static str,
    requirements: &'static str,
    example: &'static str,
}

static ALLOWED_ARGUMENTS: [Argument; 5] = [
    Argument {
        argument: "-h",
        description: "Prints whole documentation (for specific information use -h [argument]), possible arguments.\
        \n  overall treat this as an API documentation. If this argument (or help, /h etc)\
        \n  is found anywhere in the list every other argument is disregarded.", 
        possibilities: "chainable: Yes with every argument; Can be used separately: Yes", 
        requirements: "requirements: nothing, or a supported argument", 
        example: "example: -h \
        \n  example: -h -a",
    },
    Argument {
        argument: "-g",
        description: "generate the folder structure that Stellaris the game utilizes",
        possibilities: "chainable: NO; Can be used separately: Yes", 
        requirements: "requirements: root folder has to point to a valid drive or directory \
        \n  It does not have to point to an existing folder or folder structure", 
        example: "example: folder C:\\testing exists but C:\\testing\\stellaris doesn't\
        \n  stellaris_localisation_tool -g \"C:\\testing\\stellaris\\newproject\"\
        \n  creates EVERY folder and subfolder mimicking Stellaris main folder structure.",
    },
    Argument {
        argument: "-a",
        description: "Scan every .yml file in folder and its subfolders for parsing errors and version checking", 
        possibilities: "chainable: Yes with -o; Can be used separately: Yes", 
        requirements: "requirements: root folder has to point to a valid drive and existing directory", 
        example: "example: folder C:\\testing\\localisation exists\
        \n  stellaris_localisation_tool -a \"C:\\testing\\localisation\"",
    },
    Argument {
        argument: "-o",
        description: "Overwrites default extension form .yml to whatever you like", 
        possibilities: "chainable: Yes with -a; Can be used separately: NO", 
        requirements: "requirements: -a argument has to be supplied, and it has to be before or after -a argument", 
        example: "example: folder C:\\testing\\localisation exists\
        \n  stellaris_localisation_tool -a \"C:\\testing\\localisation\" -o \"\"\
        \n  (only files with no extensions will be parsed. e.g. file but not file.e)",
    },
    Argument {
        argument: "-s",
        description: "Scan single file at given path for parsing errors and version checking", 
        possibilities: "chainable: NO; Can be used separately: Yes", 
        requirements: "requirements: file has to exist, file extension has to supplied (.yml, .txt or nothing)", 
        example: "example: file C:\\testing\\localisation\\test.yml exists\
        \n  stellaris_localisation_tool -s \"C:\\testing\\localisation\\test.yml\"\
        \n\n  example: file C:\\testing\\localisation\\test exists\
        \n  stellaris_localisation_tool -s \"C:\\testing\\localisation\\test\"", 
    },
];

struct KeyVersion {
    version: u16,
    language: u16,
}

fn real_main () -> i32 {
    let args : Vec<String> = std::env::args().collect();
    let mut help_found = false;

    println!("{:?}", args);

    for x in  0..args.len() {
        if args[x] == "-h"
        {
            if args.len() > x+1 {
                for y in 0..ALLOWED_ARGUMENTS.len() {
                    if args[x+1] == ALLOWED_ARGUMENTS[y].argument{
                        print!("  {}    {}\n\n  {}\n\n  {}\n\n  {}", ALLOWED_ARGUMENTS[y].argument, ALLOWED_ARGUMENTS[y].description, ALLOWED_ARGUMENTS[y].possibilities, ALLOWED_ARGUMENTS[y].requirements, ALLOWED_ARGUMENTS[y].example);
                        return 0;
                    }
                }
                println!("Supplied argument isn't supported.", );
                return 0;
            }
            help_found = true;
        }
        else if args[x] == "/h" || args[x] == "\\h" || args[x] == "help" {
            help_found = true;
            break;
        }
    }
    if args.len() == 1 || help_found{
        print!("  Everything that is printed here is subject to change prior to 1.0\n\
        \n  Also please remember that C:\\my awesome folder is not synonymus with\
        \n  \"C:\\my awesome folder\" (notice the quotes) first one will be parsed as\
        \n  [\"C:\\my\", \"awesome\", \"folder\"] while the second one as\
        \n  [\"C:\\my awesome folder\"]. All paths have to be absolute\
        \n  If you're new, try \"-h -h\"\n\
        \n  chainable: Means you can chain arguments so that they work differently together\
        \n  if you supply multiple arguments then program will 'consume' them until nothing is left\n\
        \n  List of possible arguments\
        \n  [nothing]\
        \n  /h\
        \n  \\h\
        \n  help\n");
        for x in 0..ALLOWED_ARGUMENTS.len() {
            print!("  {}    {}\n\n  {}\n\n  {}\n\n  {}\n-------------\n", ALLOWED_ARGUMENTS[x].argument, ALLOWED_ARGUMENTS[x].description, ALLOWED_ARGUMENTS[x].possibilities, ALLOWED_ARGUMENTS[x].requirements, ALLOWED_ARGUMENTS[x].example);
        }
        return 0;
    }
    let mut extension = ffi::OsStr::new("yml");
    let mut all_keys: HashMap<String, KeyVersion> = HashMap::new();
    let mut localisation_families: Vec<HashMap<String, Translation>> = std::vec::Vec::with_capacity(ALLOWED_LANGUAGES.len());
    for _ in 0..ALLOWED_LANGUAGES.len() {
        localisation_families.push(HashMap::new());
    }
    let mut folder_structure: std::vec::Vec<Cfs::Folder> = Vec::new();
    let mut errors: std::vec::Vec<String> = Vec::new();
    let mut x = 1;
    'outer: while x < args.len() {
        for y in 1..ALLOWED_ARGUMENTS.len() {
            if args[x] == ALLOWED_ARGUMENTS[y].argument {
                match y {
                    1 => { // -g
                        if x+1 == args.len() {
                            println!("Didn't supply a path, disregarding");
                            break 'outer;
                        }
                        let possible_path = args[x+1].replace("\"", "\\");
                        let path = path::Path::new(&possible_path);
                        if !path.is_absolute() {
                            println!("Supplied path isn't absolute, disregarding");
                            x=x+1;
                            continue 'outer;
                        }
                        if !(path.components().next().unwrap().as_ref() as &path::Path).exists() {
                            println!("Supplied path points to a drive that doesn't exist, disregarding");
                            x=x+1;
                            continue 'outer;
                        }
                        Cfs::populate_folder_structure(&mut folder_structure);
                        'main: loop {
                            Cfs::make_folder_structure(&folder_structure, &String::from(path.to_str().unwrap()), &mut errors);
                            if errors.len() > 0 {
                                println!("There were errors while trying to create folder structure:");
                                for error in errors.iter() {
                                    println!("{}",error);
                                }
                                loop {
                                    println!("Would you like to try agian or continue(not recomended)?");
                                    println!("T- Try again");
                                    println!("C- Continue");
                                    let mut response = String::new();
                                    io::stdin().read_line(&mut response).expect("Failed to read user response");
                                    if response == "T".to_owned() || response == "t".to_owned() {
                                        continue 'main;
                                    } else if response == "C".to_owned() || response == "c".to_owned() {
                                        break 'main;
                                    } else {
                                        println!("You didn't supply C or T, please try again.");
                                    }
                                }
                            } else {
                                break;
                            }
                        }
                        x=x+2;
                        continue 'outer;
                    }
                    2 => { // -a
                        if x+1 == args.len() {
                            println!("Didn't supply a path, disregarding");
                            break 'outer;
                        }
                        let possible_path = args[x+1].replace("\"", "\\");
                        let path = path::Path::new(&possible_path);
                        if !path.is_absolute() {
                            println!("Supplied path isn't absolute, disregarding");
                            x=x+1;
                            continue 'outer;
                        }
                        if !path.is_dir() {
                            println!("Supplied path doesn't point to a directory, disregarding");
                            x=x+1;
                            continue 'outer;
                        }
                        if args.len() > x+3 {
                            if args[x+2] == "-o" {
                                extension = ffi::OsStr::new(&args[x+3]);
                            }
                            x+=x+2;
                        }
                        scrape_localisations(path, &mut localisation_families, &mut all_keys, extension);
                        x+=2;
                        continue 'outer;
                    }
                    3 => { // -o
                        if x+1 == args.len() {
                            println!("Didn't supply a path, disregarding");
                            break 'outer;
                        } else {
                            extension = ffi::OsStr::new(&args[x+1]);
                        }
                        x=x+2;
                        continue 'outer;
                    }
                    4 => { // -s
                        if x+1 == args.len() {
                            println!("Didn't supply a path, disregarding");
                            break 'outer;
                        }
                        let possible_path = args[x+1].replace("\"", "\\");
                        let path = path::Path::new(&possible_path);
                        if !path.is_absolute() {
                            println!("Supplied path isn't absolute, disregarding");
                            x=x+1;
                            continue 'outer;
                        }
                        if !path.is_file() {
                            println!("Supplied path doesn't point to a file that exists, disregarding");
                            x=x+1;
                            continue 'outer;
                        }
                        parse_localisation(path, &mut localisation_families, &mut all_keys);
                        x+=2;
                        continue 'outer;
                    }
                    _ => {
                        print!("Logic error", );
                        return 1;
                    }
                }
            }
        }
        println!("{} is unsupported", args[x]);
        x+=1;
    }
    for (key, key_version) in all_keys.iter() {
        for x in 0..localisation_families.len() {
            if let Some(check) = localisation_families[x].get(&key.to_owned()) {
                if key_version.version > check.version {
                    println!("Key: {} in a {} family on line {} in file {} has a version {}, and thus needs retranslation, because same key in a {} family in file {} on line {}, has a version {}", 
                    key, 
                    ALLOWED_LANGUAGES[x], 
                    check.line, 
                    check.origin, 
                    check.version, 
                    ALLOWED_LANGUAGES[key_version.language as usize], 
                    localisation_families[key_version.language as usize].get(key).expect("This should not happen, logic error,").origin,
                    localisation_families[key_version.language as usize].get(key).expect("This should not happen, logic error").line, 
                    key_version.version);
                }
            }
        }
    }
    return 0;
}

fn main() { // I would like to call it config but I can't :C
    std::process::exit(match real_main() {
        0 => 0,
        _ => {
            eprintln!("Unhandled error code");
            1
        }
    })
}
fn parse_localisation(
    path: &path::Path, 
    localisation_families: &mut std::vec::Vec<HashMap<String, Translation>>, 
    all_keys: &mut HashMap<String, KeyVersion>,
    ) {
    println!("Now parsing: {:?}", path);
    let mut currentlanguage = 9999;
    let mut currenttranslation: Translation;
    let file = fs::OpenOptions::new()
        .read(true)
        .open(path).expect("openinig translation failed");
    let file = io::BufReader::new(file);
    let mut lines_iterator = file.lines();
    let mut line = lines_iterator.next().expect("File is empty").expect("There was an error reading a file");
    if line.len() < 3 {
        panic!{"File too short"};
    }
    if &line.as_bytes()[0..3] == [239, 187, 191] {
        line.drain(..3);
    } else {
        panic!("Localisation without UTF8 BOM");
    }
    let mut line_number: u32 = 1;
    'main: loop {
        let mut key: String;
        let mut value: String;
        let line_copy = line.clone();

        if let Some(collon) = line_copy.find(':') {
            let mut possiblenumber: String = "".to_owned();
            let pair = line_copy.split_at(collon);
            key = pair.0.to_owned();
            value = pair.1.to_owned();
            let mut not_found_start_of_key = true;
            for char in key.chars() {
                if !char.is_ascii() {
                    panic!("Key on line {} contains non-ascii characters", line_number);
                }
                if char == '#' && not_found_start_of_key {
                    line = match lines_iterator.next() {
                        Some(line) => {
                            line_number+=1;
                            line.expect("There was an error reading a file")
                        },
                        None => break 'main,
                    };
                    continue 'main;
                } else if char != ' ' {
                    not_found_start_of_key = false;
                }
            }
            key.trim_in_place();
            value.trim_in_place();
            
            if value.len() == 1 {
                let mut found = false;
                for (i, language) in ALLOWED_LANGUAGES.iter().enumerate() {
                    if &key == language {
                        found = true;
                        currentlanguage = i;
                        break;
                    }
                }
                if !found {
                    panic!("Language not supported");
                }
            } else {
                if currentlanguage == 9999 {
                    panic!{"Key-value pair was found before language key, on line {}", line_number};
                }
                else {
                    value.drain(0..1);
                    let mut begining: u32 = 0;
                    let hash_found = value.rfind('#').unwrap_or(0);
                    if let Some(_begining) = value.find('\"') {
                        begining = _begining as u32;
                    }
                    let mut end: u32 = 0;
                    if let Some(_end) = value.rfind('\"') {
                        end = _end as u32;
                    }
                    if let Some(at) = value.find(':') {
                        if ((at as u32) < begining || at as u32 > end) && at < hash_found {
                            panic!("colon found in wrong place at line: {}", line_number);
                        }
                    }
                    {
                        for (_, c) in value.char_indices().into_iter() {
                            if (c as u32 > 47) && ((c as u32) < 58) {
                                &possiblenumber.push(c);
                            }
                            else {
                                break;
                            }
                        }
                        value.drain(0..possiblenumber.len());
                        value.trim_in_place();
                        let mut first_quote = 0;
                        if let Some(at) = value.find('\"') 
                        {
                            if at != 0 {
                                panic!("Non-space, non-numeric character was found before first quote, this line will fail to parse. line number: {}\n parsed line: {}", line_number, line);
                            }
                            first_quote = at;
                        }
                        if let Some(at) = value.rfind('\"') 
                        {
                            if at == first_quote {
                                panic!("There is only one quote, this line will parse as nothing. Literally \"\". line number: {}\n parsed line: {}", line_number, line);
                            }
                            if hash_found > at {
                                println!("line {}", line_number);
                                let mut text_after_last_quote = value.get((at+1)..(hash_found-2)).expect("I don't know", ).to_owned();
                                text_after_last_quote.trim_in_place();
                                println!("{}, {}", text_after_last_quote, text_after_last_quote.len());
                                if text_after_last_quote.len() > 0 {
                                    panic!("Non-comment text was found after last quote, it will be disregarded(if it is a comment precede it with \"#\"). line number: {}\n parsed line: {}", line_number, line);
                                } 
                            }
                        }
                    }
                    let possiblenumber: u16 = match possiblenumber.parse() {
                        Ok(val) => val,
                        Err(_) => 0,
                    };
                    currenttranslation = Translation {origin: path.to_str().expect("Non UTF-8 os string").to_owned(), translation: value, version: possiblenumber, line: line_number};
                    if let Some(key_version) = all_keys.get(&key.to_owned()) {
                        if key_version.version < possiblenumber {
                            all_keys.insert(key.to_owned(), KeyVersion{version: possiblenumber, language: currentlanguage as u16});
                        }
                    } else {
                        all_keys.insert(key.to_owned(), KeyVersion{version: possiblenumber, language: currentlanguage as u16});
                    }
                    localisation_families[currentlanguage].insert(key.to_owned(), currenttranslation);
                }
            }
        }
        else {
            line.trim_in_place();
            if !line.len() == 0 {
                panic!("Line X is formated wrongly");
            }
        }
        line = match lines_iterator.next() {
            Some(line) => {
                line_number+=1;
                line.expect("There was an error reading a file")
            },
            None => break,
        }
    }
}
fn scrape_localisations (
    dir: &path::Path, 
    localisation_families: &mut std::vec::Vec<HashMap<String, Translation>>, 
    all_keys: &mut HashMap<String, KeyVersion>, 
    extension: &ffi::OsStr) {
    for entry in fs::read_dir(dir).expect("cokolwiek") {
        let path = entry.expect("coś").path();
        if path.is_dir() {
            scrape_localisations(&path, localisation_families, all_keys, extension)
        } else if let Some(ext) = path.extension() {
            if ext == extension {
                parse_localisation(&path, localisation_families, all_keys)
            }
        }
    }
}