const { invoke } = window.__TAURI__.core;
const { appWindow } = window.__TAURI__.window;
let greetInputEl;
let greetMsgEl;
let button;
let folder_input = document.querySelector(".input-decoration")

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: greetInputEl.value });
}

async function changeTheme(themeName) {
  const player = document.getElementById('player');
  
  // Remove all theme classes first
  player.classList.remove('theme-blue', 'theme-purple', 'theme-green');
  
  // Add the selected theme class
  if (themeName) {
      player.classList.add(themeName);
  }
}

let themes  = ['','theme-blue','theme-purple','theme-green']
// This function could be called when a new song is loaded
async function updateThemeBasedOnSong(songMood) {
  // Example: change theme based on song mood
  switch(songMood) {
      case 'happy':
          changeTheme(''); // Pink theme (default)
          break;
      case 'calm':
          changeTheme('theme-blue');
          break;
      case 'mysterious':
          changeTheme('theme-purple');
          break;
      case 'energetic':
          changeTheme('theme-green');
          break;
      default:
          changeTheme(''); // Default to pink theme
  }
}

// You could also create a function to generate a theme from album artwork
async function generateThemeFromArtwork(imageElement) {

  // This is a placeholder for color extraction logic
  // In a real implementation, you'd use a library like Vibrant.js
  // to extract colors from the album artwork
  
  // For demonstration, we'll just randomly select a theme
  const themes = ['', 'theme-blue', 'theme-purple', 'theme-green'];
  const randomTheme = themes[Math.floor(Math.random() * themes.length)];
  changeTheme(randomTheme);
}

async function handleSongChangeW() {

  let title = document.getElementById("title")
  let play = document.getElementById("play") 
  let r = await invoke("get_song_state")
  if (r[0]==="Playing"){
    play.innerHTML="PAUSE"
  } else{
    play.innerHTML="PLAY"
  }
  console.log(r);
  let rr = r[1]?.slice(1)
  title.innerHTML = rr.slice(0,rr.length-1);
  let xr = r[3]?.slice(1);
  folder_input.value = xr.slice(0,xr.length-1);
}


async function handleSongChange() {

  let title = document.getElementById("title")
  let play = document.getElementById("play") 
  let r = await invoke("get_song_state")
  if (r[0]==="Playing"){
    play.innerHTML="PAUSE"
  } else{
    play.innerHTML="PLAY"
  }
  console.log(r);
  let rr = r[1]?.slice(1)
  title.innerHTML = rr.slice(0,rr.length-1);
  let xr = r[3]?.slice(1);
  folder_input.value = xr.slice(0,xr.length-1);
  let array = r[4];
  let random = Math.floor(Math.random()*array.length)
  let image = document.getElementById("image")
  image.src = "images/"+array[random];
  changeTheme(themes[Math.floor(Math.random()*themes.length)]);
}

window.addEventListener("DOMContentLoaded", () => {
  handleSongChange();
  let refresh_button = document.querySelector(".refresh-button")
  refresh_button.addEventListener("click",async() =>{
    console.log("clicked")
    await invoke("refresh",{filename:folder_input.value})
  })
  
  document.querySelectorAll('.theme-button').forEach(button => {
  button.addEventListener('click', () => {
    console.log(button.classList[1]);
    let themeName = (button.classList[1]).split("-")[1] +"-"+ (button.classList[1]).split("-")[0];
    changeTheme(themeName);
  });
});
let play = document.getElementById("play") 
  play.addEventListener("click",async()=>{
    if (play.innerHTML === "PLAY"){
      await invoke("play");
      play.innerHTML="PAUSE"
    } else{
      await invoke("pause");
      play.innerHTML="PLAY"
    }
  handleSongChangeW();
  })
  let prev = document.getElementById("prev")
  prev.addEventListener("click",async()=>{
    console.log("next pressed")
  await invoke("prev");
  play.innerHTML="PAUSE";
  handleSongChange();
  })


let next = document.getElementById("next")
  next.addEventListener("click",async()=>{
    console.log("next pressed")
  await invoke("next");
  play.innerHTML="PAUSE";
  handleSongChange();
  })

  let interval = setInterval(async() => {
    let isempty = await invoke("isempty")
    if (isempty) {
      play.innerHTML="PLAY";
    }
  }, 50);

  greetInputEl = document.querySelector("#greet-input");
  greetMsgEl = document.querySelector("#greet-msg");

});
