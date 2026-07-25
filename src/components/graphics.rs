use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, PointerEvent, Window};

const PARTICLE_DENSITY: f64 = 0.0005; // particles per square pixel
const PARTICLE_SIZE: f64 = 2.0;
const POINTER_RADIUS: f64 = 120.0;
const REPULSION_STRENGTH: f64 = 8.0;
const RETURN_SPEED: f64 = 0.03;
const DRIFT_X: f64 = 5.0;
const DRIFT_Y: f64 = 7.0;
const PARTICLE_COLOR: &str = "rgba(99, 102, 241, 0.6)";

#[derive(Clone)]
struct Particle {
    x: f64,
    y: f64,
    base_x: f64,
    base_y: f64,
    vx: f64,
    vy: f64,
}

impl Particle {
    fn new(x: f64, y: f64) -> Self {
        Self {
            x,
            y,
            base_x: x,
            base_y: y,
            vx: 0.0,
            vy: 0.0,
        }
    }

    fn update(&mut self, pointer_x: f64, pointer_y: f64, time: f64, reduced_motion: bool) {
        let dx = self.x - pointer_x;
        let dy = self.y - pointer_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if !reduced_motion && distance < POINTER_RADIUS && distance > 0.0 {
            let force = (POINTER_RADIUS - distance) / POINTER_RADIUS;
            let angle = dy.atan2(dx);
            self.vx += angle.cos() * force * REPULSION_STRENGTH;
            self.vy += angle.sin() * force * REPULSION_STRENGTH;
        }

        self.x += self.vx;
        self.y += self.vy;

        self.vx *= 0.92;
        self.vy *= 0.92;

        let (drift_x, drift_y) = if reduced_motion {
            (0.0, 0.0)
        } else {
            (
                (time * 0.00035 + self.base_y * 0.008).sin() * DRIFT_X,
                (time * 0.00028 + self.base_x * 0.007).cos() * DRIFT_Y,
            )
        };
        let dx_home = self.base_x + drift_x - self.x;
        let dy_home = self.base_y + drift_y - self.y;
        self.x += dx_home * RETURN_SPEED;
        self.y += dy_home * RETURN_SPEED;
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(
            self.x,
            self.y,
            PARTICLE_SIZE,
            0.0,
            std::f64::consts::PI * 2.0,
        )
        .unwrap();
        ctx.fill();
    }
}

#[derive(Clone)]
struct CanvasState {
    width: f64,
    height: f64,
    needs_resize: bool,
}

#[component]
pub fn GraphicsCanvas() -> impl IntoView {
    let canvas_ref = create_node_ref::<leptos::html::Canvas>();

    create_effect(move |_| {
        if let Some(canvas) = canvas_ref.get() {
            let canvas: HtmlCanvasElement = (*canvas).clone().unchecked_into();
            start_animation(canvas);
        }
    });

    view! {
        <div class="canvas-container">
            <canvas id="graphics-canvas" node_ref=canvas_ref></canvas>
        </div>
    }
}

fn setup_canvas(canvas: &HtmlCanvasElement, ctx: &CanvasRenderingContext2d) -> (f64, f64) {
    let window = web_sys::window().unwrap();
    let dpr = window.device_pixel_ratio().min(2.0);
    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    canvas.set_width((width * dpr) as u32);
    canvas.set_height((height * dpr) as u32);
    canvas
        .style()
        .set_property("width", &format!("{}px", width))
        .unwrap();
    canvas
        .style()
        .set_property("height", &format!("{}px", height))
        .unwrap();

    ctx.set_transform(1.0, 0.0, 0.0, 1.0, 0.0, 0.0).unwrap();
    ctx.scale(dpr, dpr).unwrap();

    (width, height)
}

