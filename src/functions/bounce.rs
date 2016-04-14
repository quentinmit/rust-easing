use super::ease::Easing;
struct Bounce;

impl Easing for Bounce {
    fn ease_in(t: f32, b: f32, c: f32, d: f32) -> f32 {
        c - Bounce::ease_out(d - t, 0.0, c, d) + b
    }

    fn ease_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        let mut inner_t = t / d;
        if inner_t < 1.0 / 2.75 {
            c * (7.5625 * inner_t * inner_t) + b
        } else if inner_t < 2.0 / 2.75 {
            inner_t -= 1.5 / 2.75;
            c * (7.5625 * inner_t * inner_t + 0.75) + b
        } else if inner_t < 2.5 / 2.75 {
            inner_t -= 2.25 / 2.75;
            c * (7.5625 * inner_t * inner_t + 0.9375) + b
        } else {
            inner_t -= 2.625 / 2.75;
            c * (7.5625 * inner_t * inner_t + 0.984375) + b
        }
    }


    fn ease_in_out(t: f32, b: f32, c: f32, d: f32) -> f32 {
        if t < (d / 2.0) {
            Bounce::ease_in(t * 2.0, 0.0, c, d) * 0.5 + b
        } else {
            Bounce::ease_out(t * 2.0 - d, 0.0, c, d) * 0.5 + c * 0.5 + b
        }

    }
}

mod test {
    #[test]
    fn test_ease_out() {
        use functions::ease::Easing;
        relative_eq!(super::Bounce::ease_out(1.0, 2.0, 3.0, 4.0), 3.4179);
        relative_eq!(super::Bounce::ease_out(1.0, 2.0, 3.0, 2.0), 4.2968);
    }

}
