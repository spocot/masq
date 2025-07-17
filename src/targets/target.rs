use crate::Theme;

pub trait MasqTarget {
    fn get_name(&self) -> &'static str;
    fn apply(&self, theme: &Theme) -> Result<(), String>;
}

macro_rules! masq_targets {
    ($($name:ident => {$($fun:tt)*})*) => {
        $(
            pub struct $name;

            impl MasqTarget for $name {
                $($fun)*

                fn get_name(&self) -> &'static str {
                    stringify!($name)
                }
            }
        )*
    };
}

masq_targets! {
    SwayTarget => {
        fn apply(&self, theme: &Theme) -> Result<(), String> {
            sway_apply(theme).map_err(|e| format!("Failed to apply theme to Sway: {}", e))
        }
    }

    Gtk3Target => {
        fn apply(&self, theme: &Theme) -> Result<(), String> {
            gtk3_apply(theme).map_err(|e| format!("Failed to apply theme to GTK 3: {}", e))
        }
    }
}

fn sway_apply(theme: &Theme) -> Result<(), String> {
    // This function can be used to apply the theme to Sway
    println!("Applying theme to Sway: {:?}", theme);
    Ok(())
}

fn gtk3_apply(theme: &Theme) -> Result<(), String> {
    // This function can be used to apply the theme to GTK 3
    println!("Applying theme to GTK 3: {:?}", theme);
    Ok(())
}
