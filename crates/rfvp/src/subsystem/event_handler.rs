use crate::subsystem::world::GameData;

#[cfg(any(feature = "gpu-render", feature = "soft-render-desktop"))]
mod winit_input;
#[cfg(any(feature = "gpu-render", feature = "soft-render-desktop"))]
pub(crate) use winit_input::from_window_event;

use super::resources::input_manager::KeyCode;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MenuKey {
    Escape,
    Confirm,
    Tab,
    Fullscreen,
    Left,
    Right,
    Up,
    Down,
    Home,
    End,
    PageUp,
    PageDown,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PointerButton {
    Primary,
    Secondary,
    Other,
}

#[derive(Clone, Copy, Debug)]
pub enum Wheel {
    Lines(f32),
    Pixels(f64),
}

#[derive(Clone, Debug)]
pub enum Input {
    Key {
        physical: Option<MenuKey>,
        logical: Option<KeyCode>,
        pressed: bool,
        repeat: bool,
    },
    PointerMove { x: f64, y: f64 },
    PointerButton { button: PointerButton, pressed: bool },
    Wheel(Wheel),
    PointerEntered,
    PointerLeft,
    Focus(bool),
    Other,
}

/// Update the engine input state from host events in physical pixels.
///
/// Because we present the virtual render target into the window using an
/// aspect-ratio-preserving scale + letterboxing (see `App::render_frame` present
/// pass), we must apply the inverse transform here so that `cursor_x/cursor_y`
/// match the coordinate space expected by `PrimHit` and all input syscalls.
pub fn update_input_events(
    input: &Input,
    data: &mut GameData,
    surface_size: (u32, u32),
    virtual_size: (u32, u32),
) {
    match input {
        Input::Key { logical: Some(key), pressed, repeat, .. } => {
            if *pressed {
                data.inputs_manager.notify_keycode_down(key.clone(), *repeat);
            } else {
                data.inputs_manager.notify_keycode_up(key.clone());
            }
        }
        Input::PointerButton { button, pressed } => {
            let key = match button {
                PointerButton::Primary => KeyCode::MouseLeft,
                PointerButton::Secondary => KeyCode::MouseRight,
                PointerButton::Other => return,
            };
            if *pressed {
                data.inputs_manager.notify_mouse_down(key);
            } else {
                data.inputs_manager.notify_mouse_up(key);
            }
        }
        Input::Wheel(Wheel::Lines(y)) => data.inputs_manager.notify_mouse_wheel(*y as i32),
        Input::PointerMove { x: px, y: py } => {
            // Map from window physical pixels -> virtual game pixels using the presented
            // content rectangle. WindowMode does not assign a special host presentation path
            // to internal render_flag==3 in this port.

            let (sw_u, sh_u) = surface_size;
            let (vw_u, vh_u) = virtual_size;

            let sw = sw_u.max(1) as f64;
            let sh = sh_u.max(1) as f64;
            let vw = vw_u.max(1) as f64;
            let vh = vh_u.max(1) as f64;

            let (px, py) = (*px, *py);

            // Keep-aspect letterbox.
            let scale = (sw / vw).min(sh / vh);
            let dst_w = vw * scale;
            let dst_h = vh * scale;
            let off_x = (sw - dst_w) * 0.5;
            let off_y = (sh - dst_h) * 0.5;

            let in_content =
                px >= off_x && px < (off_x + dst_w) && py >= off_y && py < (off_y + dst_h);

            // Convert to virtual coordinates (clamped to the virtual bounds when inside).
            let mut vx = ((px - off_x) / scale) as i32;
            let mut vy = ((py - off_y) / scale) as i32;
            if in_content {
                let max_x = (vw as i32).saturating_sub(1);
                let max_y = (vh as i32).saturating_sub(1);
                vx = vx.clamp(0, max_x);
                vy = vy.clamp(0, max_y);
            }

            data.inputs_manager.notify_mouse_move(vx, vy);
            // Treat the cursor as "in screen" only when it is inside the presented
            // virtual content area (exclude letterboxed regions).
            data.inputs_manager.set_mouse_in(in_content);
        }
        Input::PointerEntered => {
            data.inputs_manager.set_mouse_in(true);
        }
        Input::PointerLeft => {
            data.inputs_manager.set_mouse_in(false);
        }
        Input::Focus(focused) => {
            // Original engine flushes input states on WM_ACTIVATEAPP.
            // Without this, we can keep stale pressed bits when focus transitions happen
            // (including the initial activation), which leads to unintended auto-click/skip.
            data.inputs_manager.set_flash();
            if *focused {
                // Eat the activation click (common on some platforms / backends).
                data.inputs_manager.suppress_next_mouse_click();

                // IMPORTANT: do not leave cursor_in stuck at false after a focus regain.
                // Some platforms do not emit CursorMoved/Entered on focus transitions.
                // The original engine's hit-testing logic keeps working as long as the
                // cursor is still inside the client area.
                data.inputs_manager.set_mouse_in(true);
            }
        }
        _ => {}
    };
}
