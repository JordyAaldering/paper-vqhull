mod prelude;
use epy::prelude::{AggregationMode, DataFrame};
use prelude::*;

fn runtime(df: &DataFrame<Record>, bench: Bench, threads: usize) -> String {
    df.clone()
        .filter(|_, r| r.bench == bench)
        .filter(|_, r| r.threads == threads)
        .group_by(|r| r.dataset)
        .summarize_by_group(&|r| r.runtime(), AggregationMode::meanstd())
        .centers
        .into_iter()
        .map(|x| format!("{:.2}", x))
        .collect::<Vec<_>>()
        .join(" & ")
}

fn runtime_table() {
    let cn125 = read_csv("threads_cn125");
    let cn132 = read_csv("threads_cn132");
    let laptop = read_csv("threads_laptop");

    println!("\\begin{{tabular}}{{lrrrrrrrrr}}");
    println!("  & \\multicolumn{{3}}{{c}}{{cn125}} & \\multicolumn{{3}}{{c}}{{cn132}} & \\multicolumn{{3}}{{c}}{{laptop}} \\\\");
    println!("  & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin \\\\\\midrule");
    println!("  PBBS seq & {} & {} & {} \\\\",
        runtime(&cn125, Bench::PBBS, 1),
        runtime(&cn132, Bench::PBBS, 1),
        runtime(&laptop, Bench::PBBS, 1));
    println!("  VQhull seq & {} & {} & {} \\\\",
        runtime(&cn125, Bench::VQhull, 1),
        runtime(&cn132, Bench::VQhull, 1),
        runtime(&laptop, Bench::VQhull, 1));
    println!("  PBBS par & {} & {} & {} \\\\",
        runtime(&cn125, Bench::PBBS, Device::CN125.physical_cores()),
        runtime(&cn132, Bench::PBBS, Device::CN132.physical_cores()),
        runtime(&laptop, Bench::PBBS, Device::Laptop.physical_cores()));
    println!("  VQhull par & {} & {} & {} \\\\",
        runtime(&cn125, Bench::VQhull, Device::CN125.physical_cores()),
        runtime(&cn132, Bench::VQhull, Device::CN132.physical_cores()),
        runtime(&laptop, Bench::VQhull, Device::Laptop.physical_cores()));
    println!("\\end{{tabular}}");
}

fn bandwidth(df: &DataFrame<Record>, bench: Bench, threads: usize) -> String {
    df.clone()
        .filter(|_, r| r.bench == bench)
        .filter(|_, r| r.threads == threads)
        .group_by(|r| r.dataset)
        .summarize_by_group(&|r| r.bandwidth(), AggregationMode::meanstd())
        .centers
        .into_iter()
        .map(|x| format!("{:.1}", x))
        .collect::<Vec<_>>()
        .join(" & ")
}

fn bandwidth_table() {
    let cn125 = read_csv("threads_cn125");
    let cn132 = read_csv("threads_cn132");
    let laptop = read_csv("threads_laptop");

    println!("\\begin{{tabular}}{{lrrrrrrrrr}}");
    println!("  & \\multicolumn{{3}}{{c}}{{cn125}} & \\multicolumn{{3}}{{c}}{{cn132}} & \\multicolumn{{3}}{{c}}{{laptop}} \\\\");
    println!("  & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin \\\\\\midrule");
    println!("  PBBS seq & {} & {} & {} \\\\",
        bandwidth(&cn125, Bench::PBBS, 1),
        bandwidth(&cn132, Bench::PBBS, 1),
        bandwidth(&laptop, Bench::PBBS, 1));
    println!("  VQhull seq & {} & {} & {} \\\\",
        bandwidth(&cn125, Bench::VQhull, 1),
        bandwidth(&cn132, Bench::VQhull, 1),
        bandwidth(&laptop, Bench::VQhull, 1));
    println!("  PBBS par & {} & {} & {} \\\\",
        bandwidth(&cn125, Bench::PBBS, Device::CN125.physical_cores()),
        bandwidth(&cn132, Bench::PBBS, Device::CN132.physical_cores()),
        bandwidth(&laptop, Bench::PBBS, Device::Laptop.physical_cores()));
    println!("  VQhull par & {} & {} & {} \\\\",
        bandwidth(&cn125, Bench::VQhull, Device::CN125.physical_cores()),
        bandwidth(&cn132, Bench::VQhull, Device::CN132.physical_cores()),
        bandwidth(&laptop, Bench::VQhull, Device::Laptop.physical_cores()));
    println!("\\end{{tabular}}");
}

fn bandwidth_utilization(df: &DataFrame<Record>, device: Device, bench: Bench, threads: usize) -> String {
    df.clone()
        .filter(|_, r| r.bench == bench)
        .filter(|_, r| r.threads == threads)
        .group_by(|r| r.dataset)
        .summarize_by_group(&|r| r.bandwidth_utilization(device), AggregationMode::meanstd())
        .centers
        .into_iter()
        .map(|x| format!("{:.1}", x))
        .collect::<Vec<_>>()
        .join(" & ")
}

