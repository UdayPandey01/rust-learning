// An enum representing different actions a user can take.
enum UserAction {
    Click { x: i32, y: i32 },
    KeyPress(char),
    ChangeColor(u8, u8, u8), // RGB color values
    NoAction,
}

// Your task is to implement this function.
fn process_action(action: UserAction) {
    // TODO: Use a `match` expression to process the `action` enum.
    match action {
        UserAction::Click {x,y} => {
            println!("Clicked at coordinates x:{}, y:{}", x, y);
        }
        UserAction::KeyPress(c) => {
            println!("presses the key: {}", c);
        }
        UserAction::ChangeColor(x,y,z) => {
            println!("Changed color to R:{}, G:{}, B:{}", x,y,z);
        }
        UserAction::NoAction => {
            println!("No action occurred.");
        }
    }
    // 1. For the `Click` variant, print "Clicked at coordinates x:{}, y:{}".
    // 2. For the `KeyPress` variant, print "Pressed the key: {}".
    // 3. For the `ChangeColor` variant, print "Changed color to R:{}, G:{}, B:{}".
    // 4. For the `NoAction` variant, print "No action occurred.".
}

fn main() {
    // You can test your function with different variants here.
    let action1 = UserAction::Click { x: 100, y: 250 };
    let action2 = UserAction::KeyPress('R');
    let action3 = UserAction::ChangeColor(255, 0, 128);
    let action4 = UserAction::NoAction;

    process_action(action1);
    process_action(action2);
    process_action(action3);
    process_action(action4);
}