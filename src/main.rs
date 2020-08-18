/*  Matteo Vinci - 2020
    Licensed under the GNU GPLv3 license
*/

use std::io;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::fs::{OpenOptions, DirEntry};
use std::fmt;

use finder::Finder;

use serde_derive::{Serialize, Deserialize};
use serde_json;

use colour::{yellow_ln, blue_ln, magenta_ln, cyan_ln, green_ln};

use quit;

const DATA_FOLDER: &str = "./data/";    //data directory(final slash is needed to make it works)
const FILE_XT: &str = ".json";  //file extension

//User interface input
//-----------------------------------------------------------------------------
fn user_input() -> u8 {
    println!
    (" -> 1) per REGISTRARE\n -> 2) per SCARICARE\n -> 3) per CARICARE\n -> 4) per MODIFICARE");
    println!(" -> 5) per VISIONARE\n -> 8) per RIMUOVERE\n -> 0) per USCIRE\n");
    let mut inp = String::new();
    io::stdin()
            .read_line(&mut inp).expect("Fallimento lettura input");
    if inp.trim().parse::<u8>().is_ok() {   //-> //Verifying input type
        let processed_input = inp.trim().parse::<u8>().unwrap();    //convert string to u8
        return processed_input;
    }
    else { return 101; }
}

//Defining JSON struct (implementing a formatted display method)
//-----------------------------------------------------------------------------
#[derive(Serialize, Deserialize)]
struct DataStruct {
        codice: String,
        testata: String,
        quantita: u16,
        nota: String,
}
impl fmt::Display for DataStruct {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n|- Codice:   {}\n\n|- Testata:   {}\n\n|- Quantità:  {}\n\n|- Nota:  {}\n",
        self.codice, self.testata, self.quantita, self.nota)
    }
}

//Add a JSON instance
//-----------------------------------------------------------------------------
fn registra() {
    //Compiling fields function
    fn c_input() -> (String, String, String, String) {
        //--CODICE
        println!("\nDigita il CODICE\nv\nv");
        let mut cod_in = String::new();
        io::stdin()
                .read_line(&mut cod_in).expect("Fallimento lettura input");
                let file_path = format!("{}{}{}", DATA_FOLDER, cod_in, FILE_XT);
                let j_path = Path::new(&file_path);
                let _display = j_path.display();
                //Search for already existing file (look at search function to check how input string is handled)
                let check: bool;
                match cerca_file(cod_in.to_string()) {
                    true => check = true ,
                    false => check = false ,
                }
                if check {
                    magenta_ln!("\n|-|- Esiste già! -|-|\n");
                    //std::process::exit(202);
                    quit::with_code(202);
                }
        //--TESTATA
        println!("\nDigita la TESTATA\nv\nv");
        let mut tes_in = String::new();
        io::stdin()
                .read_line(&mut tes_in).expect("Fallimento lettura input");
        //--QUANTITÀ
        println!("\nDigita la QUANTITÀ (NUMERI INTERI POSITIVI)\nv\nv");
        let mut quant_in = String::new();
        io::stdin()
                .read_line(&mut quant_in).expect("Fallimento lettura input");
        if !quant_in.trim().parse::<u16>().is_ok() {
            yellow_ln!("\nERRORE :> input non valido\n");
            //std::process::exit(101);
            quit::with_code(101);
        }
        //--NOTA
        println!("\nNota:");
        let mut nota_in = String::new();
        io::stdin()
                .read_line(&mut nota_in).expect("Fallimento lettura input");
    
        //v-v-v
        //output
        (cod_in.trim_end().to_string(), tes_in.trim_end().to_string(), quant_in.trim().to_string(), nota_in.trim_end().to_string())
    }

    let campi = c_input();
    let (c1, c2, c3, c4) = campi;
    
    let bolla = DataStruct {
        codice : c1.to_uppercase(),
        testata : c2,
        quantita : c3.parse::<u16>().unwrap(),
        nota : c4,
    };

    //Define JSON-file path (named after bolla.codice)
    let file_path = format!("{}{}{}", DATA_FOLDER, bolla.codice, FILE_XT);
    let j_path = Path::new(&file_path);
    let display = j_path.display();
    
    //Create and write JSON file
    let file = match OpenOptions::new().create(true).write(true).open(&j_path) {
        Err(why) => panic!("Impossibile creare e/o scrivere {}: {}", display, why) ,
        Ok(file) => file ,
        };
            
        match serde_json::to_writer_pretty(&file, &bolla) {
            Err(why) => panic!("Impossibile scrivere in {}: {}", display, why) ,
            Ok(_) => println!("\nScrittura con successo in {}\n", display) ,
        }
            
        //file.write_all(b"\n");    //Add line at the end
}

