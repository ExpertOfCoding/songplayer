use std::sync::Mutex;
use tauri::State;
use std::io::BufReader;
use std::fs::File;
use std::fs;
use std::io::Read;
use std::{io, process};
use rodio::{Decoder, OutputStream, Sink};
use rodio::source::Source;
use std::io::Write;
// Define your song states
#[derive(Debug, Clone, Copy, PartialEq)]
enum SongState {
    Waiting,
    Playing,
    Paused,
    Error,
}

struct CurrentSong{
    sink: Mutex<Sink>,
    title: Mutex<String>,
}

// Define a struct to hold your state
struct AppState {
    song_state: Mutex<SongState>,
    songs : Mutex<Vec<String>>,
    current_index : Mutex<i32>,
    folder : Mutex<String>,
}

fn get_image_files() -> Vec<String> {
    let paths = fs::read_dir("../src/images/").unwrap();
    let mut pathnames = vec![];

    for entry in paths {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                let ext = ext.to_string_lossy().to_lowercase();
                if ext == "png" || ext == "jpg" || ext == "jpeg" || ext == "gif" || ext == "bmp" || ext == "webp" {
                    if let Some(filename) = path.file_name() {
                        let filename_str = filename.to_string_lossy().into_owned();
                        println!("Image File: {}", filename_str);
                        pathnames.push(filename_str);
                    }
                }
            }
        }
    }

    pathnames
}


fn get_audio_files() -> Vec<String> {
    let mut file = File::open("C:/cutyplay/folder.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let paths = fs::read_dir(contents.as_str()).unwrap();
    let mut pathnames = vec![];

    for entry in paths {
        if let Ok(entry) = entry {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if ext == "mp3" {
                    if let Some(filename) = path.file_name() {
                        let filename_str = filename.to_string_lossy().into_owned();
                        println!("MP3 File: {}", filename_str);
                        pathnames.push(filename_str);
                    }
                }
            }
        }
    }
    pathnames
}



#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn refresh_songs(state: &State<AppState>) {
    let mut songs = state.songs.lock().unwrap();
    *songs = get_audio_files();
}

#[tauri::command]
fn refresh(state: State<AppState>, _sink_state: State<CurrentSong>,filename : String) {
    refresh_songs(&state);
    write_folder_name(filename,&state);
}

fn load_file(filename : &str) -> Decoder<BufReader<File>>{
    let mut file = File::open("C:/cutyplay/folder.txt").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    let file = match File::open(format!("{}{}",contents,filename).as_str()) {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Dosya açılamadı '{}': {}", "./audio.mp3", e);
            process::exit(1);
        }
    };

    let source = match Decoder::new(BufReader::new(file)) {
        Ok(source) => source,
        Err(e) => {
            eprintln!("Ses dosyası decode edilemedi: {}", e);
            process::exit(1);
        }
    };
    source
    
}


#[tauri::command]
fn play(state: State<AppState>, sink_state : State<CurrentSong>){
    refresh_songs(&state); // buraya ekledik
    let mut song_state = state.song_state.lock().unwrap();
    let songs = state.songs.lock().unwrap();
    let index = state.current_index.lock().unwrap();
    let mut sink = sink_state.sink.lock().unwrap();
    let mut title = sink_state.title.lock().unwrap();   
    if *song_state == SongState::Waiting{
        let source = load_file(songs[*index as usize].as_str());
        sink.append(source);
        *title =((*songs)[*index as usize]).to_string();
        *song_state = SongState::Playing;
    }
    else if *song_state == SongState::Paused{
        sink.play();
        *song_state = SongState::Playing;
    }

}

#[tauri::command]
fn next(state: State<AppState>, sink_state : State<CurrentSong>){
    refresh_songs(&state); // buraya ekledik
    let songs = state.songs.lock().unwrap();
    let mut index = state.current_index.lock().unwrap();
    let mut song_state = state.song_state.lock().unwrap();
    let mut sink = sink_state.sink.lock().unwrap();
    let mut title = sink_state.title.lock().unwrap();
    if *index as usize >= songs.len() - 1 {
        *index = 0;
    }else{
        *index += 1;
    }
    let source = load_file(&(songs[*index as usize]));
    sink.clear();
    sink.append(source);
    sink.play();
    *title =((*songs)[*index as usize]).to_string();
    *song_state = SongState::Playing;
}

