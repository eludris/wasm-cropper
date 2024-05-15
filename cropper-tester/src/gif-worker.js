import * as wasm from "eludris-wasm-cropper";


wasm.init_panic_hook();

const xd = Object.getPrototypeOf(wasm.FrameInfo);


onmessage = (e) => {
    switch (e.data.action) {

        case ("chunkGif"): {
            console.log("chunkGif in worker\n\n", e.data)

            let chunked = wasm.chunkGif(e.data.data.buffer, e.data.data.chunks)
            console.log("chunkGif data\n\n", chunked)

            postMessage(
                {
                    action: "chunkGifWorkerResult",
                    data: chunked,
                }
            )
            break;
        }

        case ("cropChunk"): {
            console.log("cropChunk in worker\n\n", e.data)
            postMessage(
                {
                    action: "cropChunkWorkerResult",
                    data: wasm.cropChunk(
                        e.data.data.buffer,
                        e.data.data.w,
                        e.data.data.h,
                        e.data.data.sx,
                        e.data.data.sy,
                        e.data.data.sw,
                        e.data.data.sh,
                    ),
                    chunk: e.data.chunk,
                }
            )
            break;
        }

        case ("combineChunks"): {
            console.log("combineChunks in worker\n\n", e.data)
            postMessage(
                {
                    action: "combineChunksWorkerResult",
                    data: wasm.combineChunks(
                        e.data.data.buffer,
                        e.data.data.sw,
                        e.data.data.sh,
                    ),
                }
            )
            break;
        }

        // TODO: Image cropping

    }
};