//Search for JSON files function
//-----------------------------------------------------------------------------
fn cerca_file(cod: String) -> bool {
    //Assembling the path of the searched object
    let s_cod = format!("{}{}{}", DATA_FOLDER, cod.trim_end().to_uppercase(), FILE_XT);
                                                //^trim is needed to remove '\n' in strings

    //Filter by extension (.json)
    fn json_filter(e: &DirEntry) -> bool {
        if let Some(n) = e.path().file_name() {
            let f_name = String::from(n.to_str().unwrap());
            if f_name.ends_with(FILE_XT) {
                return true;
            }
        }
        false
    }
    //Search between JSON files
    let mut found: bool = false;
    let json_finder = Finder::new(DATA_FOLDER);
    for i in json_finder.filter(&json_filter).into_iter() {
        if i.path().to_str().unwrap() == s_cod.to_string() {
            println!("\nTrovato   --->   | {} |", cod.trim_end().to_uppercase());
            found = true;
            break;
        }
    }
    if !found {
        blue_ln!("\n{} >>> Non esiste ancora\n", cod.trim_end().to_uppercase());
    }
    
    return found;
}

fn carico_scarico(m: bool) {
    println!("Digita il Codice:\nv\nv");
    let mut load_cod = String::new();
    io::stdin().read_line(&mut load_cod).expect("Fallimento lettura input");
    let load_path: String;
    let cod_copy = &load_cod;   //needed to avoid load_code move
    //Searching the matching file (look at search function to check how input string is handled)
    let check: bool;
    match cerca_file(cod_copy.to_string()) {
        true => check = true ,
        false => check = false ,
    }
    if check {
        //Open file (without truncate it for now)
        load_path = format!("{}{}{}", DATA_FOLDER, load_cod.trim_end().to_uppercase(), FILE_XT);
        let j_path = Path::new(&load_path);
        let display = j_path.display();
        let file = match OpenOptions::new().create(false).read(true).write(true).open(&j_path) {
            Err(why) => panic!("Impossibile aprire e/o scrivere {}: {}", display, why) ,
            Ok(file) => file ,
        };
        
        //Copy JSON content into buffer reader
        let mut r_buffer = BufReader:: new(&file);
        let mut content = String:: new();
        match r_buffer.read_to_string(&mut content) {
            Err(why) => panic!("Impossibile scrivere da buffer {}: {}", display, why) ,
            Ok(file) => file ,
            };
        
        //Build a DataStruct from string content
        let mut data: DataStruct = serde_json::from_str(&content).unwrap();

        //Asking for amount
        println!("\nQUANTITÀ ATTUALE : {}\n", data.quantita);
        println!("\nDigitare quantità carico/scarico (solo numeri INTERI POSITIVI):\nv");
        let mut amount = String::new();
        io::stdin().read_line(&mut amount).expect("Fallimento lettura input");
        if amount.trim().to_string().parse::<u16>().is_ok()
        && amount.trim().to_string().parse::<u16>().unwrap() > 0 {  //-> //Avoid wrong digit
            
            //Reopen the file to truncate it and read from json structure to update the value (quantità)
            let file = match OpenOptions::new().create(false).read(true).truncate(true).write(true).open(&j_path) {
                Err(why) => panic!("Impossibile creare e/o scrivere {}: {}", display, why) ,
                Ok(file) => file ,
            };

            //Update quantity by amount
            if m {
                //(add)
                data.quantita += amount.trim().parse::<u16>().unwrap();    //converting input string to u16
            }
            else {
                //(subtract)
                if data.quantita > amount.trim().parse::<u16>().unwrap() {
                    data.quantita -= amount.trim().parse::<u16>().unwrap();
                }
                else {
                    yellow_ln!("ERRORE: _> Quantità inserita maggiore di quella presente\n");
                    //std::process::exit(101);
                    quit::with_code(101);
                }
            }
            //Rewrite
            match serde_json::to_writer_pretty(&file, &data) {
                Err(why) => panic!("Impossibile scrivere in {}: {}", display, why) ,
                Ok(_) => println!("\nCarico aggiornato <<< {}\n", display) ,
            }
        }   //End of "if amount > 0"

        else { yellow_ln!("\nATTENZIONE :> digitazione non corretta\n") };

    }   //End of check
}

