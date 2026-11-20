import init, { Chip8Emulator } from './emulator/pkg/emulator.js';

const SCREEN_WIDTH = 64;
const SCREEN_HEIGHT = 32;
const SCALE = 10; // 64*10 = 640, 32*10 = 320
const CYCLES_PER_FRAME = 10;

// Standard QWERTY to CHIP-8 16-key hex map
const KEY_MAP = {
  '1': 0x1, '2': 0x2, '3': 0x3, '4': 0xC,
  'q': 0x4, 'w': 0x5, 'e': 0x6, 'r': 0xD,
  'a': 0x7, 's': 0x8, 'd': 0x9, 'f': 0xE,
  'z': 0xA, 'x': 0x0, 'c': 0xB, 'v': 0xF,
};

async function start() {
  const wasm = await init();
  const emu = new Chip8Emulator();

  const canvas = document.getElementById('screen');
  const ctx = canvas.getContext('2d');
  const romSelect = document.getElementById('rom-select');
  const reloadBtn = document.getElementById('reload-btn');

  async function loadRom(url) {
    const res = await fetch(url);
    if (!res.ok) {
      console.error(`Failed to load ROM at ${url}`);
      return;
    }
    const buffer = await res.arrayBuffer();
    emu.load_rom(new Uint8Array(buffer));
  }

  function renderDisplay() {
    // Re-create view each frame in case WASM linear memory ever grows
    const displayView = new Uint8Array(
      wasm.memory.buffer,
      emu.display_ptr(),
      emu.display_len()
    );

    ctx.fillStyle = '#000000';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.fillStyle = '#33ff66';
    for (let y = 0; y < SCREEN_HEIGHT; y++) {
      for (let x = 0; x < SCREEN_WIDTH; x++) {
        if (displayView[y * SCREEN_WIDTH + x] !== 0) {
          ctx.fillRect(x * SCALE, y * SCALE, SCALE, SCALE);
        }
      }
    }
  }

  window.addEventListener('keydown', (e) => {
    const mapped = KEY_MAP[e.key.toLowerCase()];
    if (mapped !== undefined) {
      emu.set_key(mapped, true);
    }
  });

  window.addEventListener('keyup', (e) => {
    const mapped = KEY_MAP[e.key.toLowerCase()];
    if (mapped !== undefined) {
      emu.set_key(mapped, false);
    }
  });

  reloadBtn.addEventListener('click', () => loadRom(romSelect.value));
  romSelect.addEventListener('change', () => loadRom(romSelect.value));

  await loadRom(romSelect.value);

  function frameLoop() {
    for (let i = 0; i < CYCLES_PER_FRAME; i++) {
      emu.tick();
    }
    emu.tick_timers();
    renderDisplay();
    requestAnimationFrame(frameLoop);
  }

  requestAnimationFrame(frameLoop);
}

start();