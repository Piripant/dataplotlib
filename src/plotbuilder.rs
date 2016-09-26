//! **plotbuilder** provides the `struct`s that organize the plot data, plus some helper functions
//!
//! It is recommended to use `PlotBuilder2D::simple_xy` rather than manually instantiating the `PlotBuilder2D` struct,
//! but feel free to approach this in the most ergonomic fashion for you.

use std::marker::Sync;
use std::sync::Arc;

pub type PlotFn = &'static (Fn(f64) -> f64 + Sync);
pub type AnimFn = &'static (Fn(f64, f64) -> f64 + Sync);

#[derive(Clone)]
pub enum PlotStyle {
    OnlyLines,
    OnlyDots,
    LinesAndDots,
}

// Storing the data of one plot
pub trait PlotData : Send + Sync {
    fn get_style (&self) -> &PlotStyle;
    fn get_plot (&self) -> (&[f32; 4], &Vec<(f64, f64)>);
}

pub struct ColoredPlot {
    pub xy: Vec<(f64, f64)>,
    pub color: [f32; 4],
    pub plot_style: PlotStyle,
}

impl PlotData for ColoredPlot {
    fn get_style (&self) -> &PlotStyle {
        &self.plot_style
    }

    fn get_plot (&self) -> (&[f32; 4], &Vec<(f64, f64)>) {
        (&self.color, &self.xy)
    }
}

// Make it thread safe
unsafe impl Send for ColoredPlot { }
unsafe impl Sync for ColoredPlot { }

// Every graph trait will be listed in this enumerator
// pub enum GraphTypes {
//    PlotGraph(PlotData),
//    BarGraph, // Yet to be implemented
//    GraphGraph, // Yet to be implemented
//}

/// `PlotBuilder2D` contains all the higher level info of the graph (like title and labels)
#[derive(Clone)]
pub struct PlotBuilder2D {
    /// **pvs** contains the actual graph values
    pub pvs: Vec<Arc<PlotData>>,

    /// **min_x** optionally defines the lower x bound. If `None`, it will be auto determined.
    pub min_x: Option<f64>,

    /// **max_x** optionally defines the upper x bound. If `None`, it will be auto determined.
    pub max_x: Option<f64>,

    /// **min_y** optionally defines the lower y bound. If `None`, it will be auto determined.
    pub min_y: Option<f64>,

    /// **max_y** optionally defines the upper y bound. If `None`, it will be auto determined.
    pub max_y: Option<f64>,

    /// A string to label the x-axis. (not implemented)
    pub x_label: Option<String>,

    /// A string to label the y-axis. (not implemented)
    pub y_label: Option<String>,

    /// A string to label the chart. (not implemented)
    pub title: Option<String>,

    /// Whether or not to draw the y-axis. (not implemented)
    pub y_axis: bool,

    /// Whether or not to draw the gridlines on the y-axis. (not implemented)
    pub y_gridlines: bool,

    /// Whether or not to draw the x-axis. (not implemented)
    pub x_axis: bool,

    /// Whether or not to draw the gridlines on the x-axis. (not implemented)
    pub x_gridlines: bool,

    /// The font file to use for any text. (not implemented)
    pub font_path: String,
}

const DEFAULT_FONT: &'static str = "/usr/share/fonts/truetype/freefont/FreeSans.ttf";

// String the data of a graph
impl PlotBuilder2D {
    /// `simply_xy` reduces boilerplate by generating some basic defaults for the `PlotBuilder2D` struct.
    /// Once the struct is returned, it's easy enough to make adjustments.
    pub fn simple_xy(xy: Vec<(f64, f64)>, color: [f32; 4]) -> PlotBuilder2D {
        let plot = ColoredPlot {xy: xy, color: color, plot_style: PlotStyle::OnlyLines};

        PlotBuilder2D {
            pvs: vec![Arc::new(plot)],
            min_x: None,
            max_x: None,
            min_y: None,
            max_y: None,
            x_label: None,
            y_label: None,
            title: None,
            y_axis: true,
            y_gridlines: true,
            x_axis: true,
            x_gridlines: true,
            font_path: DEFAULT_FONT.to_string(),
        }
    }

    // TODO: implement multiple plots
}