fn stampa() {
    println!("\nDigita codice da stampare a video\nv");
    let mut stmp_cod = String::new();
    io::stdin()
        .read_line(&mut stmp_cod).expect("\nFallimento lettura input\n");
    //Searching the matching file (look at search function to check how input string is handled)
    let check: bool;
    match cerca_file(stmp_cod.to_string()) {
        true => check = true ,
        false => check = false ,
    }
    if check {
        //Open file for reading only
        let stmp_path: String;
        stmp_path = format!("{}{}{}", DATA_FOLDER, stmp_cod.trim_end().to_uppercase(), FILE_XT);
        let j_path = Path::new(&stmp_path);
        let display = j_path.display();
        let file = match OpenOptions::new().create(false).read(true).write(false).open(&j_path) {
            Err(why) => panic!("Impossibile aprire e/o scrivere {}: {}", display, why) ,
            Ok(file) => file ,
        } ;
        //Copy JSON file content into buffer reader
        let mut r_buffer = BufReader:: new(&file);
        let mut content = String:: new();
        match r_buffer.read_to_string(&mut content) {
            Err(why) => panic!("Impossibile scrivere da buffer {}: {}", display, why) ,
            Ok(file) => file ,
            };
        
        //Build a DataStruct from string content
        let data: DataStruct = serde_json::from_str(&content).unwrap();
        cyan_ln!("{}", data);
    }
}

