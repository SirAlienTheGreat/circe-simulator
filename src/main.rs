use gtk4 as gtk;
use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box, Button, Label,Orientation,Image};
use std::time;
use std::time::Duration;
use std::thread::sleep;
use std::thread;
use webbrowser;

fn main() {
    let application = Application::builder()
        .application_id("circe.simulator")
        .build();
    application.connect_activate(build_ui);
    application.run();
}
fn build_ui(app: &Application) {
    //let sailor = Image::from_file("odysseus-sailor.jpg");

    // Should be a horizontal box of verticle boxes
    let mut horizontal_box = gtk::Box::new(Orientation::Horizontal,5);
    let mut verticle_boxes:Vec<Box> = vec![];

    let mut men:Vec<Button> = vec![];
    let mut sailor_pics:Vec<Image> = vec![];
    let mut pig_pics:Vec<Image> = vec![];

    for q in 0..5{
        verticle_boxes.push(gtk::Box::new(Orientation::Vertical,5));
        for i in 0..5{
            sailor_pics.push(Image::from_file("odysseus-sailor.jpg"));
            pig_pics.push(Image::from_file("pig.png"));
            men.push(Button::builder()
                .label(&["Odysseus man ".to_string(),(i+1).to_string()].join(""))
                .height_request(150)
                .width_request(150)
                .build());
            if 5*q+i !=7 { // dont add pig features if odysseus
                men[5 * q + i].set_child(Some(&sailor_pics[5 * q + i]));
                men[5 * q + i].connect_clicked(|but| {
                    but.set_label(&"pig".to_string());
                    let pig = Image::from_file("pig.png");
                    but.set_child(Some(&pig));
                });
            }
            verticle_boxes[q].append(&men[5*q+i]);
        }
        horizontal_box.append(&verticle_boxes[q]);
    }

    let odysseus = Image::from_file("odysseus.jpg");
    men[7].set_child(Some(&odysseus));

    men[7].connect_clicked(|but|{
        webbrowser::open("https://www.online-stopwatch.com/timer/year/").expect("failed to open timer in browser");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Circe simulator")
        .child(&horizontal_box)
        .build();

    window.present();
}



