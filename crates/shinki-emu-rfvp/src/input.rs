use rfvp::app::embedded::session::{Input, MenuKey, PointerButton, Wheel};
use rfvp::subsystem::resources::input_manager::KeyCode;
use shinki_emu_abi::*;

fn physical(name: &str) -> Option<MenuKey> {
    Some(match name {
        "Escape" => MenuKey::Escape,
        "Enter" | "NumpadEnter" | "Space" => MenuKey::Confirm,
        "Tab" => MenuKey::Tab,
        "F11" => MenuKey::Fullscreen,
        "ArrowLeft" => MenuKey::Left,
        "ArrowRight" => MenuKey::Right,
        "ArrowUp" => MenuKey::Up,
        "ArrowDown" => MenuKey::Down,
        "Home" => MenuKey::Home,
        "End" => MenuKey::End,
        "PageUp" => MenuKey::PageUp,
        "PageDown" => MenuKey::PageDown,
        _ => return None,
    })
}
fn logical(name: &str) -> Option<KeyCode> {
    Some(match name {
        "Escape" => KeyCode::Esc,
        "Enter" => KeyCode::Enter,
        "Space" | " " => KeyCode::Space,
        "Shift" => KeyCode::Shift,
        "Control" => KeyCode::Ctrl,
        "Tab" => KeyCode::Tab,
        "ArrowLeft" => KeyCode::LeftArrow,
        "ArrowRight" => KeyCode::RightArrow,
        "ArrowUp" => KeyCode::UpArrow,
        "ArrowDown" => KeyCode::DownArrow,
        "F1" => KeyCode::F1,
        "F2" => KeyCode::F2,
        "F3" => KeyCode::F3,
        "F4" => KeyCode::F4,
        "F5" => KeyCode::F5,
        "F6" => KeyCode::F6,
        "F7" => KeyCode::F7,
        "F8" => KeyCode::F8,
        "F9" => KeyCode::F9,
        "F10" => KeyCode::F10,
        "F11" => KeyCode::F11,
        "F12" => KeyCode::F12,
        _ => return None,
    })
}

pub unsafe fn translate(
    events: &[InputEvent],
    epoch: u64,
    newest: u64,
) -> Result<(Vec<Input>, Option<bool>), Status> {
    let mut native = Vec::with_capacity(events.len());
    let mut suspended = None;
    for event in events {
        if event.pressed > 1 || event.repeat > 1 || !event.x.is_finite() || !event.y.is_finite() {
            return Err(INVALID_ARGUMENT);
        }
        if matches!(
            event.kind,
            INPUT_POINTER_MOVE | INPUT_POINTER_BUTTON | INPUT_WHEEL
        ) && (event.displayed.session_epoch != epoch
            || event.displayed.output_epoch != 1
            || event.displayed.generation == 0
            || event.displayed.generation > newest)
        {
            return Err(STALE_CONTEXT);
        }
        match event.kind {
            INPUT_KEY => native.push(Input::Key {
                physical: physical(unsafe { event.physical_key.text(128)? }),
                logical: logical(unsafe { event.logical_key.text(128)? }),
                pressed: event.pressed != 0,
                repeat: event.repeat != 0,
            }),
            INPUT_POINTER_MOVE => native.push(Input::PointerMove { x: event.x, y: event.y }),
            INPUT_POINTER_BUTTON => {
                let button = match event.button {
                    BUTTON_PRIMARY => PointerButton::Primary,
                    BUTTON_SECONDARY => PointerButton::Secondary,
                    BUTTON_MIDDLE => PointerButton::Other,
                    _ => return Err(INVALID_ARGUMENT),
                };
                native.push(Input::PointerMove { x: event.x, y: event.y });
                native.push(Input::PointerButton { button, pressed: event.pressed != 0 });
            }
            INPUT_WHEEL => native.push(Input::Wheel(Wheel::Lines(event.y as f32))),
            INPUT_WINDOW => {
                if event.window.focused > 1
                    || event.window.suspended > 1
                    || !event.window.scale_factor.is_finite()
                    || event.window.scale_factor <= 0.0
                {
                    return Err(INVALID_ARGUMENT);
                }
                native.push(Input::Focus(event.window.focused != 0));
                suspended = Some(event.window.suspended != 0);
                // O/scale factor never mutate the configured R or script-visible L.
            }
            _ => return Err(INVALID_ARGUMENT),
        }
    }
    Ok((native, suspended))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn button(button: u32) -> InputEvent {
        InputEvent {
            kind: INPUT_POINTER_BUTTON,
            pressed: 1,
            repeat: 0,
            button,
            physical_key: Bytes::EMPTY,
            logical_key: Bytes::EMPTY,
            x: 150.0,
            y: 75.0,
            displayed: FrameIdentity {
                session_epoch: 7,
                output_epoch: 1,
                generation: 2,
            },
            window: WindowState {
                drawable_width: 1920,
                drawable_height: 1080,
                scale_factor: 2.0,
                focused: 1,
                suspended: 0,
            },
        }
    }

    #[test]
    fn frame_pixels_are_not_scaled_again_at_abi_boundary() {
        let (events, _) = unsafe { translate(&[button(BUTTON_SECONDARY)], 7, 3) }.unwrap();
        assert!(matches!(
            events.as_slice(),
            [
                Input::PointerMove { x: 150.0, y: 75.0 },
                Input::PointerButton { button: PointerButton::Secondary, pressed: true }
            ]
        ));
    }

    #[test]
    fn menu_physical_keys_do_not_replace_logical_game_keys() {
        let mut event = button(0);
        event.kind = INPUT_KEY;
        event.physical_key = Bytes::new(b"NumpadEnter");
        event.logical_key = Bytes::new(b"Enter");
        let mut held = event;
        held.physical_key = Bytes::new(b"ControlLeft");
        held.logical_key = Bytes::new(b"Control");
        held.repeat = 1;
        let (events, _) = unsafe { translate(&[event, held], 7, 3) }.unwrap();
        assert!(matches!(
            events.as_slice(),
            [
                Input::Key { physical: Some(MenuKey::Confirm), logical: Some(KeyCode::Enter), pressed: true, repeat: false },
                Input::Key { physical: None, logical: Some(KeyCode::Ctrl), pressed: true, repeat: true }
            ]
        ));
    }

    #[test]
    fn malformed_buttons_and_old_session_inputs_are_rejected() {
        assert!(matches!(
            unsafe { translate(&[button(0)], 7, 3) },
            Err(INVALID_ARGUMENT)
        ));
        assert!(matches!(
            unsafe { translate(&[button(BUTTON_PRIMARY)], 8, 3) },
            Err(STALE_CONTEXT)
        ));
        let mut event = button(BUTTON_PRIMARY);
        event.pressed = 2;
        assert!(matches!(
            unsafe { translate(&[event], 7, 3) },
            Err(INVALID_ARGUMENT)
        ));
    }
}
