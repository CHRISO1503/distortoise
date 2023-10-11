use nih_plug::nih_dbg;
use nih_plug::prelude::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use std::cell::Cell;
use std::time::Duration;
use std::time::Instant;

pub struct PeakMeter<L>
where
    L: Lens<Target = f32>,
{
    level_dbfs: L,
    hold_time: Option<Duration>,
}

pub struct PeakMeterOutline;

impl<L> PeakMeter<L>
where
    L: Lens<Target = f32>,
{
    pub fn new(cx: &mut Context, level_dbfs: L, hold_time: Option<Duration>) -> Handle<Self> {
        Self {
            level_dbfs,
            hold_time,
        }
        .build(cx, |_cx| ())
    }
}

impl<L> View for PeakMeter<L>
where
    L: Lens<Target = f32>,
{
    fn element(&self) -> Option<&'static str> {
        Some("peak-meter")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let held_peak_value_db = Cell::new(f32::MIN);
        let last_held_peak_value: Cell<Option<Instant>> = Cell::new(None);
        let level_dbfs = self.level_dbfs.get(cx);
        let peak_dbfs = match self.hold_time {
            Some(hold_time) => {
                let mut peak_level = held_peak_value_db.get();
                let peak_time = last_held_peak_value.get();

                let now = Instant::now();
                if level_dbfs >= peak_level
                    || peak_time.is_none()
                    || now > peak_time.unwrap() + hold_time
                {
                    peak_level = level_dbfs;
                    held_peak_value_db.set(peak_level);
                    last_held_peak_value.set(Some(now));
                }

                peak_level
            }
            None => util::MINUS_INFINITY_DB,
        };

        if level_dbfs > -50.0 {
            nih_dbg!("{:?}", level_dbfs);
        }

        let bounds = cx.bounds();
        let level_height = bounds.h * level_dbfs / 100.0 + bounds.h;
        let peak_height = bounds.h * peak_dbfs / 100.0 + bounds.h;
        // Level bar
        {
            let mut path = vg::Path::new();
            path.move_to(bounds.x, bounds.y + bounds.h);
            path.line_to(bounds.x + bounds.w, bounds.y + bounds.h);
            path.line_to(bounds.x + bounds.w, bounds.y + bounds.h - level_height);
            path.line_to(bounds.x, bounds.y + bounds.h - level_height);
            path.line_to(bounds.x, bounds.y + bounds.h);
            path.close();
            canvas.fill_path(&mut path, &vg::Paint::color(vg::Color::rgb(231, 124, 124)));
        }
        // Level peak
        {
            let mut path = vg::Path::new();
            path.move_to(bounds.x, bounds.y + bounds.h - peak_height);
            path.line_to(bounds.x + bounds.w, bounds.y + bounds.h - peak_height);
            canvas.stroke_path(&mut path, &vg::Paint::color(vg::Color::rgb(231, 124, 124)));
        }
    }
}

impl PeakMeterOutline {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_cx| ())
    }
}

impl View for PeakMeterOutline {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let mut paint = vg::Paint::color(vg::Color::rgb(255, 255, 255));
        paint.set_line_width(2.0);
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        path.move_to(bounds.x, bounds.y);
        path.line_to(bounds.x + bounds.w, bounds.y);
        path.line_to(bounds.x + bounds.w, bounds.y + bounds.h);
        path.line_to(bounds.x, bounds.y + bounds.h);
        path.line_to(bounds.x, bounds.y);
        path.close();
        canvas.stroke_path(&mut path, &paint);
    }
}
