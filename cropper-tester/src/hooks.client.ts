import MyWorker from "/src/gif-worker.js?worker"

const myWorker = new MyWorker;

type CropEventDetail<T="image" | "gif"> = [T, Uint8Array, number, number, number, number, number, number]
type CropEvent<T> = CustomEvent<CropEventDetail<T> >

document.addEventListener(
  "startCropper",
  (
    (e: CropEvent<"image" | "gif">) => {
      myWorker.postMessage({transfer: e.detail});
    }
  ) as EventListener
)

myWorker.onmessage = (e: MessageEvent<{transfer: Blob}>) => {
  document.dispatchEvent(
    new CustomEvent("cropComplete", {detail: e.data.transfer})
  );
}

myWorker.onerror = (e) => {
  console.error(e);
}