cargo build --release --target wasm32-unknown-unknown

wasm-bindgen --target web --out-dir wbindgen/ target/wasm32-unknown-unknown/release/hextacker.wasm

cp ./wbindgen/hextacker.js ./hextacker.js

get-content .\hextacker.js | %{$_ -replace [regex]::Escape("import * as __wbg_star0 from 'env';")} | out-file hextacker.tmp -encoding utf8
rm hextacker.js
mv hextacker.tmp hextacker.js

get-content .\hextacker.js | %{$_ -replace [regex]::Escape("let wasm;"),"let wasm; export const set_wasm = (w) => wasm = w;"} | out-file hextacker.tmp -encoding utf8
rm hextacker.js
mv hextacker.tmp hextacker.js

get-content .\hextacker.js | %{$_ -replace [regex]::Escape("imports['env'] = __wbg_star0;"),"return imports.wbg;"} | out-file hextacker.tmp -encoding utf8
rm hextacker.js
mv hextacker.tmp hextacker.js

get-content .\hextacker.js | %{$_ -replace [regex]::Escape("const imports = getImports();"),"return getImports();"} | out-file hextacker.tmp -encoding utf8
rm hextacker.js
mv hextacker.tmp hextacker.js

rm ./webtest/hextacker.js
mv ./hextacker.js ./webtest/hextacker.js

rm ./webtest/hextacker_bg.wasm
mv ./wbindgen/hextacker_bg.wasm ./webtest/hextacker_bg.wasm