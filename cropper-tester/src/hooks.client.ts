import MyWorker from "/src/gif-worker.js?worker"

import * as events from "$lib/events";

import * as wasm from "eludris-wasm-cropper";


type CropGifEventDetail = {
  buffer: Uint8Array,
  imageWidth: number,
  imageHeight: number,
  windowX: number,
  windowY: number,
  windowWidth: number,
  windowHeight: number
};
type CropGifEvent = CustomEvent<CropGifEventDetail>;

type ChunkGifResultEvent = CustomEvent<Uint8Array[][]>;

type CropChunkResultEventDetail = {
  data: Uint8Array[],
  chunk: number
};
type CropChunkResultEvent = CustomEvent<CropChunkResultEventDetail>;

type CombineChunksResultEvent = CustomEvent<Uint8Array>;


enum CropperEvent {
  CHUNK_GIF_RESULT = "_cropperInternal_ChunkGifResult",
  CROP_CHUNK_RESULT = "_cropperInternal_CropChunkResult",
  COMBINE_CHUNKS_RESULT = "_cropperInternal_CombineChunksResult",
} 


const onmessage = (e: MessageEvent<{action: string, data: any, chunk?: number}>) => {
  switch (e.data.action) {

    case ("chunkGifWorkerResult"): {
      console.log("chunkGifWorkerResult received")
      console.log(e.data)

      document.dispatchEvent(
        new CustomEvent(
          CropperEvent.CHUNK_GIF_RESULT,
          {
            detail: e.data.data
          }
        )
      )
      break;
    }

    case ("cropChunkWorkerResult"): {
      console.log("cropChunkWorkerResult received")
      console.log(e.data)

      document.dispatchEvent(
        new CustomEvent(
          CropperEvent.CROP_CHUNK_RESULT,
          {
            detail: {
              data: e.data.data,
              chunk: e.data.chunk,
            }
          }
        )
      )
      break;
    }

    case ("combineChunksWorkerResult"): {
      console.log("combineChunksWorkerResult received")
      console.log(e.data)

      document.dispatchEvent(
        new CustomEvent(
          CropperEvent.COMBINE_CHUNKS_RESULT,
          {
            detail: e.data.data
          }
        )
      )
      break;
    }

  }
}

const onerror = (e: ErrorEvent) => {
  console.error(e);
}


let workers = [];
for (let i = 0; i < 2; i++) {  // window.navigator.hardwareConcurrency
  let worker = new MyWorker;
  workers.push(worker);
  worker.onmessage = onmessage;
  worker.onerror = onerror;
}


document.addEventListener(
  "cropGifInWorker",
  (
    async (e: CropGifEvent) => {
      let data = e.detail;

      workers[0].postMessage(
        {
          action: "chunkGif",
          data: {
            buffer: data.buffer,
            chunks: workers.length,
          }
        },
        // [data.buffer.buffer]
      )

      let chunks: any[][];  // Array<Array<FrameInfo>>
      {
        let event: ChunkGifResultEvent = await events.consumeSingleEvent(CropperEvent.CHUNK_GIF_RESULT);
        chunks = event.detail;
      }

      let croppedChunks: any[];  // Array<FrameInfo>
      {
        let waiters: Promise<CropChunkResultEvent>[] = []

        for (let i = 0; i < workers.length; i++) {
          workers[i].postMessage(
            {
              action: "cropChunk",
              data: {
                buffer: chunks[i],
                w: data.imageWidth,
                h: data.imageHeight,
                sx: data.windowX,
                sy: data.windowY,
                sw: data.windowWidth,
                sh: data.windowHeight,
              },
              chunk: i
            },
          )

          waiters.push(
            events.consumeSingleEvent(
              CropperEvent.CROP_CHUNK_RESULT,
              (e) => e.detail.chunk == i
            )
          )
        }

        croppedChunks = (await Promise.all(waiters)).map((e) => e.detail.data).flat(1);
      }
      
      workers[0].postMessage(
        {
          action: "combineChunks",
          data: {
            buffer: croppedChunks,
            w: data.windowWidth,
            h: data.windowHeight,
          }
        },
      )

      let combined: CombineChunksResultEvent = await events.consumeSingleEvent(CropperEvent.COMBINE_CHUNKS_RESULT);
      document.dispatchEvent(
        new CustomEvent(
          "cropGifResult",
          {
            detail: new Blob([combined.detail])
          }
        )
      )

    }
  ) as events.AsyncEventListener
)


// Temporary thing to figure out why workers don't work
document.addEventListener(
  "cropGifNoWorker",
  (
    async (e: CropGifEvent) => {
      let data = e.detail;

      let chunks = wasm.chunkGif(data.buffer, workers.length)

      console.log(
        data.imageWidth,
        data.imageHeight,
        data.windowX,
        data.windowY,
        data.windowWidth,
        data.windowHeight
      )

      let croppedChunks = []
      for (let i = 0; i < workers.length; i++) {
        let res = wasm.cropChunk(
          chunks[i],
          data.imageWidth,
          data.imageHeight,
          data.windowX,
          data.windowY,
          data.windowWidth,
          data.windowHeight
        )
        croppedChunks.push(...res)
      }

      console.log(croppedChunks)

      let result = wasm.combineChunks(
        croppedChunks,
        data.windowWidth,
        data.windowHeight
      )

      // This is listener is blocking, so we give the "caller" time to start listening.
      setTimeout(
        () => {
          document.dispatchEvent(
            new CustomEvent(
              "cropGifResult",
              {
                detail: new Blob([result])
              }
            )
          )
        },
        10
      )
    }
  ) as events.AsyncEventListener
)
