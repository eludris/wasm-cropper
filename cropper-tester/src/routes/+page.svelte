<script lang="ts">
  // Based on https://github.com/eludris/client

  import { consumeSingleEvent } from "$lib/events";

  let image: HTMLImageElement // = document.createElement('img');
  let cutout: HTMLDivElement // = document.createElement('div');
  let outImage: HTMLImageElement // = document.createElement('img');

  let avatarFiles: FileList | undefined | null = undefined;
  let cropperFile: Blob | null | undefined = undefined;
  let cropperKind = "avatar";

  let dragging = false;
  let imageX = 0;
  let imageY = 0;
  let lastX = 0;
  let lastY = 0;
  let xBoundary = 0;
  let yBoundary = 0;
  let scale = 1;

  $: if (avatarFiles) {
    cropperFile = avatarFiles![0];
    image.src = URL.createObjectURL(cropperFile!)
  }

  const updateBoundaries = () => {
    xBoundary = (image.width - cutout.clientWidth / scale) / 2;
    yBoundary = (image.height - cutout.clientHeight / scale) / 2;
  };

  const updateImagePosition = () => {
    if (image.width * scale > cutout.clientWidth) {
      imageX = Math.max(Math.min(imageX, xBoundary), -xBoundary);
    } else {
      imageX = 0;
    }
    if (image.height * scale > cutout.clientHeight) {
      imageY = Math.max(Math.min(imageY, yBoundary), -yBoundary);
    } else {
      imageY = 0;
    }
    image.style.transform = `translate(${imageX * scale}px, ${imageY * scale}px) scale(${scale})`;
  };

  const startDrag = (x: number, y: number) => {
    updateBoundaries();  // Somehow prevents a bug where boundaries aren't set correctly
    lastX = x;
    lastY = y;
    dragging = true;
  };

  const mouseStartDrag = (e: MouseEvent) => {
    startDrag(e.x, e.y);
  };

  const touchStartDrag = (e: TouchEvent) => {
    // TODO: Maybe support zoom by dragging with two fingers
    startDrag(e.touches[0].clientX, e.touches[0].clientY);
  };

  const stopDrag = () => {
    dragging = false;
  };

  const move = (x: number, y: number) => {
    if (dragging) {
      // Divide move distance by scale to slow down movement when zoomed.
      imageX += (x - lastX) / scale;
      imageY += (y - lastY) / scale;
      updateImagePosition();
      lastX = x;
      lastY = y;
    }
  }

  const mouseMove = (e: MouseEvent) => {
    move(e.x, e.y);
  };
  
  const touchMove = (e: TouchEvent) => {
    move(e.touches[0].clientX, e.touches[0].clientY)
  }

  const onWheel = (e: WheelEvent) => {
    if (e.deltaY < 0) {
      scale = Math.min(scale + 0.1, 5);
    } else {
      scale = Math.max(scale - 0.1, 0.5);
    }
    scaleImage();
  };

  const scaleImage = () => {
    updateBoundaries();
    updateImagePosition();
  };

  const cropImpl = async (kind: "GIF" | "image"): Promise<Blob> => {
    let inEvent: string;
    let resultEvent: string;
  
    if (kind == "GIF") {
      inEvent = "cropGifInWorker";
      resultEvent = "cropGifResult";
    } else {
      inEvent = "cropImageInWorker";
      resultEvent = "cropImageResult";
    }

    // Compensate for any scaling automatically done by css...
    let cssScale = image.width / image.naturalWidth;
    let effectiveScale = scale * cssScale;
    
    // Dispatch event to start cropper worker...
    document.dispatchEvent(
      new CustomEvent(
        inEvent,
        {
          detail:
          {
            buffer: new Uint8Array(await cropperFile!.arrayBuffer()),
            imageWidth: image.naturalWidth,
            imageHeight: image.naturalHeight,
            windowX: (xBoundary - imageX) / cssScale,
            windowY: (yBoundary - imageY) / cssScale,
            windowWidth: cutout.clientWidth / effectiveScale,
            windowHeight: cutout.clientHeight / effectiveScale
          }
        }
      )
    )
  

    
    let event: CustomEvent<Blob> = await consumeSingleEvent(resultEvent, ()=>true);
    
    return event.detail;
  }

  const doCrop = async () => {
    updateBoundaries();

    let imageResponse = await fetch(image.src);
    let contentType = imageResponse.headers.get('content-type');
    let blob: Blob;
   
    if (contentType == 'image/gif') {
      blob = await cropImpl("GIF");
    } else {
      blob = await cropImpl("image");
    }
    
    outImage.src = URL.createObjectURL(blob);
  }

  const cropSkip = () => {
    if (cropperKind == "avatar") {
      cropperKind = "banner"
    } else {
      cropperKind = "avatar"
    };
  }

  const cropperDismiss = () => {
    alert("no")
  }