fn writetofile(file_name : &String)  -> std::io::Result<()>  {
    fs::write("C:/cutyplay/folder.txt", file_name.as_bytes())?;
    Ok(())
}


fn write_folder_name(filename : String, state: &State<AppState>) {
    let mut folder_name = state.folder.lock().unwrap();
    let _ = check_existence();
    let _ = writetofile(&filename);
    *folder_name = filename;
}


#[tauri::command]
fn prev(state: State<AppState>, sink_state : State<CurrentSong>){
    refresh_songs(&state); // buraya ekledik
    let songs = state.songs.lock().unwrap();
    let mut index = state.current_index.lock().unwrap();
    let mut song_state = state.song_state.lock().unwrap();
    let mut sink = sink_state.sink.lock().unwrap();
    let mut title = sink_state.title.lock().unwrap();
    if *index as usize == 0 {
        *index = (songs.len() - 1) as i32;
    }else{
        *index -= 1;
    }
    let source = load_file(&(songs[*index as usize]));
    sink.clear();
    sink.append(source);
    sink.play();
    *title =((*songs)[*index as usize]).to_string();
    *song_state = SongState::Playing;
}



#[tauri::command]
fn isempty(state: State<AppState>, sink_state : State<CurrentSong>) -> bool{
    let mut song_state = state.song_state.lock().unwrap();
    let mut sink = sink_state.sink.lock().unwrap();
    let is_playing = sink.empty();
    if is_playing{
        *song_state = SongState::Waiting;
    }
    is_playing
}

#[tauri::command]
fn pause(state: State<AppState>, sink_state : State<CurrentSong>) {
    let mut song_state = state.song_state.lock().unwrap();
    let mut sink = sink_state.sink.lock().unwrap();
    if *song_state == SongState::Playing {
        sink.pause();
        *song_state = SongState::Paused;
    }
}

#[tauri::command]
fn get_song_state(state: State<AppState>,song_name_state : State<CurrentSong>) -> (String,String,String,String,Vec<String>) {
    let song_state = state.song_state.lock().unwrap();
    let folder = state.folder.lock().unwrap();
    let songs = state.songs.lock().unwrap();
    let song_name = song_name_state.title.lock().unwrap();
    (format!("{:?}",*song_state),format!("{:?}",*song_name),format!("{:?}",*songs),format!("{:?}",*folder),get_image_files())
}

fn check_existence() -> Result<(), io::Error> {
    if fs::exists("C:/cutyplay/folder.txt").expect("no") {
        println!("it exists");
        Ok(())
    } else {
        let _file = File::create("C:/cutyplay/folder.txt")?;
        println!("it did not exist");  
        Ok(())
    }
}

fn read_from_folder() -> String{
    if fs::exists("C:/cutyplay/folder.txt").expect("no") {
        println!("it exists");
        let mut file = File::open("C:/cutyplay/folder.txt").unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        println!("{:?}",contents);
        if contents.is_empty() {
            fs::write("C:/cutyplay/folder.txt",b"C:/cutyplay/songs/").unwrap();
            return "C:/cutyplay/songs/".to_string();
        }
        contents
    } else {
        let mut xfile = File::create("C:/cutyplay/folder.txt").unwrap();
        xfile.write_all(b"C:/cutyplay/songs/").unwrap();
        println!("it did not exist");  
        println!("C:/cutyplay/songs/");
        "C:/cutyplay/songs/".to_string()
    }
}


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = read_from_folder();
    let (_stream, stream_handle) = OutputStream::try_default().unwrap_or_else(|err| {
        eprintln!("Ses çıkışı başlatılamadı: {}", err);
        process::exit(1);
    });
    let sink = Sink::try_new(&stream_handle).unwrap_or_else(|err| {
        eprintln!("Ses sink'i oluşturulamadı: {}", err);
        process::exit(1);
    });
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Register the shared state
        .manage(AppState {
            song_state: Mutex::new(SongState::Waiting),
            songs:Mutex::new(get_audio_files()),
            current_index:Mutex::new(0),
            folder:Mutex::new(read_from_folder()),
        }).manage(
            CurrentSong{
                sink: Mutex::new(sink),
                title: Mutex::new("".to_string()),
            }
        )
        // Register all command handlers
        .invoke_handler(tauri::generate_handler![
            greet,
            play,
            pause,
            next,
            prev,
            refresh,
            isempty,
            get_song_state
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}