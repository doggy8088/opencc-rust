use opencc_rust::converter;
use std::{hint::black_box, time::Instant};
fn main() {
    let c = converter("cn", "tw2").unwrap();
    let sample = "云电脑里的云盘空间不足，登录后通过网络访问服务器。这个订阅套餐支持点赞和反馈，点击保存。视频质量不好，远程打印也卡。他的账号发帖后，私信我一段录屏文本。保存期限七天，考试通过名单公布。\n";
    let mut input = sample.repeat(1_048_576 / sample.len() + 1);
    let end = (0..=1_048_576)
        .rev()
        .find(|&i| input.is_char_boundary(i))
        .unwrap();
    input.truncate(end);
    while input.len() < 1_048_576 {
        input.push(' ');
    }
    for _ in 0..3 {
        black_box(c.convert(black_box(&input)));
    }
    let mut times = Vec::new();
    for _ in 0..15 {
        let start = Instant::now();
        black_box(c.convert(black_box(&input)));
        times.push(start.elapsed().as_secs_f64() * 1000.0);
    }
    times.sort_by(f64::total_cmp);
    println!(
        "bytes={} median_ms={:.3} min_ms={:.3}",
        input.len(),
        times[7],
        times[0]
    );
}
