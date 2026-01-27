use leptos::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, Window};
use std::f64::consts::PI;
use std::cell::RefCell;
use std::rc::Rc;

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

    let shapes = create_shapes(width, height);
    let time = Rc::new(RefCell::new(0.0_f64));

    let f = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));
    let g = f.clone();

    let canvas_clone = canvas.clone();
    *g.borrow_mut() = Some(Closure::new(move || {
        let window = web_sys::window().unwrap();
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();

        ctx.set_fill_style(&JsValue::from_str("rgba(10, 10, 15, 0.15)"));
        ctx.fill_rect(0.0, 0.0, width, height);

        let mut t = time.borrow_mut();
        *t += 0.008;

        for shape in &shapes {
            draw_shape(&ctx, shape, *t, width, height);
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

struct Shape {
    x_ratio: f64,
    y_ratio: f64,
    size: f64,
    rotation_speed: f64,
    orbit_radius: f64,
    orbit_speed: f64,
    sides: u32,
    color: String,
    phase: f64,
}

fn create_shapes(_width: f64, _height: f64) -> Vec<Shape> {
    vec![
        Shape {
            x_ratio: 0.7,
            y_ratio: 0.3,
            size: 80.0,
            rotation_speed: 0.5,
            orbit_radius: 30.0,
            orbit_speed: 0.3,
            sides: 6,
            color: "rgba(99, 102, 241, 0.3)".to_string(),
            phase: 0.0,
        },
        Shape {
            x_ratio: 0.2,
            y_ratio: 0.6,
            size: 60.0,
            rotation_speed: -0.7,
            orbit_radius: 20.0,
            orbit_speed: 0.4,
            sides: 4,
            color: "rgba(129, 140, 248, 0.25)".to_string(),
            phase: PI / 3.0,
        },
        Shape {
            x_ratio: 0.8,
            y_ratio: 0.7,
            size: 50.0,
            rotation_speed: 0.6,
            orbit_radius: 25.0,
            orbit_speed: -0.35,
            sides: 3,
            color: "rgba(99, 102, 241, 0.2)".to_string(),
            phase: PI / 2.0,
        },
        Shape {
            x_ratio: 0.3,
            y_ratio: 0.2,
            size: 40.0,
            rotation_speed: -0.4,
            orbit_radius: 15.0,
            orbit_speed: 0.5,
            sides: 5,
            color: "rgba(165, 180, 252, 0.2)".to_string(),
            phase: PI,
        },
        Shape {
            x_ratio: 0.5,
            y_ratio: 0.85,
            size: 70.0,
            rotation_speed: 0.3,
            orbit_radius: 35.0,
            orbit_speed: -0.25,
            sides: 8,
            color: "rgba(99, 102, 241, 0.15)".to_string(),
            phase: PI * 1.5,
        },
    ]
}

fn draw_shape(ctx: &CanvasRenderingContext2d, shape: &Shape, time: f64, width: f64, height: f64) {
    let base_x = shape.x_ratio * width;
    let base_y = shape.y_ratio * height;

    let x = base_x + (time * shape.orbit_speed + shape.phase).cos() * shape.orbit_radius;
    let y = base_y + (time * shape.orbit_speed + shape.phase).sin() * shape.orbit_radius;
    let rotation = time * shape.rotation_speed;

    ctx.save();
    ctx.translate(x, y).unwrap();
    ctx.rotate(rotation).unwrap();

    ctx.begin_path();

    for i in 0..=shape.sides {
        let angle = (i as f64) * 2.0 * PI / (shape.sides as f64) - PI / 2.0;
        let px = angle.cos() * shape.size;
        let py = angle.sin() * shape.size;

        if i == 0 {
            ctx.move_to(px, py);
        } else {
            ctx.line_to(px, py);
        }
    }

    ctx.close_path();
    ctx.set_stroke_style(&JsValue::from_str(&shape.color));
    ctx.set_line_width(2.0);
    ctx.stroke();

    ctx.restore();
}
