import * as wasm from "eludris-wasm-cropper";

onmessage = (e) => {
    var method;
    let [kind, buffer, sx, sy, sw, sh, dx, dy] = e.data.transfer;
    if (kind == "gif") {
        method = wasm.cropGif;
    }
    else if (kind == "image") {
        method = wasm.cropImage;
    }
    else {
        throw new Error(`${kind} is not a valid cropper operation.`)
    }
    var croppedBuffer = method(buffer, sx, sy, sw, sh, dx, dy);
    postMessage({transfer: new Blob([croppedBuffer])});
};
