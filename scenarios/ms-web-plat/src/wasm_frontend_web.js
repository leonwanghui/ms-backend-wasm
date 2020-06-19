const wasm =
  import('../pkg/wasm_backend_web');

/**
 * @param {number} op_type
 * @param {number} data_type
 * @param {string} file_data
 * @returns {any}
 */
export function load(op_type, data_type, file_data) {

  wasm
    .then(m => {
      let body = m.run(op_type, data_type, file_data);

      console.log(body);
      document.getElementById('text').innerHTML =
        JSON.stringify(body, null, 2);
    })
    .catch(console.error);
};