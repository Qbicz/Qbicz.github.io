use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent, Window};
use std::cell::RefCell;
use std::rc::Rc;

const PARTICLE_DENSITY: f64 = 0.0005; // particles per square pixel
const PARTICLE_SIZE: f64 = 2.0;
const MOUSE_RADIUS: f64 = 120.0;
const REPULSION_STRENGTH: f64 = 8.0;
const RETURN_SPEED: f64 = 0.03;
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

    fn update(&mut self, mouse_x: f64, mouse_y: f64) {
        let dx = self.x - mouse_x;
        let dy = self.y - mouse_y;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance < MOUSE_RADIUS && distance > 0.0 {
            let force = (MOUSE_RADIUS - distance) / MOUSE_RADIUS;
            let angle = dy.atan2(dx);
            self.vx += angle.cos() * force * REPULSION_STRENGTH;
            self.vy += angle.sin() * force * REPULSION_STRENGTH;
        }

        self.x += self.vx;
        self.y += self.vy;

        self.vx *= 0.92;
        self.vy *= 0.92;

        let dx_home = self.base_x - self.x;
        let dy_home = self.base_y - self.y;
        self.x += dx_home * RETURN_SPEED;
        self.y += dy_home * RETURN_SPEED;
    }

    fn draw(&self, ctx: &CanvasRenderingContext2d) {
        ctx.begin_path();
        ctx.arc(self.x, self.y, PARTICLE_SIZE, 0.0, std::f64::consts::PI * 2.0).unwrap();
        ctx.fill();
    }
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

fn start_animation(canvas: HtmlCanvasElement) {
    let window = web_sys::window().expect("no window");
    let dpr = window.device_pixel_ratio();

    let width = window.inner_width().unwrap().as_f64().unwrap();
    let height = window.inner_height().unwrap().as_f64().unwrap();

    canvas.set_width((width * dpr) as u32);
    canvas.set_height((height * dpr) as u32);
    canvas.style().set_property("width", &format!("{}px", width)).unwrap();
    canvas.style().set_property("height", &format!("{}px", height)).unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()
        .unwrap();

    ctx.scale(dpr, dpr).unwrap();

    let particles = Rc::new(RefCell::new(create_particles(width, height)));
    let mouse_pos = Rc::new(RefCell::new((-1000.0, -1000.0)));

    let document = window.document().unwrap();

    let mouse_pos_clone = mouse_pos.clone();
    let mouse_closure = Closure::<dyn FnMut(MouseEvent)>::new(move |event: MouseEvent| {
        let mut pos = mouse_pos_clone.borrow_mut();
        pos.0 = event.client_x() as f64;
        pos.1 = event.client_y() as f64;
    });

    document.add_event_listener_with_callback("mousemove", mouse_closure.as_ref().unchecked_ref()).unwrap();
    mouse_closure.forget();

    let mouse_pos_leave = mouse_pos.clone();
    let leave_closure = Closure::<dyn FnMut()>::new(move || {
        let mut pos = mouse_pos_leave.borrow_mut();
        pos.0 = -1000.0;
        pos.1 = -1000.0;
    });

    document.add_event_listener_with_callback("mouseleave", leave_closure.as_ref().unchecked_ref()).unwrap();
    leave_closure.forget();

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let canvas_clone = canvas.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();

        ctx.set_fill_style(&JsValue::from_str("rgba(10, 10, 15, 1.0)"));
        ctx.fill_rect(0.0, 0.0, width, height);

        let (mouse_x, mouse_y) = *mouse_pos.borrow();

        ctx.set_fill_style(&JsValue::from_str(PARTICLE_COLOR));

        let mut particles = particles.borrow_mut();
        for particle in particles.iter_mut() {
            particle.update(mouse_x, mouse_y);
            particle.draw(&ctx);
        }

        request_animation_frame(window, f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(window, g.borrow().as_ref().unwrap());

    let window = web_sys::window().unwrap();
    let canvas_for_resize = canvas_clone.clone();
    let resize_closure = Closure::<dyn FnMut()>::new(move || {
        let window = web_sys::window().unwrap();
        let dpr = window.device_pixel_ratio();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();

        canvas_for_resize.set_width((width * dpr) as u32);
        canvas_for_resize.set_height((height * dpr) as u32);
        canvas_for_resize.style().set_property("width", &format!("{}px", width)).unwrap();
        canvas_for_resize.style().set_property("height", &format!("{}px", height)).unwrap();
    });

    window.add_event_listener_with_callback("resize", resize_closure.as_ref().unchecked_ref()).unwrap();
    resize_closure.forget();
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
    let rows = (particle_count + cols - 1) / cols;

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
