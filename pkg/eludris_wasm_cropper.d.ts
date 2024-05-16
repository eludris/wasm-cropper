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
