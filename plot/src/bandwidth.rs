mod prelude;
use prelude::*;
use epy::prelude::*;

fn threads(device: Device) {
    let path = format!("threads_{}", device);
    let df = read_csv(&path);
    let unique = unique_threads(&df);

    let vqhull = df.clone().filter(|_, r| r.bench == Bench::VQhull);
    let pbbs = df.filter(|_, r| r.bench == Bench::PBBS);

    let mut ax = LinePlot::new(
            |r: &Record| r.threads as f64,
            "Threads",
            "Bandwidth utilization (\\si{{\\percent}})",
        )
        .grouped_series(&vqhull,
            |r| r.dataset as usize as f64,
            move |r| r.bandwidth_utilization(device),
            AggregationMode::meanstd(),
        )
        .grouped_series(&pbbs,
            |r| r.dataset as usize as f64,
            move |r| r.bandwidth_utilization(device),
            AggregationMode::meanstd(),
        )
        .build_axis();

    if device.physical_cores() != device.logical_cores() {
        ax = ax.line(Cs::Axis(device.physical_cores() as f64, 0.0), Cs::Axis(device.physical_cores() as f64, 200.0), None);
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
    let path = format!("../paper/fig_{}_threads_bandwidth.tex", device);
    tikz.write(&path).unwrap();
}

fn points(device: Device) {
    let path = format!("points_{}", device);
    let df = read_csv(&path);
    let unique = unique_sizes(&df);

    let vqhull = df.clone().filter(|_, r| r.bench == Bench::VQhull);
    let pbbs = df.filter(|_, r| r.bench == Bench::PBBS);

    let mut ax = LinePlot::new(
            |r: &Record| r.size as f64,
            "Input size (points)",
            "Bandwidth utilization (\\si{{\\percent}})",
        )
        .grouped_series(&vqhull,
            |r| r.dataset as usize as f64,
            move |r| r.bandwidth_utilization(device),
            AggregationMode::meanstd(),
        )
        .grouped_series(&pbbs,
            |r| r.dataset as usize as f64,
            move |r| r.bandwidth_utilization(device),
            AggregationMode::meanstd(),
        )
        .build_axis();

    ax.style.title = Some(device.to_string());
    ax.style.ymin = Some(0.0);
    ax.style.xmode = Scaling::Log;
    ax.style.xticks = unique.into_iter().map(|t| t as f64).collect::<Vec<_>>().into();
    copy_linestyle(&mut ax);
    if device == Device::CN132 {
        ax.style.legend_pos = Some(Anchor::NorthWest);
        adjust_legend(&mut ax);
    } else {
        remove_legend(&mut ax);
    }

    let tikz = TikzPicture::from_axis(ax);
    let path = format!("../paper/fig_{}_points_bandwidth.tex", device);
    tikz.write(&path).unwrap();
}

fn main() {
    threads(Device::CN125);
    threads(Device::CN132);
    threads(Device::Laptop);
    points(Device::CN125);
    points(Device::CN132);
    points(Device::Laptop);
}
