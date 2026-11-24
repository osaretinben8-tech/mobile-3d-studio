// Loads the Rust geometry kernel compiled to WebAssembly.
window.rustMeshKernelReady = (async () => {
  try {
    const module = await import('./wasm/pkg/mobile_3d_mesh_kernel.js');
    await module.default();
    window.rustMeshKernel = module.process_mesh_json;
    window.rustMeshKernelError = null;
    document.documentElement.dataset.rustMeshKernel = 'ready';
    return true;
  } catch (error) {
    window.rustMeshKernelError = error;
    document.documentElement.dataset.rustMeshKernel = 'unavailable';
    console.error('Rust mesh kernel failed to load:', error);
    return false;
  }
})();
