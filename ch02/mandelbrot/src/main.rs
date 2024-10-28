use std::{fs::File, str::FromStr};

use clap::Arg;
use image::{codecs::png::PngEncoder, ImageEncoder};
use num::Complex;

/// `c`가 만델브로 집합에 속하는지 아닌지를 판단하며,
/// 결론 내리는 데 필요한 반복 횟수는 최대 `limit`회로 제한한다.
///
/// `c`가 만델브로 집합에 속하지 않으면 `Some(i)`를 반환하는데,
/// 여기서 `i`는 `c`가 원점을 중심으로 반경이 2인 원을 벗어난데 걸린 반복 횟수다.
/// `c`가 만델브로 집합에 속하는 것 같으면
/// (좀 더 정확히 말해서 반복 횟수가 `limit`이 될 때까지도 `c`가 만델브로 집하에 속하지 않는다는 것을 입증하지 못하면)
/// `None`을 반환한다.
fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex::new(0.0, 0.0);
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

/// `s`를 `"400x600"`이나 `"1.0.0.5"`와 같은 좌표 쌍으로 파싱한다.
///
/// `s`는 정확히 <left><sep><right> 형식으로 되어 있어야 하는데,
/// 여기서 <sep>은 `separator` 인수에 넘기는 문자이고
/// <left>와 <right>는 둘 다 `T::from_str`로 파싱할 수 있는 문자열이다.
/// `separator`는 반드시 아스키 문자여야 한다.
///
/// `s`가 올바른 형식으로 되어 있으면 `Some<(x, y)>`를 반환한다.
/// 제대로 바싱되지 않으면 `None`을 반환한다.
fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) => match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
            (Ok(l), Ok(r)) => Some((l, r)),
            _ => None,
        },
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("s", ','), None);
    assert_eq!(parse_pair::<i32>("10,", ','), None);
    assert_eq!(parse_pair::<i32>(",10", ','), None);
    assert_eq!(parse_pair("10,20", ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy", ','), None);
    assert_eq!(parse_pair::<f64>("0.5x", 'x'), None);
    assert_eq!(parse_pair("0.5x1.5", 'x'), Some((0.5, 1.5)));
}

/// 쉼표로 구분된 부동소수점 수 쌍을 복소수로 파싱한다.
fn parse_complex(s: &str) -> Option<Complex<f64>> {
    parse_pair(s, ',').map(|(re, im)| Complex::new(re, im))
}

#[test]
fn test_parse_complex() {
    assert_eq!(
        parse_complex("1.25,-0.0625"),
        Some(Complex::new(1.25, -0.0625))
    );
    assert_eq!(parse_complex(",-0.0625"), None);
}

/// 결과 이미지의 픽셀 좌표가 주어지면 여기에 대응하는 복소평면 위의 점을 반환한다.
///
/// `bound`는 픽셀 단위로 된 이미지의 폭과 높이를 갖는 쌍이다.
/// `pixel`은 이미지의 특정 픽셀을 나타내는 (열, 행)으로 된 쌍이다.
/// `upper_left`와 `upper_right` 매개변수는 이미지가 커버하는 영역을 지정하는 복소평면 위의 두 점이다.
fn pixel_to_point(
    bounds: (usize, usize),
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) -> Complex<f64> {
    let (width, height) = (
        lower_right.re - upper_left.re,
        upper_left.im - lower_right.im,
    );
    Complex::new(
        upper_left.re + pixel.0 as f64 * width / bounds.0 as f64,
        // 여기서 뺄셈을 하는 이유는 pixel.1의 경우 아래로 갈수록 증가하지만 허수 부분은 위로 갈수록 증가하기 때문이다.
        upper_left.im - pixel.1 as f64 * height / bounds.1 as f64,
    )
}

/// 직사각형 모양의 망델브로 집합을 픽셀 버퍼에 렌더링한다.
///
/// `bound`는 인수는 한 바이트 안에 회색조로 된 픽셀 하나가 들어가는 `pixels` 버퍼의 폭과 높이를 갖는다.
/// `upper_left`와 `lower_right`인수는 픽셀 버퍼의 왼쪽 위 모서리와 오른쪽 아래 모리에 해당하는 복소평면 위의 두 점을 지정한다.
fn render(
    pixels: &mut [u8],
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
) {
    assert!(pixels.len() == bounds.0 * bounds.1);

    for row in 0..bounds.1 {
        for column in 0..bounds.0 {
            let point = pixel_to_point(bounds, (column, row), upper_left, lower_right);
            pixels[row * bounds.0 + column] = match escape_time(point, 255) {
                None => 0,
                Some(count) => 255 - count as u8,
            };
        }
    }
}

/// `bound` 크기의 `pixels` 버퍼를 `filename` 파일에 기록한다.
fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), anyhow::Error> {
    let output = File::create(filename)?;
    let encoder = PngEncoder::new(output);
    encoder.write_image(
        pixels,
        bounds.0 as u32,
        bounds.1 as u32,
        image::ExtendedColorType::L8,
    )?;

    Ok(())
}

