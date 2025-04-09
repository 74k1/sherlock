use gtk4::{
    self,
    gdk::{self, Key},
    prelude::*,
    Builder, Entry, EventControllerKey, Justification,
};

use gtk4::{Box as HVBox, ListBox, ScrolledWindow};
use std::rc::Rc;

use super::tiles::{util::TextViewTileBuilder, Tile};
use super::util::*;
use crate::g_subclasses::sherlock_row::SherlockRow;
use crate::{loader::pipe_loader::PipeData, APP_STATE};

pub fn display_pipe(pipe_content: Vec<PipeData>, method: &str) {
    // Initialize the builder with the correct path
    let builder = Builder::from_resource("/dev/skxxtz/sherlock/ui/search.ui");

    // Get the requred object references
    let vbox: HVBox = builder.object("vbox").unwrap();
    let search_bar: Entry = builder.object("search-bar").unwrap_or_default();
    let result_viewport: ScrolledWindow = builder.object("scrolled-window").unwrap();
    let results: Rc<ListBox> = Rc::new(builder.object("result-frame").unwrap());

    let keyword = search_bar.text();

    let tiles = Tile::pipe_data(&pipe_content, &method, &keyword);
    for item in tiles {
        results.append(&item);
    }

    result_viewport.set_policy(gtk4::PolicyType::Automatic, gtk4::PolicyType::Automatic);
    results.focus_first();
    search_bar.grab_focus();

    change_event(&search_bar, &results, pipe_content, &method);

    nav_event(results, result_viewport);
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.add_stack_page(vbox, "search-page");
        }
    });
}
pub fn display_raw<T: AsRef<str>>(content: T, center: bool) {
    let builder = TextViewTileBuilder::new("/dev/skxxtz/sherlock/ui/text_view_tile.ui");
    let buffer = builder.content.buffer();
    builder.content.add_css_class("raw_text");
    builder.content.set_monospace(true);
    let sanitized: String = content.as_ref().chars().filter(|&c| c != '\0').collect();
    buffer.set_text(&sanitized);
    if center {
        builder.content.set_justification(Justification::Center);
    }

    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.add_stack_page(builder.object, "search-page");
        }
    });
}
pub fn display_next<T: AsRef<str>>(content: T) {
    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            let builder = TextViewTileBuilder::new("/dev/skxxtz/sherlock/ui/text_view_tile.ui");
            builder.content.set_wrap_mode(gtk4::WrapMode::Word);
            let buf = builder.content.buffer();
            buf.set_text(content.as_ref());

            if let Some(stack) = &state.stack {
                stack.add_named(&builder.object, Some("next-page"));
                show_stack_page("next-page", Some(gtk4::StackTransitionType::SlideLeft));
            }
        }
    });
}

fn nav_event(results_ev_nav: Rc<ListBox>, result_viewport: ScrolledWindow) {
    let event_controller = EventControllerKey::new();
    event_controller.set_propagation_phase(gtk4::PropagationPhase::Capture);
    event_controller.connect_key_pressed(move |_, key, _, modifiers| {
        match key {
            gdk::Key::Up => {
                results_ev_nav.focus_prev(&result_viewport);
            }
            gdk::Key::Down => {
                results_ev_nav.focus_next(&result_viewport);
                return true.into();
            }
            gdk::Key::Return => {
                if let Some(row) = results_ev_nav
                    .selected_row()
                    .and_downcast_ref::<SherlockRow>()
                {
                    row.emit_by_name::<()>("row-should-activate", &[]);
                }
            }
            Key::_1 | Key::_2 | Key::_3 | Key::_4 | Key::_5 => {
                if modifiers.contains(gdk::ModifierType::CONTROL_MASK) {
                    let key_index = match key {
                        Key::_1 => 1,
                        Key::_2 => 2,
                        Key::_3 => 3,
                        Key::_4 => 4,
                        Key::_5 => 5,
                        _ => return false.into(),
                    };
                    execute_by_index(&*results_ev_nav, key_index);
                }
            }
            _ => (),
        }
        false.into()
    });

    APP_STATE.with(|state| {
        if let Some(ref state) = *state.borrow() {
            state.add_event_listener(event_controller);
        }
    });
}

fn change_event(
    search_bar: &Entry,
    results: &Rc<ListBox>,
    pipe_content: Vec<PipeData>,
    method: &str,
) {
    //Cloning:
    let results_ev_changed = Rc::clone(results);
    let pipe_content_clone = pipe_content.clone();
    let method = method.to_string();

    search_bar.connect_changed(move |search_bar| {
        let current_text = search_bar.text();

        while let Some(row) = results_ev_changed.last_child() {
            results_ev_changed.remove(&row);
        }
        let tiles = Tile::pipe_data(&pipe_content_clone, &method, &current_text);
        for item in tiles {
            results_ev_changed.append(&item);
        }

        results_ev_changed.focus_first();
    });
}
