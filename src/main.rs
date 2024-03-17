use gtk::prelude::*;

fn build_ui(app: &gtk::Application) {
    let glade_src = include_str!("layout.glade");
    let builder = gtk::Builder::from_string(glade_src);

    let win: gtk::Window = builder.object(
        "applicationwindow1"
        ).unwrap();
    win.set_application(Some(app));

    // Switch
    let is_dead_switch: gtk::Switch = builder.object("is_dead_switch").unwrap();

    // Inputs
    let msg_input: gtk::Entry = builder.object("message_input").unwrap();

    // Submit button
    let btn: gtk::Button = builder.object("generate_btn").unwrap();

    //Outputs
    let msg_output: gtk::Label = builder.object("message_output").unwrap();

    let img_output: gtk::Image = builder.object("image_output").unwrap();
    let img_output_clone = img_output.clone();


    btn.connect_clicked(move |_| {
        let is_dead = is_dead_switch.is_active();
        if is_dead {
            img_output_clone.set_from_file(
                Some("./images/cat_dead.png"))
        } else {
            img_output_clone.set_from_file(
                Some("./images/cat.png"))
        }
        msg_output.set_text(&format!(
                "{}\n \\\n \\",
                msg_input.text().as_str()
                ));
        img_output_clone.show();
    });

    win.show_all();
    img_output.hide();
}

fn main() {
    let app = gtk::Application::new(
        Some("com.catsay-gui-glade"),
        Default::default()
        );

    app.connect_activate(build_ui);
    app.run();
}
