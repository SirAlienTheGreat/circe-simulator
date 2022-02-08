use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label,Orientation};

fn main() {
    let application = Application::builder()
        .application_id("circe.simulator")
        .build();
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(app: &Application) {
    // Should be a horizontal box of verticle boxes
    let mut horizontal_box = gtk::Box::new(Orientation::Horizontal,5);
    let mut verticle_boxes:Vec<Box> = vec![];

    let mut men:Vec<Button> = vec![];
    for q in 0..6{
        verticle_boxes.push(gtk::Box::new(Orientation::Vertical,5));
        for i in 0..6{
            men.push(Button::builder()
                .label(&["Odysseus man ".to_string(),(i+1).to_string()].join(""))
                .build());
            men[6*q+i].connect_clicked(|but|{
                but.set_label(&"pig".to_string());
            });
            verticle_boxes[q].append(&men[6*q+i]);
        }
        horizontal_box.append(&verticle_boxes[q]);
        println!("appended box {}",q);
    }

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Circe simulator")
        .child(&horizontal_box)
        .build();

    window.present();
}
/*
fn create_element(symbol:&str,atomic_number:i32) -> Button{
    let mut but = Button::builder()
        .label(&symbol)
        .build();
    but.connect_clicked(|_|{
                            println!("Element:, with atomic number:")
});
    return but;
}*/