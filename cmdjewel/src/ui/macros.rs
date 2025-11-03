/// Creates a vertical spacer of size $size, or 1 by default
#[macro_export]
macro_rules! vspacer {
    () => {
        vspacer!(1)
    };

    ($size:expr) => {{
        let n: usize = $size; // Force types to be unsigned integers
        TextView::new("\n".repeat(n))
    }};
}

/// Creates a horizontal spacer of size $size, or 1 by default
#[macro_export]
macro_rules! hspacer {
    () => {
        hspacer!(1)
    };

    ($size:expr) => {{
        let n: usize = $size; // Force types to be unsigned integers
        TextView::new(" ".repeat(n))
    }};
}

/// Creates a confirmation dialog.
#[macro_export]
macro_rules! confirm {
    ($s:expr, $label:expr, $cb:expr) => {
        $s.add_layer(
            Dialog::around(TextView::new($label))
                .title(strings::ARE_SURE)
                .button(strings::YES, $cb)
                .dismiss_button(strings::NO),
        )
    };
}

/// Creates a gamemode button for the main menu, that changes `about_gamemode` when focused.
#[macro_export]
macro_rules! gamemode_btn {
    ($label:expr, $desc:expr, $cb:expr) => {
        FocusTracker::new(Button::new_raw(
            "╭───────────╮\n│".to_string()
                + format!("{: ^11}", $label).as_str()
                + "│\n╰───────────╯",
            $cb,
        ))
        .on_focus(|_| {
            EventResult::Consumed(Some(Callback::from_fn(|s| {
                s.call_on_name("about_gamemode", |view: &mut TextView| {
                    view.set_content($desc)
                });
            })))
        })
    };
}
