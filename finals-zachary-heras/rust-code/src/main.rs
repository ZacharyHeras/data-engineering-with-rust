use image::RgbImage;
use plotters::prelude::*;
use uuid::Uuid;
use std::time::Instant;
use std::error::Error;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let start_time = Instant::now();

    let duration_seconds = 1.0;
    let frequency_hz = 5.0;
    let sampling_rate_hz = 5000.0;

    for _ in 0..1000 {
        let img = plot_sine_wave(duration_seconds, frequency_hz, sampling_rate_hz);
        let img_reversed = plot_sine_wave_reversed(duration_seconds, frequency_hz, sampling_rate_hz);

        let uuid = Uuid::new_v4();
        let filename = format!("rust_figures/sine_wave_{}.png", uuid);

        let reversed_filename = format!("rust_figures/sine_wave_reversed_{}.png", uuid);

        match img {
            Ok(value) => value.save(&filename)?,
            _ => panic!("Image generation failed!"),
        }

        match img_reversed {
            Ok(value) => value.save(&reversed_filename)?,
            _ => panic!("Image generation failed!"),
        }
    }

    let elapsed_time = start_time.elapsed();

    println!("Elapsed time: {:?}", elapsed_time);

    Ok(())
}

fn plot_sine_wave(
    duration_seconds: f64,
    frequency_hz: f64,
    sampling_rate_hz: f64,
) -> Result<RgbImage, Box<dyn std::error::Error>> {
    let num_points = (duration_seconds * sampling_rate_hz) as usize;
    let dt = 1.0 / sampling_rate_hz;

    let x_min = 0.0;
    let x_max = duration_seconds;
    let y_min = -1.2;
    let y_max = 1.2;
    let width: u32 = 800;
    let height: u32 = 600;

    let mut data = vec![];
    for i in 0..=num_points {
        let t = x_min + (i as f64) * dt;
        let x = 2.0 * std::f64::consts::PI * frequency_hz * t;
        let y = x.sin();
        data.push((t, y));
    }

    let mut img = RgbImage::new(width, height);

    {
        let root = BitMapBackend::with_buffer(&mut img, (width, height)).into_drawing_area();

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root).build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.draw_series(LineSeries::new(data.iter().map(|&(t, y)| (t, y)), &BLACK))?;
    }

    Ok(img)
}

fn plot_sine_wave_reversed(
    duration_seconds: f64,
    frequency_hz: f64,
    sampling_rate_hz: f64,
) -> Result<RgbImage, Box<dyn std::error::Error>> {
    let num_points = (duration_seconds * sampling_rate_hz) as usize;
    let dt = 1.0 / sampling_rate_hz;

    let x_min = 0.0;
    let x_max = duration_seconds;
    let y_min = -1.2;
    let y_max = 1.2;
    let width: u32 = 800;
    let height: u32 = 600;

    let mut data = vec![];
    for i in 0..=num_points {
        let t = x_min + (i as f64) * dt;
        let x = 2.0 * std::f64::consts::PI * frequency_hz * t;
        let y = x.sin();
        data.push((t, y));
    }

    data.reverse();

    let mut img = RgbImage::new(width, height);

    {
        let root = BitMapBackend::with_buffer(&mut img, (width, height)).into_drawing_area();

        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root).build_cartesian_2d(x_min..x_max, y_min..y_max)?;

        chart.draw_series(LineSeries::new(data.iter().map(|&(t, y)| (t, y)), &BLACK))?;
    }

    Ok(img)
}

#[test]
fn test_plot_sine_wave() -> Result<(), Box<dyn Error>> {
    let duration_seconds = 1.0;
    let frequency_hz = 1.0;
    let sampling_rate_hz = 100.0;

    let result = plot_sine_wave_reversed(duration_seconds, frequency_hz, sampling_rate_hz)?;

    assert_eq!(result.width(), 800);
    assert_eq!(result.height(), 600);
    
    Ok(())
}

#[test]
fn test_plot_sine_wave_reversed() -> Result<(), Box<dyn Error>> {
    let duration_seconds = 1.0;
    let frequency_hz = 1.0;
    let sampling_rate_hz = 100.0;

    let result = plot_sine_wave_reversed(duration_seconds, frequency_hz, sampling_rate_hz)?;

    assert_eq!(result.width(), 800);
    assert_eq!(result.height(), 600);
    
    Ok(())
}
