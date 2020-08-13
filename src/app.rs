use js_sys::Math;
use std::time::Duration;
use web_sys::HtmlAudioElement;
use yew::services::interval::{IntervalService, IntervalTask};
use yew::{html, Component, ComponentLink, Html, ShouldRender};

const MERGIN: u8 = 10;
const TIME_INTERVAL: f64 = 0.01;
const GAME_DURATION: f64 = 10.0;

fn clamp(val: u8, min: u8, max: u8) -> u8 {
    if val < min {
        min
    } else if val > max {
        max
    } else {
        val
    }
}

struct Pos {
    x: u8,
    y: u8,
}

impl Pos {
    fn new_random() -> Self {
        let x = clamp(((Math::random() * 100.0) as u8), MERGIN, 100 - MERGIN);
        let y = clamp(((Math::random() * 100.0) as u8), MERGIN, 100 - MERGIN);

        Self { x, y }
    }

    fn to_style(&self) -> String {
        assert!(self.x <= 100 && self.y <= 100);
        format!("top:{}vh;left:{}vw", self.y, self.x)
    }
}

pub struct App {
    link: ComponentLink<Self>,
    pos: Pos,
    score: u64,
    audio: HtmlAudioElement,
    is_started: bool,
    is_clickable: bool,
    time: f64,
    interval_task: Option<IntervalTask>,
}

#[derive(Debug)]
pub enum AppMessage {
    TargetClick,
    OnInterval,
    Restart,
}

impl Component for App {
    type Message = AppMessage;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let audio =
            HtmlAudioElement::new_with_src("./isso_daipan.wav").expect("failed to get daipan.");

        App {
            link,
            pos: Pos::new_random(),
            score: 0,
            audio,
            is_started: false,
            is_clickable: true,
            time: 0.0,
            interval_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::info!("event: {:?}", msg);
        match msg {
            AppMessage::TargetClick => {
                if !self.is_clickable {
                    return false;
                }

                if !self.is_started && self.is_clickable {
                    let duration = Duration::from_secs_f64(TIME_INTERVAL);
                    let callback = self.link.callback(|_| AppMessage::OnInterval);
                    let task = IntervalService::spawn(duration, callback);
                    self.interval_task = Some(task);
                    self.is_started = true;
                }

                self.pos = Pos::new_random();
                self.score += 1;
                self.audio.set_current_time(0.0);

                let _ = self.audio.play().expect("failed to play audio");

                true
            }

            AppMessage::OnInterval => {
                self.time += TIME_INTERVAL;

                if self.time >= GAME_DURATION {
                    self.interval_task = None;
                    self.time = GAME_DURATION;
                    self.is_started = false;
                    self.is_clickable = false;
                }

                true
            }

            AppMessage::Restart => {
                self.time = 0.0;
                self.score = 0;
                self.is_clickable = true;
                self.is_started = false;
                true
            }
        }
    }

    fn change(&mut self, _new_prop: Self::Properties) -> bool {
        false
    }

    fn view(&self) -> Html {
        html! {
            <>
                <img
                    src="ff.png"
                    class="target"
                    draggable=false
                    style=self.pos.to_style()
                    onclick=self.link.callback(|_| AppMessage::TargetClick)
                />
                <div class="score">{format!("Score: {}", self.score)}</div>
                <div class="timer">{format!("Time: {:.2}", self.time)}</div>
               <input
                   type="button"
                   value="restart"
                   disabled=self.is_started || self.is_clickable
                   onclick=self.link.callback(|_| AppMessage::Restart)
               />
            </>
        }
    }
}