</script>

<svelte:body
  on:mouseup={stopDrag}
  on:mousemove={mouseMove}
  on:touchend={stopDrag}
  on:touchmove={touchMove}
/>

<div>
  <input
    id="image-input"
    name="avatar"
    type="file"
    accept="image/*"
    bind:files={avatarFiles}
  />
  <div id="cropper">
    <div id="cropper-image-preview">
      <img alt="Hi marco" id="cropper-img" bind:this={image} />
      <!-- svelte-ignore a11y-no-static-element-interactions -->
      <div
        id="overlay"
        on:mousedown={mouseStartDrag}
        on:touchstart={touchStartDrag}
        on:wheel={onWheel}
      >
        <div
          id="overlay-cutout"
          class={cropperKind}
          bind:this={cutout}
        />
      </div>
    </div>
    <div id="cropper-slider">
      <input type="range" min="0.5" max="5" step="0.0001" bind:value={scale} on:input={scaleImage} />
    </div>
  </div>
  <div>
    <div id="cropper-buttons">
      <button class="cropper-button" id="cropper-skip-button" on:click={cropSkip}>
        Skip
      </button>
      <div id="button-separator" />
      <button class="cropper-button" id="cropper-cancel-button" on:click={cropperDismiss}>
        Cancel
      </button>
      <button class="cropper-button" id="cropper-crop-button" on:click={doCrop}>
        Crop
      </button>
    </div>
  </div>
</div>
<div id="crop-preview-container">
  <img alt="Result preview" id="crop-preview" bind:this={outImage}/>
</div>

<style>
  #cropper {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  #cropper-image-preview {
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
    background-color: var(--purple-100);
    border-radius: 5px;
    overflow: hidden;
    width: max(300px, 60vw);
    height: 300px;
  }

  #cropper-img {
    border-radius: 5px;
    object-fit: cover;
    height: 100%;
  }

  #overlay {
    position: absolute;
    top: 0;
    left: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 100%;
    width: 100%;
    cursor: move;
    touch-action: none;
  }

  #overlay-cutout.avatar{
    border: 3px solid white;
    border-radius: 10px;
    box-shadow: 0 0 200px 200px #000c;
    width: 256px;
    height: 256px;
    border-radius: 100%;
  }

  #overlay-cutout.banner{
    border: 3px solid white;
    border-radius: 10px;
    box-shadow: 0 0 200px 200px #000c;
    width: 90%;
    aspect-ratio: 6;
    border-radius: 5px;
  }

  #cropper-slider {
    display: flex;
    margin: 10px auto;
    justify-content: center;
  }

  #cropper-buttons {
    display: flex;
    flex-direction: row;
  }

  .cropper-button {
    margin: 5px;
    /* background-color: transparent;
    border: none;
    color: white; */
    padding: 10px 10px;
    border-radius: 5px;
    font-size: 12pt;
    margin: 0px 20px;
  }

  #button-separator {
    flex-grow: 1;
  }

  #cropper-crop-button {
    /* background-color: var(--gray-300); */
    padding: 10px 40px;
  }

  .cropper-button:hover {
    text-decoration: underline;
  }

  #cropper-crop-button:hover {
    background-color: var(--gray-400);
  }

  #crop-preview-container {
    display: flex;
    align-items: center;
    justify-content: center;
  }

  #crop-preview {
    width: 90%;
  }
</style>
