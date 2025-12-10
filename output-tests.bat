mkdir output -p
cargo run --bin naga -- naga/tests/in/wgsl/mesh-shader.wgsl output/all.spv --spirv-version 1.4 --generate-debug-symbols --compact --keep-coordinate-space
cargo run --bin naga -- naga/tests/in/wgsl/mesh-shader.wgsl output/task.spv --spirv-version 1.4 --generate-debug-symbols --compact --entry-point ts_main --keep-coordinate-space
cargo run --bin naga -- naga/tests/in/wgsl/mesh-shader.wgsl output/mesh.spv --spirv-version 1.4 --generate-debug-symbols --compact --entry-point ms_main --keep-coordinate-space
cargo run --bin naga -- naga/tests/in/wgsl/mesh-shader.wgsl output/frag.spv --spirv-version 1.4 --generate-debug-symbols --compact --entry-point fs_main --keep-coordinate-space

spirv-opt output/all.spv -o output/all-opt.spv
spirv-opt output/task.spv -o output/task-opt.spv
spirv-opt output/mesh.spv -o output/mesh-opt.spv
spirv-opt output/frag.spv -o output/frag-opt.spv

spirv-cross output/task.spv --output output/task.glsl -V
spirv-cross output/mesh.spv --output output/mesh.glsl -V
spirv-cross output/frag.spv --output output/frag.glsl -V

glslc -fshader-stage=task --target-env=vulkan1.2 -c output/task.glsl -o output/task-glsl.spv
glslc -fshader-stage=mesh --target-env=vulkan1.2 -c output/mesh.glsl -o output/mesh-glsl.spv
glslc -fshader-stage=frag --target-env=vulkan1.2 -c output/frag.glsl -o output/frag-glsl.spv

glslc -fshader-stage=task --target-env=vulkan1.2 -c output/task-correct.glsl -o output/task-correct.spv
glslc -fshader-stage=mesh --target-env=vulkan1.2 -c output/mesh-correct.glsl -o output/mesh-correct.spv
glslc -fshader-stage=frag --target-env=vulkan1.2 -c output/frag-correct.glsl -o output/frag-correct.spv

glslc -fshader-stage=mesh --target-env=vulkan1.2 -c output/mesh-custom.glsl -o output/mesh-custom.spv

spirv-dis output/actually-run.spv -o output/actually-run.spv-asm
spirv-cross output/actually-run.spv --output output/actually-run.glsl -V