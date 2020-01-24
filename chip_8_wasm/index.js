const canvas = document.getElementById('chip8-canvas');
const ctx = canvas.getContext('2d');


let isRunning = false;

import("./crate/pkg/index.js").then(wasm => {

  wasm.init();

  loadRom(wasm, 'PONG2')
  //  loadRom(wasm, 'WIPEOFF')
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
    doStep(wasm);
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
