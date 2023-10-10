use std::cell::RefCell;
use std::rc::Rc;

use wasm_bindgen::prelude::*;
use web_sys::{
    window, HtmlCanvasElement, WebGl2RenderingContext as GL, WebGl2RenderingContext, WebGlBuffer,
    WebGlProgram, WebGlShader, WebGlVertexArrayObject,
};
use yew::prelude::*;

// Wrap gl in Rc (Arc for multi-threaded) so it can be injected into the render-loop closure.
pub struct WebGLPage {
    node_ref: NodeRef,
    program: Option<WebGlProgram>,
    buffer: Option<WebGlBuffer>,
    vao: Option<WebGlVertexArrayObject>,
}

impl Component for WebGLPage {
    type Message = ();
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            node_ref: NodeRef::default(),
            program: None,
            buffer: None,
            vao: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <canvas ref={self.node_ref.clone()} width="500" height="500" />
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        // Only start the render loop if it's the first render
        // There's no loop cancellation taking place, so if multiple renders happen,
        // there would be multiple loops running. That doesn't *really* matter here because
        // there's no props update and no SSR is taking place, but it is something to keep in
        // consideration
        if !first_render {
            return;
        }
        // Once rendered, store references for the canvas and GL context. These can be used for
        // resizing the rendering area when the window or canvas element are resized, as well as
        // for making GL calls.
        let canvas = self.node_ref.cast::<HtmlCanvasElement>().unwrap();
        let context = canvas
            .get_context("webgl2")
            .unwrap()
            .unwrap()
            .dyn_into::<WebGl2RenderingContext>()
            .unwrap();
        self.init_gl(&context);
        self.render_gl(context);
    }
}

impl WebGLPage {
    fn request_animation_frame(f: &Closure<dyn FnMut()>) {
        window()
            .unwrap()
            .request_animation_frame(f.as_ref().unchecked_ref())
            .expect("should register `requestAnimationFrame` OK");
    }

    fn init_gl(&mut self, context: &WebGl2RenderingContext) {
        let vert_shader = Self::compile_shader(
            context,
            WebGl2RenderingContext::VERTEX_SHADER,
            r##"#version 300 es
     
            in vec4 position;
    
            void main() {
            
                gl_Position = position;
            }
            "##,
        )
        .unwrap();

        let frag_shader = Self::compile_shader(
            context,
            WebGl2RenderingContext::FRAGMENT_SHADER,
            r##"#version 300 es
        
            precision highp float;
            out vec4 outColor;
            
            void main() {
                outColor = vec4(1, 1, 1, 1);
            }
            "##,
        )
        .unwrap();

        self.program = Self::link_program(context, &vert_shader, &frag_shader).unwrap();

        let buffer = context
            .create_buffer()
            .ok_or("Failed to create buffer")
            .unwrap();
        self.buffer = Some(buffer);

        let vao = context
            .create_vertex_array()
            .ok_or("Could not create vertex array object")
            .unwrap();
        self.vao = Some(vao);

        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.buffer.as_ref());
        context.bind_vertex_array(self.vao.as_ref());

        // Note that `Float32Array::view` is somewhat dangerous (hence the
        // `unsafe`!). This is creating a raw view into our module's
        // `WebAssembly.Memory` buffer, but if we allocate more pages for ourself
        // (aka do a memory allocation in Rust) it'll cause the buffer to change,
        // causing the `Float32Array` to be invalid.
        //
        // As a result, after `Float32Array::view` we have to be very careful not to
        // do any memory allocations before it's dropped.
        let vertices: [f32; 9] = [-0.7, -0.7, 0.0, 0.7, -0.7, 0.0, 0.0, 0.7, 0.0];
        unsafe {
            let positions_array_buf_view = js_sys::Float32Array::view(&vertices);

            context.buffer_data_with_array_buffer_view(
                WebGl2RenderingContext::ARRAY_BUFFER,
                &positions_array_buf_view,
                WebGl2RenderingContext::STATIC_DRAW,
            );
        }

        // Unbind buffers and arrays
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
        context.bind_vertex_array(None);
    }

    fn render_gl(&self, context: WebGl2RenderingContext) {
        self.draw(&context);

        // Gloo-render's request_animation_frame has this extra closure
        // wrapping logic running every frame, unnecessary cost.
        // Here constructing the wrapped closure just once.

        let cb = Rc::new(RefCell::new(None));

        *cb.borrow_mut() = Some(Closure::wrap(Box::new({
            let cb = cb.clone();
            move || {
                // This should repeat every frame
                //timestamp += 20.0;
                //gl.uniform1f(time.as_ref(), timestamp as f32);
                //gl.draw_arrays(GL::TRIANGLES, 0, 6);
                WebGLPage::request_animation_frame(cb.borrow().as_ref().unwrap());
            }
        }) as Box<dyn FnMut()>));

        WebGLPage::request_animation_frame(cb.borrow().as_ref().unwrap());
    }

    fn draw(&self, context: &WebGl2RenderingContext) {
        // 1. Bind buffers
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, self.buffer.as_ref());

        // 2. Set the shader program
        context.use_program(self.program.as_ref());
        let position_attribute_location =
            context.get_attrib_location(self.program.clone().unwrap().as_ref(), "position");

        context.vertex_attrib_pointer_with_i32(
            position_attribute_location as u32,
            3,
            WebGl2RenderingContext::FLOAT,
            false,
            0,
            0,
        );
        context.enable_vertex_attrib_array(position_attribute_location as u32);

        // 3. Configure
        context.clear_color(0.0, 0.0, 0.0, 1.0);
        context.clear(WebGl2RenderingContext::COLOR_BUFFER_BIT);

        // 4. Draw
        let vert_count = (/*vertices.len()*/9 / 3) as i32;
        context.draw_arrays(WebGl2RenderingContext::TRIANGLES, 0, vert_count);

        // 5. Cleanup draw
        context.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, None);
    }

    fn compile_shader(
        context: &WebGl2RenderingContext,
        shader_type: u32,
        source: &str,
    ) -> Result<WebGlShader, String> {
        let shader = context
            .create_shader(shader_type)
            .ok_or_else(|| String::from("Unable to create shader object"))?;
        context.shader_source(&shader, source);
        context.compile_shader(&shader);

        if context
            .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(shader)
        } else {
            Err(context
                .get_shader_info_log(&shader)
                .unwrap_or_else(|| String::from("Unknown error creating shader")))
        }
    }

    fn link_program(
        context: &WebGl2RenderingContext,
        vert_shader: &WebGlShader,
        frag_shader: &WebGlShader,
    ) -> Result<Option<WebGlProgram>, String> {
        let program = context
            .create_program()
            .ok_or_else(|| String::from("Unable to create shader object"))?;

        context.attach_shader(&program, vert_shader);
        context.attach_shader(&program, frag_shader);
        context.link_program(&program);

        if context
            .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
            .as_bool()
            .unwrap_or(false)
        {
            Ok(Some(program))
        } else {
            Err(context
                .get_program_info_log(&program)
                .unwrap_or_else(|| String::from("Unknown error creating program object")))
        }
    }
}
