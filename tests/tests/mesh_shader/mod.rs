use wgpu::util::DeviceExt;
use wgpu_test::{gpu_test, GpuTestConfiguration, TestParameters, TestingContext};

fn bytes_to_shader(device: &wgpu::Device, bytes: &[u8]) -> wgpu::ShaderModule {
    unsafe {
        let words = std::slice::from_raw_parts(bytes.as_ptr() as *const u32, bytes.len() / 4);
        device.create_shader_module_spirv(&wgpu::ShaderModuleDescriptorSpirV {
            label: None,
            source: words.into(),
        })
    }
}

fn mesh_pipeline_build(
    ctx: &TestingContext,
    task: Option<&[u8]>,
    mesh: &[u8],
    frag: Option<&[u8]>,
    draw: bool,
) {
    let device = &ctx.device;
    let task = task.map(|a| bytes_to_shader(device, a));
    let mesh = bytes_to_shader(device, mesh);
    let frag = frag.map(|a| bytes_to_shader(device, a));
    let pipeline = device.create_mesh_pipeline(&wgpu::MeshPipelineDescriptor {
        label: None,
        layout: None,
        task: task.as_ref().map(|task| wgpu::TaskState {
            module: task,
            entry_point: Some("main"),
            compilation_options: Default::default(),
        }),
        mesh: wgpu::MeshState {
            module: &mesh,
            entry_point: Some("main"),
            compilation_options: Default::default(),
        },
        fragment: frag.as_ref().map(|frag| wgpu::FragmentState {
            module: frag,
            entry_point: Some("main"),
            targets: &[],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
        depth_stencil: None,
        multisample: Default::default(),
        multiview: None,
        cache: None,
    });
    if draw {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
                color_attachments: &[],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(&pipeline);
            pass.draw_mesh_tasks(1, 1, 1);
        }
        ctx.queue.submit(Some(encoder.finish()));
        ctx.device.poll(wgt::Maintain::wait());
    }
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum DrawType {
    #[allow(dead_code)]
    Standard,
    Indirect,
    MultiIndirect,
    MultiIndirectCount,
}

fn mesh_draw(ctx: &TestingContext, typ: DrawType) {
    let device = &ctx.device;
    let task = bytes_to_shader(device, BASIC_TASK);
    let mesh = bytes_to_shader(device, BASIC_MESH);
    let frag = bytes_to_shader(device, NO_WRITE_FRAG);
    let pipeline = device.create_mesh_pipeline(&wgpu::MeshPipelineDescriptor {
        label: None,
        layout: None,
        task: Some(wgpu::TaskState {
            module: &task,
            entry_point: Some("main"),
            compilation_options: Default::default(),
        }),
        mesh: wgpu::MeshState {
            module: &mesh,
            entry_point: Some("main"),
            compilation_options: Default::default(),
        },
        fragment: Some(wgpu::FragmentState {
            module: &frag,
            entry_point: Some("main"),
            targets: &[],
            compilation_options: Default::default(),
        }),
        primitive: wgpu::PrimitiveState {
            cull_mode: Some(wgpu::Face::Back),
            ..Default::default()
        },
        depth_stencil: None,
        multisample: Default::default(),
        multiview: None,
        cache: None,
    });
    let buffer = match typ {
        DrawType::Standard => None,
        DrawType::Indirect | DrawType::MultiIndirect | DrawType::MultiIndirectCount => Some(
            device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: None,
                usage: wgpu::BufferUsages::INDIRECT,
                contents: bytemuck::bytes_of(&[1u32; 4]),
            }),
        ),
    };
    let count_buffer = match typ {
        DrawType::MultiIndirectCount => Some(device.create_buffer_init(
            &wgpu::util::BufferInitDescriptor {
                label: None,
                usage: wgpu::BufferUsages::INDIRECT,
                contents: bytemuck::bytes_of(&[1u32; 1]),
            },
        )),
        _ => None,
    };
    let mut encoder =
        device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
    {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });
        pass.set_pipeline(&pipeline);
        match typ {
            DrawType::Standard => pass.draw_mesh_tasks(1, 1, 1),
            DrawType::Indirect => pass.draw_mesh_tasks_indirect(buffer.as_ref().unwrap(), 0),
            DrawType::MultiIndirect => {
                pass.multi_draw_mesh_tasks_indirect(buffer.as_ref().unwrap(), 0, 1)
            }
            DrawType::MultiIndirectCount => pass.multi_draw_indexed_indirect_count(
                buffer.as_ref().unwrap(),
                0,
                count_buffer.as_ref().unwrap(),
                0,
                1,
            ),
        }
        pass.draw_mesh_tasks_indirect(buffer.as_ref().unwrap(), 0);
    }
    ctx.queue.submit(Some(encoder.finish()));
    ctx.device.poll(wgt::Maintain::wait());
}

const BASIC_TASK: &[u8] = include_bytes!("basic.task.spv");
const BASIC_MESH: &[u8] = include_bytes!("basic.mesh.spv");
//const BASIC_FRAG: &[u8] = include_bytes!("basic.frag.spv");
const NO_WRITE_FRAG: &[u8] = include_bytes!("no-write.frag.spv");

fn default_gpu_test_config() -> GpuTestConfiguration {
    GpuTestConfiguration::new().parameters(
        TestParameters::default()
            .test_features_limits()
            .features(wgpu::Features::MESH_SHADER | wgpu::Features::SPIRV_SHADER_PASSTHROUGH),
    )
}

// Mesh pipeline configs
#[gpu_test]
static MESH_PIPELINE_BASIC_MESH: GpuTestConfiguration = default_gpu_test_config().run_sync(|ctx| {
    mesh_pipeline_build(&ctx, None, BASIC_TASK, None, true);
});
#[gpu_test]
static MESH_PIPELINE_BASIC_TASK_MESH: GpuTestConfiguration =
    default_gpu_test_config().run_sync(|ctx| {
        mesh_pipeline_build(&ctx, Some(BASIC_TASK), BASIC_MESH, None, true);
    });
#[gpu_test]
static MESH_PIPELINE_BASIC_MESH_FRAG: GpuTestConfiguration =
    default_gpu_test_config().run_sync(|ctx| {
        mesh_pipeline_build(&ctx, None, BASIC_MESH, Some(NO_WRITE_FRAG), true);
    });
#[gpu_test]
static MESH_PIPELINE_BASIC_TASK_MESH_FRAG: GpuTestConfiguration = default_gpu_test_config()
    .run_sync(|ctx| {
        mesh_pipeline_build(
            &ctx,
            Some(BASIC_TASK),
            BASIC_MESH,
            Some(NO_WRITE_FRAG),
            true,
        );
    });

// Mesh draw
#[gpu_test]
static MESH_DRAW_INDIRECT: GpuTestConfiguration = default_gpu_test_config().run_sync(|ctx| {
    mesh_draw(&ctx, DrawType::Indirect);
});
#[gpu_test]
static MESH_MULTI_DRAW_INDIRECT: GpuTestConfiguration = default_gpu_test_config().run_sync(|ctx| {
    mesh_draw(&ctx, DrawType::MultiIndirect);
});
#[gpu_test]
static MESH_MULTI_DRAW_INDIRECT_COUNT: GpuTestConfiguration =
    default_gpu_test_config().run_sync(|ctx| {
        mesh_draw(&ctx, DrawType::MultiIndirectCount);
    });
