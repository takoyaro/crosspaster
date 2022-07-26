use std::env;

fn main() {
    use enigo::*;
    let mut enigo = Enigo::new();
    // let args: Vec<String> = env::args().collect();
    // println!("My path is {}.", args[0]);
    // println!("I got {:?} arguments: {:?}.", args.len() - 1, &args[1..]);
    // if args[1]=="f1"{
    //     enigo.key_click(Key::F1);
    // }
    // if args[1]=="f2"{
    //     enigo.key_click(Key::F2);
    // }
    // if args[1]=="f11"{
    //     enigo.key_click(Key::F11);
    // }
    // if args[1]=="enter"{
    //     enigo.key_click(Key::Return);
    // }
    // if args[1]=="seq"{
    //     let sequence = &args[2..].join(" ");
    //     enigo.key_sequence(sequence)
    // }
    // if args[1]=="mouseL"{
    //     enigo.mouse_click(MouseButton::Left);
    // }
    // if args[1]=="w"{
    //     enigo.key_click(Key::Layout('w'));
    // }
    // if args[1]=="a"{
    //     enigo.key_click(Key::Layout('a'));
    // }
    // if args[1]=="s"{
    //     enigo.key_click(Key::Layout('s'));
    // }
    // if args[1]=="d"{
    //     enigo.key_click(Key::Layout('d'));
    // }
    // //paste
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    
    enigo.key_up(Key::Control);
}