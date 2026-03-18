#let core = plugin(
  "./target/wasm32-unknown-unknown/release/typst_godsays_core.wasm",
)

#let godsays(words) = str(core.godsays(words.to-bytes(
  endian: "little",
  size: 4,
)))
