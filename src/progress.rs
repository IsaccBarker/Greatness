use indicatif::{ProgressBar, ProgressStyle};
use rand::seq::SliceRandom;

pub fn new_progress_bar(length: u64) -> ProgressBar {
    let spinners = vec![
        "|/-\\",
        "▁▃▄▅▆▇█▇▆▅▄▃",
        "▉▊▋▌▍▎▏▎▍▌▋▊▉",
        "▖▘▝▗",
        "┤┘┴└├┌┬┐",
        "◢◣◤◥",
        "◰◳◲◱",
        "◴◷◶◵",
        "◐◓◑◒",
        "◇◈◆",
        "◇◈◆",
        "🕛🕐🕑🕒🕓🕔🕕🕖🕗🕘🕙🕚",
        "🌍🌎🌏",
        "🌑🌒🌓🌔🌕🌖🌗🌘",
        "-=≡",
    ];

    let pb = ProgressBar::new(length);
    pb.set_style(
        ProgressStyle::default_bar()
            .tick_chars(spinners.choose(&mut rand::thread_rng()).unwrap())
            .template("{spinner} {bar:40.bold} {pos}/{len} {msg}")
            .progress_chars("█  "),
    );

    return pb;
}
