mkdir -p build
cd build

while true
do
  wc -c main.wasm
  zig build-lib \
    -O Debug \
    -rdynamic \
    -dynamic -target wasm32-freestanding ../src/*.cpp

  sleep 1
done

# clang \
#    --target=wasm32 \
#    -O3 \
#    -flto \
#    -nostdlib \
#    -Wl,--no-entry \
#    -Wl,--export-all \
#    -Wl,--lto-O3 \
#    -Wl,--allow-undefined \
#    -Wall \
#    -mbulk-memory \
#    -o main.wasm \
#    ../main.c

# ../wasm_sourcemap.py \
#   main.wasm \
#   --dwarfdump /usr/local/opt/llvm/bin/llvm-dwarfdump \
#   -s \
#   -w main.wasm \
#   -u main.wasm.map \
#   -o main.wasm.map
