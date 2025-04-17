use crate::sorts::Sorts;
use macroquad::{
    color::{BLACK, WHITE},
    input::{self, MouseButton},
    shapes, text,
};

struct Button {
    x: f32,
    y: f32,
    width: f32,
    height: f32,
    text: &'static str,
    text_x: f32,
    text_y: f32,
    font_size: f32,
    ret_value: Sorts,
}

pub fn change_sort(alg: &mut Sorts) {
    let buttons = [
        Button {
            x: 10.0,
            y: 10.0,
            width: 100.0,
            height: 25.0,
            text: "Selection Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::SelectionSort,
        },
        Button {
            x: 10.0,
            y: 40.0,
            width: 100.0,
            height: 25.0,
            text: "Insertion Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::InsertionSort,
        },
        Button {
            x: 10.0,
            y: 70.0,
            width: 100.0,
            height: 25.0,
            text: "Shaker Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::CocktailShakerSort,
        },
        Button {
            x: 10.0,
            y: 100.0,
            width: 100.0,
            height: 25.0,
            text: "Quick Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::QuickSort,
        },
        Button {
            x: 10.0,
            y: 130.0,
            width: 100.0,
            height: 25.0,
            text: "Merge Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::MergeSort,
        },
        Button {
            x: 10.0,
            y: 160.0,
            width: 100.0,
            height: 25.0,
            text: "Radix Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::RadixSort,
        },
        Button {
            x: 10.0,
            y: 190.0,
            width: 100.0,
            height: 25.0,
            text: "Shell Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::ShellSort,
        },
        Button {
            x: 10.0,
            y: 220.0,
            width: 100.0,
            height: 25.0,
            text: "Bitonic Sort",
            text_x: 2.7,
            text_y: 15.0,
            font_size: 15.0,
            ret_value: Sorts::BitonicSort,
        },
    ];

    buttons.iter().for_each(draw_button);
    *alg = buttons
        .iter()
        .filter_map(|btn| {
            if btn.is_clicked() {
                Some(btn.ret_value)
            } else {
                None
            }
        })
        .next()
        .unwrap_or(*alg);
}

fn draw_button(button: &Button) {
    shapes::draw_rectangle(button.x, button.y, button.width, button.height, WHITE);
    text::draw_text(
        button.text,
        button.x + button.text_x,
        button.y + button.text_y,
        button.font_size,
        BLACK,
    );
}

impl Button {
    fn is_clicked(&self) -> bool {
        let mouse_pressed = input::is_mouse_button_pressed(MouseButton::Left);

        if !mouse_pressed {
            return false;
        }

        let (mouse_x, mouse_y) = input::mouse_position();

        mouse_x >= self.x
            && mouse_x <= self.x + self.width
            && mouse_y >= self.y
            && mouse_y <= self.y + self.height
    }
}
