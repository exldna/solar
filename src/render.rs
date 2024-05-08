use crate::shader;
use crate::space::SpaceView;
use glam::{Mat4, Vec4};
use miniquad::*;

const MAX_BODIES_COUNT: usize = 100;
const MAX_TRACK_LENGTH: usize = 1000;

const CIRCLE_RADIUS: f32 = 2.;
const CIRCLE_QUALITY: usize = 12;
const CIRCLE_ASPECT: f32 = std::f32::consts::PI * 2. / CIRCLE_QUALITY as f32;
const CIRCLE_INDICES_COUNT: usize = (CIRCLE_QUALITY - 2) * 3;

type Context = Box<dyn RenderingBackend>;

pub struct Render {
    ctx: Context,
    proj: Mat4,
    bodies_render: BodiesRender,
    tracks_render: TracksRender,
}

impl Render {
    pub fn new() -> Self {
        let mut ctx: Context = window::new_rendering_backend();
        let bodies_render = BodiesRender::new(&mut ctx);
        let tracks_render = TracksRender::new(&mut ctx);
        Self {
            ctx,
            proj: projection_matrix(),
            bodies_render,
            tracks_render,
        }
    }

    pub fn draw(&mut self, space_view: &SpaceView) {
        self.ctx.begin_default_pass(PassAction::default());

        let view_proj = self.proj * *space_view.view_matrix();
        self.bodies_render
            .draw(&mut self.ctx, space_view.instances(), &view_proj);
        self.tracks_render
            .draw(&mut self.ctx, space_view.tracks(), &view_proj);

        self.ctx.end_render_pass();
        self.ctx.commit_frame();
    }
}

fn projection_matrix() -> Mat4 {
    let screen_size = window::screen_size();
    let (half_width, half_height) = (screen_size.0 / 2., screen_size.1 / 2.);
    Mat4::orthographic_rh_gl(-half_width, half_width, -half_height, half_height, 0., 1.)
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct BodyRenderVertex {
    pub body_pos: [f32; 2],
    pub color: [f32; 4],
}

struct BodiesRender {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl BodiesRender {
    fn new(ctx: &mut Context) -> Self {
        let circle_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&circle_vertices()),
        );

        let circle_index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&circle_indices()),
        );

        let bodies_instance_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::empty::<BodyRenderVertex>(MAX_BODIES_COUNT),
        );

        let bindings = Bindings {
            vertex_buffers: vec![circle_vertex_buffer, bodies_instance_buffer],
            index_buffer: circle_index_buffer,
            images: vec![],
        };

        let shader_source = shader::bodies_render::source(ctx.info().backend);
        let shader_meta = shader::bodies_render::meta();
        let shader = ctx.new_shader(shader_source, shader_meta).unwrap();

        let pipeline = ctx.new_pipeline(
            &[
                BufferLayout::default(),
                BufferLayout {
                    step_func: VertexStep::PerInstance,
                    ..Default::default()
                },
            ],
            &[
                VertexAttribute::with_buffer("in_vert_pos", VertexFormat::Float2, 0),
                VertexAttribute::with_buffer("in_body_pos", VertexFormat::Float2, 1),
                VertexAttribute::with_buffer("in_color", VertexFormat::Float4, 1),
            ],
            shader,
            PipelineParams::default(),
        );

        Self { pipeline, bindings }
    }

    fn draw(&mut self, ctx: &mut Context, bodies: &[BodyRenderVertex], mvp: &Mat4) {
        let buffer = self.bindings.vertex_buffers[1];
        ctx.buffer_update(buffer, BufferSource::slice(bodies));

        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let uniforms = shader::bodies_render::Uniforms::new(*mvp);
        ctx.apply_uniforms(UniformsSource::table(&uniforms));

        ctx.draw(0, CIRCLE_INDICES_COUNT as i32, bodies.len() as i32);
    }
}

fn circle_vertices() -> Vec<glam::Vec2> {
    let mut vertices = Vec::with_capacity(CIRCLE_QUALITY);
    for i in 0..CIRCLE_QUALITY {
        let angel = i as f32 * CIRCLE_ASPECT;
        vertices.push(glam::Vec2::new(
            angel.cos() * CIRCLE_RADIUS,
            angel.sin() * CIRCLE_RADIUS,
        ));
    }
    vertices
}

fn circle_indices() -> [u8; CIRCLE_INDICES_COUNT] {
    let mut indices: [u8; CIRCLE_INDICES_COUNT] = [0; CIRCLE_INDICES_COUNT];
    for i in 0..CIRCLE_QUALITY - 2 {
        indices[i * 3 + 1] = (i + 1) as u8;
        indices[i * 3 + 2] = (i + 2) as u8;
    }
    indices
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TrackRenderVertex {
    pub pos: [f32; 2],
}

struct TracksRender {
    pipeline: Pipeline,
    bindings: Bindings,
}

impl TracksRender {
    fn new(ctx: &mut Context) -> Self {
        let track_vertex_buffer = ctx.new_buffer(
            BufferType::VertexBuffer,
            BufferUsage::Dynamic,
            BufferSource::empty::<TrackRenderVertex>(MAX_TRACK_LENGTH),
        );

        let mut indices = [0u16; MAX_TRACK_LENGTH * 2];
        for i in 0..MAX_TRACK_LENGTH {
            indices[i * 2] = i as u16;
            indices[i * 2 + 1] = i as u16 + 1;
        }

        let track_index_buffer = ctx.new_buffer(
            BufferType::IndexBuffer,
            BufferUsage::Immutable,
            BufferSource::slice(&indices),
        );

        let bindings = Bindings {
            vertex_buffers: vec![track_vertex_buffer],
            index_buffer: track_index_buffer,
            images: vec![],
        };

        let shader_source = shader::tracks_render::source(ctx.info().backend);
        let shader_meta = shader::tracks_render::meta();
        let shader = ctx.new_shader(shader_source, shader_meta).unwrap();

        let pipeline = ctx.new_pipeline(
            &[BufferLayout::default()],
            &[VertexAttribute::with_buffer(
                "in_pos",
                VertexFormat::Float2,
                0,
            )],
            shader,
            PipelineParams {
                primitive_type: PrimitiveType::Lines,
                ..PipelineParams::default()
            },
        );

        Self { pipeline, bindings }
    }

    fn draw(&mut self, ctx: &mut Context, tracks: &[Vec<TrackRenderVertex>], mvp: &Mat4) {
        ctx.apply_pipeline(&self.pipeline);
        ctx.apply_bindings(&self.bindings);

        let uniforms = shader::tracks_render::Uniforms::new(*mvp, Vec4::ONE);
        ctx.apply_uniforms(UniformsSource::table(&uniforms));

        for track in tracks.iter() {
            let left = 0.max(track.len() as isize - MAX_TRACK_LENGTH as isize) as usize;
            ctx.buffer_update(
                self.bindings.vertex_buffers[0],
                BufferSource::slice(&track[left..]),
            );
            let num_elements = (track.len() - left) as i32 * 2 - 1;
            ctx.draw(0, num_elements, track.len() as i32);
        }
    }
}
