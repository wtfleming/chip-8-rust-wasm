const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');


let isRunning = false;

import("./crate/pkg/index.js").then(wasm => {

  wasm.init();

  loadRom(wasm, 'PONG2')
    // loadRom(wasm, 'WIPEOFF')
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
    }
  });

  document.addEventListener("keyup", event => {
    let keyCode = keyMap[event.key];
    if (keyCode >= 0 && keyCode <= 0xf) {
      wasm.key_up(keyMap[event.key]);
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
    // Run 9 steps to emulate a ~540hz cpu
    for (let x = 0; x <= 9; x++) {
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
  wasm.load_game_js(rom);
}


// CHIP-8 Keypad    User Keyboard
// +-+-+-+-+        +-+-+-+-+
// |1|2|3|C|        |1|2|3|4|
// +-+-+-+-+        +-+-+-+-+
// |4|5|6|D|        |Q|W|E|R|
// +-+-+-+-+   <=   +-+-+-+-+
// |7|8|9|E|        |A|S|D|F|
// +-+-+-+-+        +-+-+-+-+
// |A|0|B|F|        |Z|X|C|V|
// +-+-+-+-+        +-+-+-+-+

const keyMap = {
  '1': 0x1,
  '2': 0x2,
  '3': 0x3,
  '4': 0xc,

  'q': 0x4,
  'w': 0x5,
  'e': 0x6,
  'r': 0xd,

  'a': 0x7,
  's': 0x8,
  'd': 0x9,
  'f': 0xe,

  'z': 0xa,
  'x': 0x0,
  'c': 0xb,
  'v': 0xf,
};
