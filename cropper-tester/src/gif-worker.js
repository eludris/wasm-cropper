import * as wasm from "eludris-wasm-cropper";

wasm.init_panic_hook();

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

        case ("mergeFrames"): {
            console.log("mergeFrames in worker\n\n", e.data)
            postMessage(
                {
                    action: "mergeFramesWorkerResult",
                    data: wasm.mergeFrames(
                        e.data.data.buffer,
                        e.data.data.w,
                        e.data.data.h,
                    ),
                }
            )
            break;
        }

        // TODO: Image cropping

    }
};
