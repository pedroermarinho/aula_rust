use wmctrl;

fn main() {
    let mut windows = wmctrl::get_windows();
    for item in &windows{
        
        println!("{:?}\n",item.title());
    }
    let win = &mut windows[4];
    win.
    // This will move the window to the top left corner and resize it to 960x540
    win.transform(wmctrl::desktop::switch_desktop(desktop: &str));
}
