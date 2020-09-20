// eslint-disable-next-line @typescript-eslint/no-explicit-any
const ctx: Worker = self as any;

export type IncomingData =
  | {
      cmd: 'init';
      bytes: Uint8Array;
    }
  | {
      cmd: 'search';
      q: string;
    };

export type OutgoingData =
  | {
      cmd: 'init';
      success: boolean;
    }
  | {
      cmd: 'search';
      q: string;
      res: Uint8Array;
    };

let wasm: typeof import('../wasm') | null = null;

async function init(bytes: Uint8Array) {
  try {
    wasm = await import('../wasm');
    wasm.init(bytes);
    ctx.postMessage({ cmd: 'init', success: true } as OutgoingData);
  } catch (e) {
    ctx.postMessage({ cmd: 'init', success: false } as OutgoingData);
  }
}

function search(q: string) {
  if (!wasm) {
    return;
  }

  const res = wasm.search(q);
  ctx.postMessage({ cmd: 'search', q, res } as OutgoingData);
}

// Respond to message from parent thread
ctx.addEventListener('message', (event) => {
  const data: IncomingData = event.data;
  if (data.cmd === 'search') {
    search(event.data.q);
  } else if (event.data.cmd === 'init') {
    init(data.bytes);
  }
});
