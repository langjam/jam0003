mkdir -p build
cd build

while true
do
  clang++ \
    -Ofast \
    -fdebug-compilation-dir=.. \
    -target wasm32 \
    -flto \
    -nostdlib \
    -Wl,--no-entry \
    -Wl,--export-all \
    -Wl,--lto-O3 \
    -Wl,--allow-undefined \
    -Wall \
    -mbulk-memory \
    -o main.wasm \
    ../src/*.cpp
  rm -rf src
  cp -r ../src src
  wc -c main.wasm
  sleep 1
done


