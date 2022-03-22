use crate::concept::{Area, Layout, Point, Points, Presenter, Rect};
use enumflags2::bitflags;

#[bitflags]
#[repr(u32)]
#[derive(Copy, Clone, Debug, PartialEq)]
enum TodoE {
    goal,
    done,
}

#[derive(Default)]
struct Todo {
    goal: String,
    done: bool,
}
enum SvgID {
    check,
}

fn app() {
    let mut todo = Todo::default();
    let on_check_box_click = || -> TodoE {
        todo.done = true;
        TodoE::done
    };
    let goal_change: Layout<TodoE, SvgID> =
        (TodoE::goal, SvgID::check, &|point, Area| -> Point { point });
    let presenter: Presenter<TodoE, SvgID> = Presenter {
        layouts: &[goal_change],
        callbacks: &[&on_check_box_click],
    };
}
