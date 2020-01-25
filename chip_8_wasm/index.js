const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');


let isRunning = false;

import("./crate/pkg/index.js").then(wasm => {

  wasm.init();

  loadRom(wasm, 'PONG2')
//    loadRom(wasm, 'WIPEOFF')
    .then(() => {
      console.log('ROM finished loading');
      runLoop(wasm);

      wasm.draw_canvas(ctx);
      wasm.update_ui();
    });


  const startButton = document.getElementById("start-button");
  const stopButton = document.getElementById("stop-button");
  const stepButton = document.getElementById("step-button");

  startButton.addEventListener('click', () => {
    isRunning = true;
    startButton.disabled = true;
    stopButton.disabled = false;
    stepButton.disabled = true;
  });

  stopButton.addEventListener('click', () => {
    isRunning = false;
    startButton.disabled = false;
    stopButton.disabled = true;
    stepButton.disabled = false;
  });

  stepButton.addEventListener('click', () => {
    doStep(wasm);
  });

  document.addEventListener("keydown", event => {
    let keyCode = keyMap[event.key];
    if (keyCode >= 0 && keyCode <= 0xf) {
      wasm.key_down(keyMap[event.key]);
//      console.log(event);
    }
    
    

  });

  document.addEventListener("keyup", event => {
    let keyCode = keyMap[event.key];
    if (keyCode >= 0 && keyCode <= 0xf) {
      wasm.key_up(keyMap[event.key]);
//      console.log(event);
    }
  });


}).catch(console.error);

function doStep(wasm) {
  let result = wasm.emulate_cycle();
  if (result === true) {
    wasm.draw_canvas(ctx);
    wasm.update_ui();
  } else {
    isRunning = false;
  }
}

function runLoop(wasm) {
  if (isRunning) {
    // Run 10 steps to emulate a 600hz cpu
    for (let x = 0; x <= 10; x++) {
      doStep(wasm);
    }
  }

  window.requestAnimationFrame(() => {
    runLoop(wasm);
  });
}



async function loadRom(wasm, name) {
  let i = await fetch(`roms/${name}`);
  let buffer = await i.arrayBuffer();
  const rom = new DataView(buffer, 0, buffer.byteLength);

  // TODO wasm.reset_cpu();   

  wasm.load_game_js(rom);
}


// Keypad                   Keyboard
// +-+-+-+-+                +-+-+-+-+
// |1|2|3|C|                |1|2|3|4|
// +-+-+-+-+                +-+-+-+-+
// |4|5|6|D|                |Q|W|E|R|
// +-+-+-+-+       =>       +-+-+-+-+
// |7|8|9|E|                |A|S|D|F|
// +-+-+-+-+                +-+-+-+-+
// |A|0|B|F|                |Z|X|C|V|
// +-+-+-+-+                +-+-+-+-+


const keyMap = {
  '1': 0x1, // 1
  '2': 0x2, // 2
  '3': 0x3, // 3
  '4': 0xc, // 4

  'q': 0x4, // Q
  'w': 0x5, // W
  'w': 0x6, // E
  'r': 0xd, // R

  'a': 0x7, // A
  's': 0x8, // S
  'd': 0x9, // D
  'f': 0xe, // F

  'z': 0xa, // Z
  'x': 0x0, // X
  'c': 0xb, // C
  'v': 0xf,  // V
};



// const keyMap = {
//   49: 0x1, // 1
//   50: 0x2, // 2
//   51: 0x3, // 3
//   52: 0xc, // 4

//   81: 0x4, // Q
//   87: 0x5, // W
//   69: 0x6, // E
//   82: 0xd, // R

//   65: 0x7, // A
//   83: 0x8, // S
//   68: 0x9, // D
//   70: 0xe, // F

//   90: 0xa, // Z
//   88: 0x0, // X
//   67: 0xb, // C
//   86: 0xf  // V
// };
