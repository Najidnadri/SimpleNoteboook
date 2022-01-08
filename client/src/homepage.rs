use egui::{Visuals, Rect, Pos2, Vec2};

use crate::{handler::{Event, GRAY, LIGHT_YELLOW, Page, ServerResponse, send_request, ChunkDetails}, Action};


pub fn home_page(event: &mut Event, ctx: &egui::CtxRef) {
    event.msg = "CONGRATS MY BOY YOU ARE REGISTERED!!".to_string();
    let mut visuals = Visuals::default();
    visuals.faint_bg_color = GRAY;
    visuals.dark_mode = false;
    visuals.override_text_color = Some(LIGHT_YELLOW);
    
    let _login_card = Rect::from_center_size(Pos2::new(500.0, 600.0), Vec2::new(300.0, 500.0));

    egui::TopBottomPanel::top("header").default_height(100.).show(ctx, |ui| {
        ui.add_sized([100.0, 100.0],  egui::Label::new("My simple notebook").underline()); 
    });

    egui::TopBottomPanel::bottom("tail").default_height(100.).show(ctx, |ui| {
        let save_button = ui.button("Save");
        if save_button.clicked() {
            event.msg = "Saving..".to_string();
            let chunk_details = ChunkDetails {
                account: event.user.clone(),
                data: event.data.clone(),
            };
            let action = Action::SavePage(chunk_details);
            let serialized_action = serde_json::to_string(&action).expect("cannot serialized reg action");
            match send_request(serialized_action, &event.stream) {
                ServerResponse::SavedPage => {
                    event.msg = "Saved".to_string();
                },
                ServerResponse::SavedPageErr => {
                    event.msg = "Cannot save the file".to_string();
                }
                _ => {},
            }
        }
    });

    egui::CentralPanel::default().show(ctx, |ui| {
        ui.add(egui::TextEdit::multiline(&mut event.data).desired_width(f32::INFINITY).desired_rows(10))
    });
}