// #[derive(Debug)]
struct Args {
    filename: String,
    pixel: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>,
    thread: usize,
}

/// 명령줄의 인수를 파싱한다.
fn get_args() -> Option<Args> {
    let parse_complex =
        |s: &str| parse_complex(s).ok_or(anyhow::anyhow!("Parse Complex Error: {s}"));

    let matches = clap::Command::new("mandelbrot")
        .version("0.1.0")
        .author("TestAquatic")
        .about("프로그래밍 러스트의 예제 코드")
        .arg(
            Arg::new("filename")
                .value_name("FILENAME")
                .num_args(1)
                .required(true)
                .help("eg: mandel.png"),
        )
        .arg(
            Arg::new("pixel")
                .value_name("PIXEL")
                .num_args(1)
                .required(true)
                .value_parser(|s: &str| {
                    parse_pair::<usize>(s, 'x').ok_or(anyhow::anyhow!("Pixel Parse Error: {s}"))
                })
                .help("eg: 1000x750"),
        )
        .arg(
            Arg::new("upper_left")
                .value_name("UPPER_LEFT")
                .num_args(1)
                .required(true)
                .value_parser(parse_complex)
                .help("eg: -1.20,0.35"),
        )
        .arg(
            Arg::new("lower_right")
                .value_name("LOWER_RIGHT")
                .num_args(1)
                .required(true)
                .value_parser(parse_complex)
                .help("eg: -1,0.20"),
        )
        .arg(
            Arg::new("thread")
                .short('t')
                .long("thread")
                .default_value("8")
                .value_parser(usize::from_str)
                .help("Number of threads to run"),
        )
        .get_matches();

    let args = Args {
        filename: matches.get_one("filename").cloned()?,
        pixel: matches.get_one("pixel").cloned()?,
        upper_left: matches.get_one("upper_left").cloned()?,
        lower_right: matches.get_one("lower_right").cloned()?,
        thread: matches.get_one("thread").cloned()?,
    };

    Some(args)
}

fn main() {
    let args = match get_args() {
        None => {
            eprintln!("Failed to get args.");
            std::process::exit(1);
        }
        Some(args) => args,
    };

    // dbg!(&args);

    let mut pixels = vec![0; args.pixel.0 * args.pixel.1];

    let threads = args.thread;
    let rows_per_band = args.pixel.1 / threads + 1;

    {
        let bands = pixels
            .chunks_mut(rows_per_band * args.pixel.0)
            .collect::<Vec<_>>();
        crossbeam::scope(|spawner| {
            for (i, band) in bands.into_iter().enumerate() {
                let top = rows_per_band * i;
                let height = band.len() / args.pixel.0;
                let band_bounds = (args.pixel.0, height);
                let band_upper_left =
                    pixel_to_point(args.pixel, (0, top), args.upper_left, args.lower_right);
                let band_lower_right = pixel_to_point(
                    args.pixel,
                    (args.pixel.0, top + height),
                    args.upper_left,
                    args.lower_right,
                );

                spawner.spawn(move |_| {
                    render(band, band_bounds, band_upper_left, band_lower_right);
                });
            }
        })
        .unwrap();
    }

    write_image(&args.filename, &pixels, args.pixel).expect("error writing PNG file");
}
