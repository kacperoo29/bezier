mod bezier_curve;
mod vec;

use num::Float;
use rand::Rng;
use wasm_bindgen::JsCast;
use yew::prelude::*;

use bezier_curve::BezierCurve;

fn get_num_value(event: InputEvent) -> f32 {
    let target = event.target().unwrap();
    let input = target.dyn_into::<web_sys::HtmlInputElement>().unwrap();
    input.value_as_number() as f32
}

struct App {
    bezier_curve: BezierCurve,
    canvas: NodeRef,
    canvas_ctx: Option<web_sys::CanvasRenderingContext2d>,
    dragging_point: Option<usize>,
}

enum Msg {
    MouseDown(MouseEvent),
    MouseUp(MouseEvent),
    MouseMove(MouseEvent),
    AddPoint,
    ChangeX((usize, f32)),
    ChangeY((usize, f32)),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            bezier_curve: BezierCurve::new(),
            canvas: NodeRef::default(),
            canvas_ctx: None,
            dragging_point: None,
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                <canvas style="border: 1px solid black"
                    ref={self.canvas.clone()} width="800" height="600"
                    onmousedown={_ctx.link().callback(Msg::MouseDown)}
                    onmouseup={_ctx.link().callback(Msg::MouseUp)}
                    onmousemove={_ctx.link().callback(Msg::MouseMove)} />
                <div>
                    <button onclick={_ctx.link().callback(|_| Msg::AddPoint)}>
                        {"Add point"}
                    </button>
                    {for self.bezier_curve.points().iter().enumerate().map(|(i, p)| {
                        html! {
                            <div>
                                <h4>{format!("Point {}: ", i)}</h4>
                                <div>
                                    <label>{"X: "}</label>
                                    <input
                                        type="number"
                                        value={p.x().to_string()}
                                        oninput={_ctx.link().callback(move |e: InputEvent| Msg::ChangeX((i, get_num_value(e))))} />
                                </div>
                                <div>
                                    <label>{"Y: "}</label>
                                    <input
                                        type="number"
                                        value={p.y().to_string()}
                                        oninput={_ctx.link().callback(move |e: InputEvent| Msg::ChangeY((i, get_num_value(e))))} />
                                </div>
                            </div>
                        }
                    })}
                </div>
            </div>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let canvas = self.canvas.cast::<web_sys::HtmlCanvasElement>().unwrap();
            let context = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<web_sys::CanvasRenderingContext2d>()
                .unwrap();
            self.canvas_ctx = Some(context);
        }

        if self.bezier_curve.points().len() < 1 {
            return;
        }

        let Some(context) = self.canvas_ctx.as_ref() else {
            return;
        };

        context.clear_rect(0.0, 0.0, 800.0, 600.0);
        context.set_fill_style(&"red".into());
        for point in self.bezier_curve.points() {
            context.fill_rect(point.x() as f64 - 5.0, point.y() as f64 - 5.0, 10.0, 10.0);
        }

        if self.bezier_curve.points().len() < 2 {
            return;
        }

        context.set_stroke_style(&"black".into());
        context.set_line_width(2.0);
        context.begin_path();

        let origin = self.bezier_curve.points()[0];
        context.move_to(origin.x() as f64, origin.y() as f64);

        // TODO: Calculate the number of points needed to draw the curve
        let number_of_points = self.bezier_curve.points().len() * 10;
        for t in 1..=number_of_points {
            let point = self
                .bezier_curve
                .calculate_point(t as f32 / number_of_points as f32);
            context.line_to(point.x() as f64, point.y() as f64);
        }

        context.stroke();
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseDown(event) => {
                let point =
                    vec::Vector2f::new_with_data(event.offset_x() as f32, event.offset_y() as f32);

                if let Some(index) = self.bezier_curve.intersect_with_control_point(point) {
                    self.dragging_point = Some(index);
                } else {
                    self.bezier_curve.add_point(point);
                }

                true
            }
            Msg::MouseUp(_) => {
                self.dragging_point = None;
                true
            }
            Msg::MouseMove(event) => {
                if let Some(index) = self.dragging_point {
                    let point = vec::Vector2f::new_with_data(
                        event.offset_x() as f32,
                        event.offset_y() as f32,
                    );
                    self.bezier_curve.set_point(index, point);
                }

                true
            }
            Msg::AddPoint => {
                let mut rng = rand::thread_rng();
                let x = rng.gen_range(0.0..800.0).ceil();
                let y = rng.gen_range(0.0..600.0).ceil();

                self.bezier_curve
                    .add_point(vec::Vector2f::new_with_data(x, y));
                true
            }
            Msg::ChangeX((index, x)) => {
                if x < 0.0 || x > 800.0 {
                    return false;
                }

                let mut point = self.bezier_curve.points()[index];
                point.set_x(x);
                self.bezier_curve.set_point(index, point);
                true
            }
            Msg::ChangeY((index, y)) => {
                if y < 0.0 || y > 600.0 {
                    return false;
                }

                let mut point = self.bezier_curve.points()[index];
                point.set_y(y);
                self.bezier_curve.set_point(index, point);
                true
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
