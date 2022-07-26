fn main() {
    use enigo::*;
    let mut enigo = Enigo::new();
    enigo.key_down(Key::Control);
    enigo.key_click(Key::Layout('v'));
    
    enigo.key_up(Key::Control);
}