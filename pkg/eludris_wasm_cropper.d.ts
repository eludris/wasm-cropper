/* tslint:disable */
/* eslint-disable */
/**
*/
export function init_panic_hook(): void;
/**
* @param {Uint8Array} buffer
* @param {number} chunks
* @returns {Array<any>}
*/
export function chunkGif(buffer: Uint8Array, chunks: number): Array<any>;
/**
* @param {(Uint8Array)[]} buffer
* @param {number} w
* @param {number} h
* @param {number} sx
* @param {number} sy
* @param {number} sw
* @param {number} sh
* @returns {(Uint8Array)[]}
*/
export function cropChunk(buffer: (Uint8Array)[], w: number, h: number, sx: number, sy: number, sw: number, sh: number): (Uint8Array)[];
/**
* @param {(Uint8Array)[]} buffer
* @param {number} w
* @param {number} h
* @returns {Uint8Array}
*/
export function mergeFrames(buffer: (Uint8Array)[], w: number, h: number): Uint8Array;
/**
* @param {Uint8Array} buffer
* @param {number} sx
* @param {number} sy
* @param {number} sw
* @param {number} sh
* @returns {Uint8Array}
*/
export function cropImage(buffer: Uint8Array, sx: number, sy: number, sw: number, sh: number): Uint8Array;
/**
* Chroma subsampling format
*/
export enum ChromaSampling {
/**
* Both vertically and horizontally subsampled.
*/
  Cs420 = 0,
/**
* Horizontally subsampled.
*/
  Cs422 = 1,
/**
* Not subsampled.
*/
  Cs444 = 2,
/**
* Monochrome.
*/
  Cs400 = 3,
}

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly init_panic_hook: () => void;
  readonly chunkGif: (a: number, b: number, c: number) => number;
  readonly cropChunk: (a: number, b: number, c: number, d: number, e: number, f: number, g: number, h: number, i: number) => void;
  readonly mergeFrames: (a: number, b: number, c: number, d: number, e: number) => void;
  readonly cropImage: (a: number, b: number, c: number, d: number, e: number, f: number, g: number) => void;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_add_to_stack_pointer: (a: number) => number;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;
/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {SyncInitInput} module
*
* @returns {InitOutput}
*/
export function initSync(module: SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {InitInput | Promise<InitInput>} module_or_path
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: InitInput | Promise<InitInput>): Promise<InitOutput>;