//Modify JSON file
//-----------------------------------------------------------------------------
fn modifica() {
    println!("\nDigita il Codice da modificare\nv\nv");
    let mut mod_cod = String::new();
        io::stdin().read_line(&mut mod_cod).expect("Fallimento lettura input");
        //Asking for confirmation
        println!("\n*_ATTENZIONE_* <:> Modifica di {} permanente | Continuare?..(s/n)",
            mod_cod.trim_end().to_uppercase());
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).unwrap();
        if choice.trim_end() == "s".to_string() || choice.trim_end() == "S".to_string() {
            //Search for already existing file (look at search function to check how input string is handled)
            let check: bool;
            match cerca_file(mod_cod.to_string()) {
                true => check = true ,
                false => check = false ,
            }
            if check {
                //Open file (without truncate it for now)
                let mod_path = format!("{}{}{}", DATA_FOLDER, mod_cod.trim_end().to_uppercase(), FILE_XT);
                let j_path = Path::new(&mod_path);
                let display = j_path.display();
                let file = match OpenOptions::new().create(false).read(true).write(true).open(&j_path) {
                    Err(why) => panic!("Impossibile aprire e/o scrivere {}: {}", display, why) ,
                    Ok(file) => file ,
                };
                
                //Copy JSON content into buffer reader
                let mut r_buffer = BufReader:: new(&file);
                let mut content = String:: new();
                match r_buffer.read_to_string(&mut content) {
                    Err(why) => panic!("Impossibile scrivere da buffer {}: {}", display, why) ,
                    Ok(file) => file ,
                    };
                
                //Build a DataStruct from string content
                let mut data: DataStruct = serde_json::from_str(&content).unwrap();

                //Modify DataStruct attributes step by step (Code excluded)
                //--TESTATA
                println!("Testata attuale >: {}\n", data.testata);
                println!("|v\nDigita nuova Testata :> ");
                let mut new_tes = String::new();
                io::stdin().read_line(&mut new_tes).expect("Fallimento lettura input");
                data.testata = new_tes.trim_end().to_string();
                //--QUANTITÀ
                println!("Quantità attuale >: {}\n", data.quantita);
                println!("|v\nDigita nuova Quantità :> ");
                let mut new_quant = String::new();
                io::stdin().read_line(&mut new_quant).expect("Fallimento lettura input");
                if !new_quant.trim().to_string().parse::<u16>().is_ok()
                || new_quant.trim().to_string().parse::<u16>().unwrap() <= 0 {  //-> //Avoid wrong digit
                    yellow_ln!("\nATTENZIONE :> digitazione quantità non corretta\n");
                    //std::process::exit(101);
                    quit::with_code(101);
                }
                data.quantita = new_quant.trim().parse::<u16>().unwrap();   //from string to u16
                //--NOTA
                println!("Nota attuale >: {}\n", data.nota);
                println!("|v\nDigita nuova Nota :> ");
                let mut new_nota = String::new();
                io::stdin().read_line(&mut new_nota).expect("Fallimento lettura input");
                data.nota = new_nota.trim_end().to_string();

                //Overwrite
                //Reopen the file to truncate it and read from json structure to update the value (quantità)
                let file = match OpenOptions::new().create(false).read(true).truncate(true).write(true).open(&j_path) {
                    Err(why) => panic!("Impossibile creare e/o scrivere {}: {}", display, why) ,
                    Ok(file) => file ,
                };
                match serde_json::to_writer_pretty(&file, &data) {
                    Err(why) => panic!("Impossibile scrivere in {}: {}", display, why) ,
                    Ok(_) => println!("\nIstanza modificata <<< {}\n", display) ,
                }
            }
        }
        //If choice was !s ...> cancel
        else {
            blue_ln!("\nModifica annullata\n");
            //std::process::exit(101);
            quit::with_code(101);
        }
}

//Remove a JSON instance
//-----------------------------------------------------------------------------
fn rimuovi() {
    println!("\nDigita il Codice da rimuovere\nv\nv\nv");
    let mut cod_rm = String::new();
    io::stdin()
        .read_line(&mut cod_rm).expect("\nFallimento lettura input\n");
    //Searching the matching file (look at search function to check how input string is handled)
    let check: bool;
    match cerca_file(cod_rm.to_string()) {
        true => check = true ,
        false => check = false ,
    }
    if check {
        let file_path = format!("{}{}{}", DATA_FOLDER, cod_rm.trim_end().to_uppercase(), FILE_XT);
        let j_path = Path::new(&file_path);
        let display = j_path.display();
        
        //Delete file
        match std::fs::remove_file(j_path) {
                Err(why) => panic!("\nImpossibile eliminare {}: {}\n", display, why) ,
                Ok(_) => println!("\n{} Eliminato\n", display) ,
            }
        }
}


fn run() {
    let mut loop_quit: bool = false;
    loop {
        match user_input() {
            0 => loop_quit = true ,
            1 => registra() ,
            2 => carico_scarico(false) , //scarico (-)
            3 => carico_scarico(true) , //carico (+)
            4 => modifica() ,
            5 => stampa() ,
            8 => rimuovi() ,
            101 => yellow_ln!("\nERRORE: digitare un'opzione valida\n") ,
            _ => yellow_ln!("\nERRORE: digitare un'opzione valida\n") ,
        }
        if loop_quit {
            println!("|\n--Chiusura!\n");
            blue_ln!("Premi Invio per uscire...");
            io::stdin().read(&mut [0]).unwrap();
            break;
        }
    }
}

fn main() {
    println!("\nCopyright (C) 2020 - Matteo Vinci");
    println!(r#"This program comes with ABSOLUTELY NO WARRANTY.
This is free software, and you are welcome to redistribute it under certain conditions.
GNU General Public License: https://www.gnu.org/licenses/gpl-3.0.html"#);
    green_ln!("\n  -*Benvenuto!*-\n");

    run();
    
}