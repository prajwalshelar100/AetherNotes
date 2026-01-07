<script lang="ts">
  import { onMount } from "svelte";

  // Future-proofing: This component handles the Apple Pencil / Stylus logic.
  // Currently, it acts as a transparent overlay that resizes with the content.

  let canvas: HTMLCanvasElement;

  onMount(() => {
    resize();
    window.addEventListener("resize", resize);
    return () => window.removeEventListener("resize", resize);
  });

  function resize() {
    if (canvas && canvas.parentElement) {
      canvas.width = canvas.parentElement.clientWidth;
      canvas.height = canvas.parentElement.clientHeight;
    }
  }
</script>

<div class="handwriting-layer">
  <canvas bind:this={canvas} class="canvas-surface"></canvas>
</div>

<style>
  .handwriting-layer {
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    z-index: 10;
    pointer-events: none; /* Text input still works */
    overflow: hidden;
  }

  .canvas-surface {
    display: block;
    width: 100%;
    height: 100%;
  }
</style>
