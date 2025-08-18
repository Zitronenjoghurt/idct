use crate::components::Component;
use egui::Ui;
use egui_plot::{Bar, BarChart, Plot};
use idct_game::utils::random::distribution::{RandomDistribution, RandomizedDistribution};
use idct_game::utils::random::seed::RandomSeed;

pub struct RandomDistributionEdit<'a> {
    distribution: &'a mut RandomDistribution,
}

impl<'a> RandomDistributionEdit<'a> {
    pub fn new(distribution: &'a mut RandomDistribution) -> Self {
        Self { distribution }
    }

    fn samples(&self) -> Vec<f32> {
        let seed = RandomSeed::from(42);
        let mut rng = seed.create_rng();
        (0..10000)
            .filter_map(|_| self.distribution.sample(&mut rng).ok())
            .collect()
    }

    fn histogram(&self) -> Histogram {
        let samples = self.samples();

        let min = samples.iter().fold(f32::INFINITY, |a, &b| a.min(b));
        let max = samples.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
        let bin_count = 30;
        let bin_width = (max - min) / bin_count as f32;

        let mut bins = vec![0; bin_count];
        for sample in &samples {
            let bin = ((sample - min) / bin_width).floor() as usize;
            let bin = bin.min(bin_count - 1);
            bins[bin] += 1;
        }

        let points = bins
            .iter()
            .enumerate()
            .map(|(i, &count)| {
                let x = min + (i as f32 + 0.5) * bin_width;
                [x as f64, count as f64]
            })
            .collect();

        Histogram {
            min,
            max,
            bin_width,
            samples,
            points,
        }
    }

    fn bar(&self, x: f64, y: f64, histogram: &Histogram) -> Bar {
        Bar::new(x, y)
            .width(histogram.bin_width as f64 * 0.9)
            .name(format!(
                "[{:.2}, {:.2}]: {:.1}%",
                x - histogram.bin_width as f64 / 2.0,
                x + histogram.bin_width as f64 / 2.0,
                (y / histogram.sample_count() as f64) * 100.0
            ))
    }

    fn bars(&self, histogram: &Histogram) -> Vec<Bar> {
        histogram
            .points
            .iter()
            .map(|[x, y]| self.bar(*x, *y, histogram))
            .collect()
    }

    fn bar_chart(&self, histogram: &Histogram) -> BarChart {
        BarChart::new("Test", self.bars(histogram))
    }

    fn show_plot(&self, histogram: &Histogram, ui: &mut Ui) {
        Plot::new("random_distribution_edit_plot")
            .height(250.0)
            .width(250.0)
            .show_axes([true, true])
            .x_axis_formatter(|grid_mark, _| format!("{:.1}", grid_mark.value))
            .y_axis_formatter(|grid_mark, _| {
                format!(
                    "{:.0}%",
                    (grid_mark.value / histogram.sample_count() as f64) * 100.0
                )
            })
            .show(ui, |plot_ui| {
                plot_ui.bar_chart(self.bar_chart(histogram));
            });
    }
}

impl Component for RandomDistributionEdit<'_> {
    fn show(self, ui: &mut Ui) {
        ui.vertical(|ui| {
            let histogram = self.histogram();
            self.show_plot(&histogram, ui);

            let current = match self.distribution {
                RandomDistribution::Normal(_) => 0,
                RandomDistribution::Uniform(_) => 1,
            };
            let mut selected = current;

            egui::ComboBox::from_label("Type")
                .selected_text(if current == 0 { "Normal" } else { "Uniform" })
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut selected, 0, "Normal");
                    ui.selectable_value(&mut selected, 1, "Uniform");
                });

            if selected != current {
                *self.distribution = if selected == 0 {
                    RandomDistribution::Normal(Default::default())
                } else {
                    RandomDistribution::Uniform(Default::default())
                };
            }

            match self.distribution {
                RandomDistribution::Normal(normal) => {
                    ui.add(egui::Slider::new(&mut normal.mean, -10.0..=10.0).text("Mean"));
                    ui.add(egui::Slider::new(&mut normal.std_dev, 0.01..=5.0).text("Std Dev"));
                }
                RandomDistribution::Uniform(uniform) => {
                    ui.add(egui::Slider::new(&mut uniform.min, -10.0..=10.0).text("Min"));
                    ui.add(egui::Slider::new(&mut uniform.max, -10.0..=10.0).text("Max"));
                }
            }

            ui.horizontal(|ui| {
                ui.label(format!(
                    "Range: [{:.2}, {:.2}]",
                    histogram.min, histogram.max
                ));
                ui.label(format!("Mean: {:.2}", histogram.mean()));
            });
        });
    }
}

struct Histogram {
    pub min: f32,
    pub max: f32,
    pub bin_width: f32,
    pub samples: Vec<f32>,
    pub points: Vec<[f64; 2]>,
}

impl Histogram {
    pub fn sample_count(&self) -> usize {
        self.samples.len()
    }

    pub fn mean(&self) -> f32 {
        self.samples.iter().sum::<f32>() / self.samples.len() as f32
    }
}
