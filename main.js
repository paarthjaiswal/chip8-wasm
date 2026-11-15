// Import the auto-generated JS glue directly from wasm-pack's output folder
import init, { ping } from './emulator/pkg/emulator.js';

async function bootstrap() {
  // 1. Fetch and instantiate the `emulator_bg.wasm` binary in browser memory
  await init();

  // 2. Call our exported Rust function!
  console.log(ping());

  // 3. Draw a quick test square on the canvas to verify Vanilla JS rendering
  const canvas = document.getElementById('screen');
  const ctx = canvas.getContext('2d');
  ctx.fillStyle = '#00ff00';
  ctx.fillRect(10, 10, 10, 10); // Simulates one 10x-scaled CHIP-8 pixel at (1, 1)
}

bootstrap();