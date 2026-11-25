use super::{Input, MenuKey, PointerButton, Wheel};
use crate::subsystem::resources::input_manager::KeyCode;
use winit::event::{MouseButton, MouseScrollDelta, WindowEvent};
use winit::keyboard::{Key, KeyCode as ScanCode, NamedKey, PhysicalKey};

pub(crate) fn from_window_event(event: &WindowEvent) -> Input {
    match event {
        WindowEvent::KeyboardInput { event, .. } => Input::Key {
            physical: menu_key(event.physical_key),
            logical: keycode(&event.logical_key),
            pressed: event.state.is_pressed(),
            repeat: event.repeat,
        },
        WindowEvent::CursorMoved { position, .. } => Input::PointerMove {
            x: position.x,
            y: position.y,
        },
        WindowEvent::MouseInput { state, button, .. } => Input::PointerButton {
            button: match button {
                MouseButton::Left => PointerButton::Primary,
                MouseButton::Right => PointerButton::Secondary,
                _ => PointerButton::Other,
            },
            pressed: state.is_pressed(),
        },
        WindowEvent::MouseWheel { delta, .. } => Input::Wheel(match delta {
            MouseScrollDelta::LineDelta(_, y) => Wheel::Lines(*y),
            MouseScrollDelta::PixelDelta(position) => Wheel::Pixels(position.y),
        }),
        WindowEvent::CursorEntered { .. } => Input::PointerEntered,
        WindowEvent::CursorLeft { .. } => Input::PointerLeft,
        WindowEvent::Focused(focused) => Input::Focus(*focused),
        _ => Input::Other,
    }
}

fn menu_key(key: PhysicalKey) -> Option<MenuKey> {
    let PhysicalKey::Code(key) = key else { return None };
    Some(match key {
        ScanCode::Escape => MenuKey::Escape,
        ScanCode::Enter | ScanCode::NumpadEnter | ScanCode::Space => MenuKey::Confirm,
        ScanCode::Tab => MenuKey::Tab,
        ScanCode::F11 => MenuKey::Fullscreen,
        ScanCode::ArrowLeft => MenuKey::Left,
        ScanCode::ArrowRight => MenuKey::Right,
        ScanCode::ArrowUp => MenuKey::Up,
        ScanCode::ArrowDown => MenuKey::Down,
        ScanCode::Home => MenuKey::Home,
        ScanCode::End => MenuKey::End,
        ScanCode::PageUp => MenuKey::PageUp,
        ScanCode::PageDown => MenuKey::PageDown,
        _ => return None,
    })
}

fn keycode(key: &Key) -> Option<KeyCode> {
    let Key::Named(key) = key else { return None };
    Some(match key {
        NamedKey::Shift => KeyCode::Shift,
        NamedKey::Control => KeyCode::Ctrl,
        NamedKey::Escape => KeyCode::Esc,
        NamedKey::Enter => KeyCode::Enter,
        NamedKey::Space => KeyCode::Space,
        NamedKey::ArrowUp => KeyCode::UpArrow,
        NamedKey::ArrowDown => KeyCode::DownArrow,
        NamedKey::ArrowLeft => KeyCode::LeftArrow,
        NamedKey::ArrowRight => KeyCode::RightArrow,
        NamedKey::F1 => KeyCode::F1,
        NamedKey::F2 => KeyCode::F2,
        NamedKey::F3 => KeyCode::F3,
        NamedKey::F4 => KeyCode::F4,
        NamedKey::F5 => KeyCode::F5,
        NamedKey::F6 => KeyCode::F6,
        NamedKey::F7 => KeyCode::F7,
        NamedKey::F8 => KeyCode::F8,
        NamedKey::F9 => KeyCode::F9,
        NamedKey::F10 => KeyCode::F10,
        NamedKey::F11 => KeyCode::F11,
        NamedKey::F12 => KeyCode::F12,
        NamedKey::Tab => KeyCode::Tab,
        _ => return None,
    })
}
