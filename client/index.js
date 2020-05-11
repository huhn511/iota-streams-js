// Note that a dynamic `import` statement here is required due to
// webpack/webpack#6615, but in theory `import { greet } from './pkg';`
// will work here one day as well!
const wasmmodule = import('./pkg');
async function generateAddress(seed, index, security, checksum) {
  try {
    let wasm = await import('./pkg')
    let address = wasm.new_address(seed, index, security, checksum)
    console.log(address);
  } catch (err) {
    console.error(err)
  }
}

async function showAlert() {
  try {
    wasmmodule
      .then(m => m.greet('World!'))
      .catch(console.error);
  } catch (err) {
    console.error(err)
  }
}


export default { showAlert, generateAddress, wasmmodule }