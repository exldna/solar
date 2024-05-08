pub mod bodies_render {
    use miniquad::*;

    pub fn source(backend: Backend) -> ShaderSource<'static> {
        const VERTEX: &str = include_str!("shaders/bodies_render.vert");
        const FRAGMENT: &str = include_str!("shaders/bodies_render.frag");

        match backend {
            Backend::OpenGl => ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            Backend::Metal => panic!("Metal not supported: write the shader"),
        }
    }

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![UniformDesc::new("mvp", UniformType::Mat4)],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        mvp: glam::Mat4,
    }

    impl Uniforms {
        pub fn new(mvp: glam::Mat4) -> Self {
            Self { mvp }
        }
    }
}

pub mod tracks_render {
    use miniquad::*;

    pub fn source(backend: Backend) -> ShaderSource<'static> {
        const VERTEX: &str = include_str!("shaders/tracks_render.vert");
        const FRAGMENT: &str = include_str!("shaders/tracks_render.frag");

        match backend {
            Backend::OpenGl => ShaderSource::Glsl {
                vertex: VERTEX,
                fragment: FRAGMENT,
            },
            Backend::Metal => panic!("Metal not supported: write the shader"),
        }
    }

    pub fn meta() -> ShaderMeta {
        ShaderMeta {
            images: vec![],
            uniforms: UniformBlockLayout {
                uniforms: vec![
                    UniformDesc::new("mvp", UniformType::Mat4),
                    UniformDesc::new("in_color", UniformType::Float4),
                ],
            },
        }
    }

    #[repr(C)]
    pub struct Uniforms {
        mvp: glam::Mat4,
        color: glam::Vec4,
    }

    impl Uniforms {
        pub fn new(mvp: glam::Mat4, color: glam::Vec4) -> Self {
            Self { mvp, color }
        }
    }
}
