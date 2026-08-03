mod prelude;
use prelude::*;
use epy::prelude::*;

fn baseline(df: &DataFrame<Record>, dataset: Dataset) -> f64 {
    let runtimes = df.clone()
        .filter(|_, r| r.threads == 1)
        .filter(|_, r| r.dataset == dataset)
        .rows()
        .into_iter()
        .map(|r| r.runtime())
        .collect::<Vec<f64>>();
    let n = runtimes.len();
    runtimes.into_iter().sum::<f64>() / (n as f64)
}

impl Record {
    fn speedup(&self, baseline_circle: f64, baseline_disk: f64, baseline_kuzmin: f64) -> f64 {
        match self.dataset {
            Dataset::Circle => baseline_circle / self.runtime(),
            Dataset::Disk => baseline_disk / self.runtime(),
            Dataset::Kuzmin => baseline_kuzmin / self.runtime(),
        }
    }
}

fn threads(device: Device) {
    let path = format!("threads_{}", device);
    let df = read_csv(&path);
    let unique = unique_threads(&df);

    let vqhull = df.clone().filter(|_, r| r.bench == Bench::VQhull);
    let pbbs = df.filter(|_, r| r.bench == Bench::PBBS);

    // Sequential baselines
    let vqhull_circle = baseline(&vqhull, Dataset::Circle);
    let vqhull_disk = baseline(&vqhull, Dataset::Disk);
    let vqhull_kuzmin = baseline(&vqhull, Dataset::Kuzmin);
    let pbbs_circle = baseline(&pbbs, Dataset::Circle);
    let pbbs_disk = baseline(&pbbs, Dataset::Disk);
    let pbbs_kuzmin = baseline(&pbbs, Dataset::Kuzmin);

    let mut ax = LinePlot::new(
            |r: &Record| r.threads as f64,
            "Threads",
            "Speedup vs sequential PBBS",
        )
        .grouped_series(&vqhull,
            |r| r.dataset as usize as f64,
            move |r| r.speedup(vqhull_circle, vqhull_disk, vqhull_kuzmin),
            AggregationMode::meanstd(),
        )
        .grouped_series(&pbbs,
            |r| r.dataset as usize as f64,
            move |r| r.speedup(pbbs_circle, pbbs_disk, pbbs_kuzmin),
            AggregationMode::meanstd(),
        )
        .build_axis();

    if device.physical_cores() != device.logical_cores() {
        ax = ax.line(Cs::Axis(device.physical_cores() as f64, 0.0), Cs::Axis(device.physical_cores() as f64, 30.0), None);
    }

    ax.style.title = Some(device.to_string());
    ax.style.ymin = Some(0.0);
    ax.style.xmode = Scaling::Log;
    ax.style.xtick_labels = Some(unique.iter().map(|t| t.to_string()).collect::<Vec<_>>().into());
    ax.style.xticks = unique.into_iter().map(|t| t as f64).collect::<Vec<_>>().into();
    copy_linestyle(&mut ax);
    if device == Device::CN132 {
        ax.style.legend_pos = Some(Anchor::NorthWest);
        adjust_legend(&mut ax);
    } else {
        remove_legend(&mut ax);
    }

    let tikz = TikzPicture::from_axis(ax);
    let path = format!("../paper/fig_{}_threads_speedup.tex", device);
    tikz.write(&path).unwrap();
}

fn main() {
    threads(Device::CN125);
    threads(Device::CN132);
    threads(Device::Laptop);
}