fn bandwidth_utilization_table() {
    let cn125 = read_csv("threads_cn125");
    let cn132 = read_csv("threads_cn132");
    let laptop = read_csv("threads_laptop");

    println!("\\begin{{tabular}}{{lrrrrrrrrr}}");
    println!("  & \\multicolumn{{3}}{{c}}{{cn125}} & \\multicolumn{{3}}{{c}}{{cn132}} & \\multicolumn{{3}}{{c}}{{laptop}} \\\\");
    println!("  & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin \\\\\\midrule");
    println!("  PBBS seq & {} & {} & {} \\\\",
        bandwidth_utilization(&cn125, Device::CN125, Bench::PBBS, 1),
        bandwidth_utilization(&cn132, Device::CN132, Bench::PBBS, 1),
        bandwidth_utilization(&laptop, Device::Laptop, Bench::PBBS, 1));
    println!("  VQhull seq & {} & {} & {} \\\\",
        bandwidth_utilization(&cn125, Device::CN125, Bench::VQhull, 1),
        bandwidth_utilization(&cn132, Device::CN132, Bench::VQhull, 1),
        bandwidth_utilization(&laptop, Device::Laptop, Bench::VQhull, 1));
    println!("  PBBS par & {} & {} & {} \\\\",
        bandwidth_utilization(&cn125, Device::CN125, Bench::PBBS, Device::CN125.physical_cores()),
        bandwidth_utilization(&cn132, Device::CN132, Bench::PBBS, Device::CN132.physical_cores()),
        bandwidth_utilization(&laptop, Device::Laptop, Bench::PBBS, Device::Laptop.physical_cores()));
    println!("  VQhull par & {} & {} & {} \\\\",
        bandwidth_utilization(&cn125, Device::CN125, Bench::VQhull, Device::CN125.physical_cores()),
        bandwidth_utilization(&cn132, Device::CN132, Bench::VQhull, Device::CN132.physical_cores()),
        bandwidth_utilization(&laptop, Device::Laptop, Bench::VQhull, Device::Laptop.physical_cores()));
    println!("\\end{{tabular}}");
}

fn energy(df: &DataFrame<Record>, bench: Bench, threads: usize) -> String {
    df.clone()
        .filter(|_, r| r.bench == bench)
        .filter(|_, r| r.threads == threads)
        .group_by(|r| r.dataset)
        .summarize_by_group(&|r| r.rapl(), AggregationMode::Quartiles)
        .centers
        .into_iter()
        .map(|x| format!("{:.1}", x))
        .collect::<Vec<_>>()
        .join(" & ")
}

fn energy_table() {
    let cn125 = read_csv("threads_cn125");
    let cn132 = read_csv("threads_cn132");
    let laptop = read_csv("threads_laptop");

    println!("\\begin{{tabular}}{{lrrrrrrrrr}}");
    println!("  & \\multicolumn{{3}}{{c}}{{cn125}} & \\multicolumn{{3}}{{c}}{{cn132}} & \\multicolumn{{3}}{{c}}{{laptop}} \\\\");
    println!("  & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin & Circle & Disk & Kuzmin \\\\\\midrule");
    println!("  PBBS seq & {} & {} & {} \\\\",
        energy(&cn125, Bench::PBBS, 1),
        energy(&cn132, Bench::PBBS, 1),
        energy(&laptop, Bench::PBBS, 1));
    println!("  VQhull seq & {} & {} & {} \\\\",
        energy(&cn125, Bench::VQhull, 1),
        energy(&cn132, Bench::VQhull, 1),
        energy(&laptop, Bench::VQhull, 1));
    println!("  PBBS par & {} & {} & {} \\\\",
        energy(&cn125, Bench::PBBS, Device::CN125.physical_cores()),
        energy(&cn132, Bench::PBBS, Device::CN132.physical_cores()),
        energy(&laptop, Bench::PBBS, Device::Laptop.physical_cores()));
    println!("  VQhull par & {} & {} & {} \\\\",
        energy(&cn125, Bench::VQhull, Device::CN125.physical_cores()),
        energy(&cn132, Bench::VQhull, Device::CN132.physical_cores()),
        energy(&laptop, Bench::VQhull, Device::Laptop.physical_cores()));
    println!("\\end{{tabular}}");
}

fn uncore(df: &DataFrame<Record>, bench: Bench, threads: usize) -> String {
    let uncore = df.clone()
        .filter(|_, r| r.bench == bench)
        .filter(|_, r| r.threads == threads)
        .group_by(|r| r.dataset)
        .summarize_by_group(&|r| r.uncore() * 1000.0, AggregationMode::Quartiles);
    uncore.lowers.iter()
        .zip(uncore.uppers.iter())
        .zip(uncore.centers.iter())
        .map(|((q1, q3), median)| format!("{:.2} & {:.2}", median, q3 - q1))
        .collect::<Vec<_>>()
        .join(" & ")
}

fn uncore_table() {
    let laptop = read_csv("threads_laptop");

    println!("\\begin{{tabular}}{{lrrrrrr}}");
    println!("  & \\multicolumn{{6}}{{c}}{{laptop}} \\\\");
    println!("  & Circle & IQR & Disk & IQR & Kuzmin & IQR \\\\\\midrule");
    println!("  PBBS seq & {} \\\\",
        uncore(&laptop, Bench::PBBS, 1));
    println!("  VQhull seq & {} \\\\",
        uncore(&laptop, Bench::VQhull, 1));
    println!("  PBBS par & {} \\\\",
        uncore(&laptop, Bench::PBBS, Device::Laptop.physical_cores()));
    println!("  VQhull par & {} \\\\",
        uncore(&laptop, Bench::VQhull, Device::Laptop.physical_cores()));
    println!("\\end{{tabular}}");
}

fn main() {
    runtime_table();
    println!();
    bandwidth_table();
    println!();
    bandwidth_utilization_table();
    println!();
    energy_table();
    println!();
    uncore_table();
}
