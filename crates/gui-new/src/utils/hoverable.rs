use freya::prelude::*;

pub struct Hoverable<Animated: AnimatedValue + PartialEq + Clone + 'static> {
    #[allow(
        unused,
        reason = "Consumers don't always need to know the hover state."
    )]
    pub hovered: Signal<bool>,
    pub animation: UseAnimation<Animated>,
    pub onmouseenter: Box<dyn FnMut(Event<MouseData>)>,
    pub onmouseleave: Box<dyn FnMut(Event<MouseData>)>,
}

/// A macro to create a hoverable animation.
///
/// # Example
///
/// ```rust
/// let bg_anim = hoverable!(|_conf| {
///     AnimColor::new("rgb(52, 52, 58)", "rgb(88, 101, 242)")
///         .ease(Ease::InOut)
///         .time(100)
/// });
macro_rules! hoverable {
    ($anim:expr) => {{
        use freya::prelude::*;

        let mut hovered = use_signal(|| false);
        let animation = use_animation($anim);

        let onmouseenter = move |_: Event<MouseData>| {
            if !hovered() {
                animation.run(AnimDirection::Forward);
                hovered.set(true);
            }
        };

        let onmouseleave = move |_: Event<MouseData>| {
            if hovered() {
                hovered.set(false);
                animation.run(AnimDirection::Reverse);
            }
        };

        $crate::utils::hoverable::Hoverable {
            hovered,
            animation,
            onmouseenter: Box::new(onmouseenter),
            onmouseleave: Box::new(onmouseleave),
        }
    }};
}

pub(crate) use hoverable;
