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
* @param {(FrameInfo)[]} buffer
* @param {number} w
* @param {number} h
* @param {number} sx
* @param {number} sy
* @param {number} sw
* @param {number} sh
* @returns {(FrameInfo)[]}
*/
export function cropChunk(buffer: (FrameInfo)[], w: number, h: number, sx: number, sy: number, sw: number, sh: number): (FrameInfo)[];
/**
* @param {(FrameInfo)[]} chunks
* @param {number} w
* @param {number} h
* @returns {Uint8Array}
*/
export function combineChunks(chunks: (FrameInfo)[], w: number, h: number): Uint8Array;
/**
* @param {Uint8Array} buffer
* @param {number} sx
* @param {number} sy
* @param {number} sw
* @param {number} sh
* @param {number} dw
* @param {number} dh
* @returns {Uint8Array}
*/
export function cropGif(buffer: Uint8Array, sx: number, sy: number, sw: number, sh: number, dw: number, dh: number): Uint8Array;
/**
* @param {Uint8Array} buffer
* @param {number} sx
* @param {number} sy
* @param {number} sw
* @param {number} sh
* @param {number} dw
* @param {number} dh
* @returns {Uint8Array}
*/
export function cropImage(buffer: Uint8Array, sx: number, sy: number, sw: number, sh: number, dw: number, dh: number): Uint8Array;
/**
* Handler for `console.log` invocations.
*
* If a test is currently running it takes the `args` array and stringifies
* it and appends it to the current output of the test. Otherwise it passes
* the arguments to the original `console.log` function, psased as
* `original`.
* @param {Array<any>} args
*/
export function __wbgtest_console_log(args: Array<any>): void;
/**
* Handler for `console.debug` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_debug(args: Array<any>): void;
/**
* Handler for `console.info` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_info(args: Array<any>): void;
/**
* Handler for `console.warn` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_warn(args: Array<any>): void;
/**
* Handler for `console.error` invocations. See above.
* @param {Array<any>} args
*/
export function __wbgtest_console_error(args: Array<any>): void;
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
/**
*/
export class FrameInfo {
  free(): void;
}
/**
* Runtime test harness support instantiated in JS.
*
* The node.js entry script instantiates a `Context` here which is used to
* drive test execution.
*/
export class WasmBindgenTestContext {
  free(): void;
/**
* Creates a new context ready to run tests.
*
* A `Context` is the main structure through which test execution is
* coordinated, and this will collect output and results for all executed
* tests.
*/
  constructor();
/**
* Inform this context about runtime arguments passed to the test
* harness.
* @param {any[]} args
*/
  args(args: any[]): void;
/**
* Executes a list of tests, returning a promise representing their
* eventual completion.
*
* This is the main entry point for executing tests. All the tests passed
* in are the JS `Function` object that was plucked off the
* `WebAssembly.Instance` exports list.
*
* The promise returned resolves to either `true` if all tests passed or
* `false` if at least one test failed.
* @param {any[]} tests
* @returns {Promise<any>}
*/
  run(tests: any[]): Promise<any>;
}
