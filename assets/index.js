const importObject = {
  module: {}
};

fetch('radix_bg.wasm').then(response =>
  response.arrayBuffer()
).then(bytes =>
  WebAssembly.instantiate(bytes, importObject)
).then(result => {
  if (result.instance instanceof Object) {
    const { exports } = result.instance;
    const { decimal_to_radix,
      float_to_fraction,
      fraction_to_unit,
      radix_fraction_to_radix,
      radix_to_decimal
    } = exports;

    console.log(decimal_to_radix)
  };
});