fn start_animation(canvas: HtmlCanvasElement) {
    let window = web_sys::window().expect("no window");
    let dpr = window.device_pixel_ratio().min(2.0);
    let reduced_motion = window
        .match_media("(prefers-reduced-motion: reduce)")
        .ok()
        .flatten()
        .map(|query| query.matches())
        .unwrap_or(false);

    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    canvas.set_width((width * dpr) as u32);
    canvas.set_height((height * dpr) as u32);
    canvas
        .style()
        .set_property("width", &format!("{}px", width))
        .unwrap();
    canvas
        .style()
        .set_property("height", &format!("{}px", height))
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.scale(dpr, dpr).unwrap();

    let canvas_state = Rc::new(RefCell::new(CanvasState {
        width,
        height,
        needs_resize: false,
    }));

    let particles = Rc::new(RefCell::new(create_particles(width, height)));
    let pointer_pos = Rc::new(RefCell::new((-1000.0, -1000.0)));

    let document = window.document().unwrap();

    let pointer_pos_move = pointer_pos.clone();
    let pointer_closure = Closure::<dyn FnMut(PointerEvent)>::new(move |event: PointerEvent| {
        let mut pos = pointer_pos_move.borrow_mut();
        pos.0 = event.client_x() as f64;
        pos.1 = event.client_y() as f64;
    });

    document
        .add_event_listener_with_callback("pointermove", pointer_closure.as_ref().unchecked_ref())
        .unwrap();
    pointer_closure.forget();

    let pointer_pos_reset = pointer_pos.clone();
    let reset_closure = Closure::<dyn FnMut()>::new(move || {
        let mut pos = pointer_pos_reset.borrow_mut();
        pos.0 = -1000.0;
        pos.1 = -1000.0;
    });

    document
        .add_event_listener_with_callback("pointercancel", reset_closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .add_event_listener_with_callback("pointerup", reset_closure.as_ref().unchecked_ref())
        .unwrap();
    document
        .add_event_listener_with_callback("pointerleave", reset_closure.as_ref().unchecked_ref())
        .unwrap();
    reset_closure.forget();

    // Resize handler - just mark that resize is needed
    let canvas_state_resize = canvas_state.clone();
    let resize_closure = Closure::<dyn FnMut()>::new(move || {
        let mut state = canvas_state_resize.borrow_mut();
        state.needs_resize = true;
    });

    window
        .add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref())
        .unwrap();
    resize_closure.forget();

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let canvas_clone = canvas.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let window = web_sys::window().unwrap();

        // Check if resize is needed
        {
            let mut state = canvas_state.borrow_mut();
            if state.needs_resize {
                let (new_width, new_height) = setup_canvas(&canvas_clone, &ctx);
                state.width = new_width;
                state.height = new_height;
                state.needs_resize = false;

                // Recreate particles for new dimensions
                let mut p = particles.borrow_mut();
                *p = create_particles(new_width, new_height);
            }
        }

        let state = canvas_state.borrow();
        let width = state.width;
        let height = state.height;
        drop(state);

        ctx.clear_rect(0.0, 0.0, width, height);

        let (pointer_x, pointer_y) = *pointer_pos.borrow();
        let time = js_sys::Date::now();

        ctx.set_fill_style_str(PARTICLE_COLOR);

        let mut particles = particles.borrow_mut();
        for particle in particles.iter_mut() {
            particle.update(pointer_x, pointer_y, time, reduced_motion);
            particle.draw(&ctx);
        }

        request_animation_frame(window, f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(window, g.borrow().as_ref().unwrap());
}

fn request_animation_frame(window: Window, closure: &Closure<dyn FnMut()>) {
    window
        .request_animation_frame(closure.as_ref().unchecked_ref())
        .unwrap();
}

fn create_particles(width: f64, height: f64) -> Vec<Particle> {
    let area = width * height;
    let particle_count = (area * PARTICLE_DENSITY).round() as usize;
    let particle_count = particle_count.max(50); // minimum 50 particles

    let mut particles = Vec::with_capacity(particle_count);
    let cols = (particle_count as f64).sqrt().ceil() as usize;
    let rows = particle_count.div_ceil(cols);

    let spacing_x = width / (cols + 1) as f64;
    let spacing_y = height / (rows + 1) as f64;

    for i in 0..particle_count {
        let col = i % cols;
        let row = i / cols;
        let x = spacing_x * (col + 1) as f64 + (js_sys::Math::random() - 0.5) * 20.0;
        let y = spacing_y * (row + 1) as f64 + (js_sys::Math::random() - 0.5) * 20.0;
        particles.push(Particle::new(x, y));
    }

    particles
}
