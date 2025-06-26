use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, HeaderBar, ListBox, ListBoxRow, 
          Orientation, Paned, ScrolledWindow, Label, Entry, Switch, SpinButton, Adjustment};
use std::process::Command;
use std::env;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;

const APP_ID: &str = "org.ummitos.settings";

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APP_ID).build();
    
    app.connect_activate(build_ui);
    
    app.run()
}

fn build_ui(app: &Application) {
    // Create the main window
    let window = ApplicationWindow::builder()
        .application(app)
        .title("UmmItOS Settings")
        .default_width(900)
        .default_height(600)
        .build();

    // Create header bar
    let header_bar = HeaderBar::builder()
        .title_widget(&Label::new(Some("UmmItOS Settings")))
        .show_title_buttons(true)
        .build();
    
    window.set_titlebar(Some(&header_bar));

    // Create main horizontal paned layout
    let paned = Paned::builder()
        .orientation(Orientation::Horizontal)
        .position(250)
        .build();

    // Create main content area with stack for page switching
    let content_stack = gtk::Stack::builder()
        .transition_type(gtk::StackTransitionType::SlideLeftRight)
        .build();

    // Create individual pages
    let system_page = create_system_page();
    let record_page = create_record_page();
    let about_page = create_about_page();

    content_stack.add_named(&system_page, Some("system"));
    content_stack.add_named(&record_page, Some("record"));
    content_stack.add_named(&about_page, Some("about"));

    // Show system page by default
    content_stack.set_visible_child_name("system");

    // Create sidebar with reference to content stack
    let sidebar = create_sidebar(&content_stack);
    paned.set_start_child(Some(&sidebar));
    paned.set_end_child(Some(&content_stack));

    window.set_child(Some(&paned));
    window.present();
}

fn create_sidebar(content_stack: &gtk::Stack) -> ScrolledWindow {
    let scrolled_window = ScrolledWindow::builder()
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();

    let listbox = ListBox::builder()
        .selection_mode(gtk::SelectionMode::Single)
        .build();

    // Add sidebar items
    let sidebar_items = vec![
        ("System", "computer"),
        ("Record", "media-record"),
        ("About", "help-about"),
    ];

    for (name, _icon) in sidebar_items {
        let row = ListBoxRow::new();
        let label = Label::builder()
            .label(name)
            .halign(gtk::Align::Start)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(12)
            .margin_end(12)
            .build();
        
        row.set_child(Some(&label));
        listbox.append(&row);
    }

    // Connect sidebar selection handler
    let content_stack_clone = content_stack.clone();
    listbox.connect_row_selected(move |_, row| {
        if let Some(row) = row {
            let index = row.index();
            println!("Selected sidebar item: {}", index);
            
            // Switch to appropriate page based on selection
            match index {
                0 => content_stack_clone.set_visible_child_name("system"),
                1 => content_stack_clone.set_visible_child_name("record"),
                2 => content_stack_clone.set_visible_child_name("about"),
                _ => {}
            }
        }
    });

    scrolled_window.set_child(Some(&listbox));
    scrolled_window
}

fn create_system_page() -> Box {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    // Add page title
    let title_label = Label::builder()
        .label("<span size='large' weight='bold'>System Settings</span>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    
    content_box.append(&title_label);

    // Add system settings section
    let system_section = create_system_section();
    content_box.append(&system_section);

    content_box
}

fn create_record_page() -> Box {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    // Add page title
    let title_label = Label::builder()
        .label("<span size='large' weight='bold'>Screen Recording</span>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    
    content_box.append(&title_label);

    // Add recording section
    let recording_section = create_recording_section();
    content_box.append(&recording_section);

    content_box
}

fn create_about_page() -> Box {
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(12)
        .margin_top(24)
        .margin_bottom(24)
        .margin_start(24)
        .margin_end(24)
        .build();

    // Add page title
    let title_label = Label::builder()
        .label("<span size='large' weight='bold'>About UmmItOS</span>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .build();
    
    content_box.append(&title_label);

    // Add about section
    let about_section = create_about_section();
    content_box.append(&about_section);

    content_box
}

fn create_recording_section() -> Box {
    let section_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .build();

    let section_title = Label::builder()
        .label("<span weight='bold'>Screen Recording (wf-recorder)</span>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .margin_top(12)
        .build();
    
    section_box.append(&section_title);

    let description = Label::builder()
        .label("Record your screen using wf-recorder")
        .halign(gtk::Align::Start)
        .wrap(true)
        .build();
    
    section_box.append(&description);

    // Recording directory setting
    let user = env::var("USER").unwrap_or_else(|_| "user".to_string());
    let default_dir = format!("/home/{}/Videos/wf-recorder", user);
    
    let dir_row = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .margin_top(8)
        .build();

    let dir_label = Label::builder()
        .label("Recording Directory:")
        .halign(gtk::Align::Start)
        .build();

    let dir_entry = Entry::builder()
        .text(&default_dir)
        .hexpand(true)
        .build();

    dir_row.append(&dir_label);
    dir_row.append(&dir_entry);
    section_box.append(&dir_row);

    // Recording status
    let status_label = Label::builder()
        .label("Status: Ready")
        .halign(gtk::Align::Start)
        .margin_top(8)
        .build();
    
    section_box.append(&status_label);

    // Recording state - shared between buttons
    let is_recording = Rc::new(RefCell::new(false));

    // Create buttons row
    let buttons_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .margin_top(12)
        .build();

    // Start recording button
    let start_btn = Button::builder()
        .label("Start Recording")
        .build();
    
    // Stop recording button
    let stop_btn = Button::builder()
        .label("Stop Recording")
        .sensitive(false) // Initially disabled
        .build();

    // Open recordings folder button
    let open_folder_btn = Button::builder()
        .label("Open Recordings Folder")
        .build();

    // Connect start button
    {
        let status_label_clone = status_label.clone();
        let dir_entry_clone = dir_entry.clone();
        let is_recording_clone = is_recording.clone();
        let start_btn_clone = start_btn.clone();
        let stop_btn_clone = stop_btn.clone();
        
        start_btn.connect_clicked(move |_| {
            if *is_recording_clone.borrow() {
                status_label_clone.set_text("Error: Recording already in progress");
                return;
            }
            
            let recording_dir = dir_entry_clone.text().to_string();
            if start_recording(&recording_dir, &status_label_clone) {
                *is_recording_clone.borrow_mut() = true;
                start_btn_clone.set_sensitive(false);
                stop_btn_clone.set_sensitive(true);
            }
        });
    }

    // Connect stop button
    {
        let status_label_clone = status_label.clone();
        let dir_entry_clone = dir_entry.clone();
        let is_recording_clone = is_recording.clone();
        let start_btn_clone = start_btn.clone();
        let stop_btn_clone = stop_btn.clone();
        
        stop_btn.connect_clicked(move |_| {
            if !*is_recording_clone.borrow() {
                status_label_clone.set_text("Error: No recording in progress");
                return;
            }
            
            let recording_dir = dir_entry_clone.text().to_string();
            stop_recording(&recording_dir, &status_label_clone);
            *is_recording_clone.borrow_mut() = false;
            start_btn_clone.set_sensitive(true);
            stop_btn_clone.set_sensitive(false);
        });
    }

    // Connect open folder button
    {
        let dir_entry_clone = dir_entry.clone();
        open_folder_btn.connect_clicked(move |_| {
            let recording_dir = dir_entry_clone.text().to_string();
            open_recordings_folder(&recording_dir);
        });
    }

    buttons_box.append(&start_btn);
    buttons_box.append(&stop_btn);
    buttons_box.append(&open_folder_btn);

    section_box.append(&buttons_box);
    section_box
}

fn create_about_section() -> Box {
    let section_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .build();

    let section_title = Label::builder()
        .label("<span weight='bold'>About UmmItOS</span>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .margin_top(12)
        .build();
    
    section_box.append(&section_title);

    let about_text = format!(
        "UmmItOS Settings v????\n\nSystem Information:\n• User: {}\n• Home: {}\n• Desktop: {}",
        env::var("USER").unwrap_or_else(|_| "Unknown".to_string()),
        env::var("HOME").unwrap_or_else(|_| "Unknown".to_string()),
        env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| "Unknown".to_string())
    );

    let about_label = Label::builder()
        .label(&about_text)
        .halign(gtk::Align::Start)
        .valign(gtk::Align::Start)
        .wrap(true)
        .build();
    
    section_box.append(&about_label);

    // Links section
    let links_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .margin_top(12)
        .build();

    let dotfiles_btn = Button::builder()
        .label("View UmmItOS Dotfiles")
        .build();
    
    dotfiles_btn.connect_clicked(|_| {
        let _ = Command::new("xdg-open")
            .arg("https://github.com/UmmItOS/ummit-dots")
            .spawn();
    });

    links_box.append(&dotfiles_btn);
    section_box.append(&links_box);

    section_box
}

fn create_system_section() -> Box {
    let section_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .build();

    let section_title = Label::builder()
        .label("<span weight='bold'>System Settings</span>")
        .use_markup(true)
        .halign(gtk::Align::Start)
        .margin_top(12)
        .build();
    
    section_box.append(&section_title);

    // Theme setting
    let theme_switch = create_switch();
    theme_switch.set_sensitive(false);
    theme_switch.set_tooltip_text(Some("This feature is not yet implemented"));
    let theme_row = create_setting_row("Dark Theme", theme_switch);
    section_box.append(&theme_row);

    // Animations setting
    let animations_switch = create_switch();
    animations_switch.set_sensitive(false);
    animations_switch.set_tooltip_text(Some("This feature is not yet implemented"));
    let animations_row = create_setting_row("Enable Animations", animations_switch);
    section_box.append(&animations_row);

    // Transparency setting
    let transparency_adj = Adjustment::new(0.8, 0.0, 1.0, 0.1, 0.1, 0.0);
    let transparency_spin = SpinButton::new(Some(&transparency_adj), 0.1, 2);
    transparency_spin.set_sensitive(false);
    transparency_spin.set_tooltip_text(Some("This feature is not yet implemented"));
    let transparency_row = create_setting_row("Window Transparency", transparency_spin);
    section_box.append(&transparency_row);

    section_box
}

fn create_setting_row<W: IsA<gtk::Widget>>(label_text: &str, widget: W) -> Box {
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(12)
        .margin_top(4)
        .margin_bottom(4)
        .build();

    let label = Label::builder()
        .label(label_text)
        .halign(gtk::Align::Start)
        .hexpand(true)
        .build();

    row_box.append(&label);
    row_box.append(&widget);

    row_box
}

fn create_switch() -> Switch {
    Switch::builder()
        .halign(gtk::Align::End)
        .build()
}

fn start_recording(recording_dir: &str, status_label: &Label) -> bool {
    println!("Starting wf-recorder...");
    
    // Check if wf-recorder is already running
    let check_running = Command::new("pidof")
        .arg("wf-recorder")
        .output();
    
    if let Ok(output) = check_running {
        let pid_str = String::from_utf8_lossy(&output.stdout);
        if !pid_str.trim().is_empty() {
            println!("wf-recorder is already running with PID: {}", pid_str.trim());
            status_label.set_text("Error: wf-recorder is already running");
            return false;
        }
    }
    
    // Create recording directory if it doesn't exist
    if let Err(e) = fs::create_dir_all(recording_dir) {
        println!("Error creating directory {}: {}", recording_dir, e);
        status_label.set_text("Error: Failed to create recording directory");
        return false;
    }

    // Generate filename with current timestamp
    let timestamp = chrono::Local::now().format("%Y-%m-%d-%H-%M-%S");
    let filename = format!("{}/wf-recorder-{}.mp4", recording_dir, timestamp);

    // Start wf-recorder
    let result = Command::new("wf-recorder")
        .arg("-a")
        .arg("--file")
        .arg(&filename)
        .spawn();

    match result {
        Ok(_) => {
            // Update status after successful start
            status_label.set_text(&format!("Recording to: {}", filename));
            
            // Send notification via hyprctl
            let _ = Command::new("hyprctl")
                .arg("notify")
                .arg("1")
                .arg("5000")
                .arg("rgb(00FF00)")
                .arg("fontsize:35   Video recording started with wf-recorder 📹")
                .spawn();
            
            true
        }
        Err(e) => {
            println!("Failed to start wf-recorder: {}", e);
            status_label.set_text("Error: Failed to start wf-recorder");
            false
        }
    }
}

fn stop_recording(recording_dir: &str, status_label: &Label) {
    println!("Stopping wf-recorder...");
    
    // Find wf-recorder process and send SIGINT
    let output = Command::new("pidof")
        .arg("wf-recorder")
        .output();

    match output {
        Ok(output) => {
            let pid_str = String::from_utf8_lossy(&output.stdout);
            let pid_str = pid_str.trim();
            
            if !pid_str.is_empty() {
                println!("Sending SIGINT to wf-recorder PID: {}", pid_str);
                let _ = Command::new("kill")
                    .arg("-SIGINT")
                    .arg(pid_str)
                    .spawn();

                // Get most recent file
                if let Ok(entries) = fs::read_dir(recording_dir) {
                    let mut files: Vec<_> = entries
                        .filter_map(|entry| entry.ok())
                        .filter(|entry| {
                            entry.path().extension()
                                .and_then(|ext| ext.to_str())
                                .map(|ext| ext == "mp4")
                                .unwrap_or(false)
                        })
                        .collect();
                    
                    files.sort_by_key(|entry| {
                        entry.metadata()
                            .and_then(|m| m.modified())
                            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
                    });
                    
                    if let Some(latest_file) = files.last() {
                        let filename = latest_file.file_name();
                        let filename_str = filename.to_string_lossy();
                        
                        status_label.set_text(&format!("Saved: {}", filename_str));
                        
                        // Send notification
                        let notification_text = format!("fontsize:35   Video recording ended and saved to: {}/{} 📹", recording_dir, filename_str);
                        let _ = Command::new("hyprctl")
                            .arg("notify")
                            .arg("5")
                            .arg("5000")
                            .arg("rgb(00FF00)")
                            .arg(&notification_text)
                            .spawn();
                    } else {
                        status_label.set_text("Recording stopped");
                    }
                } else {
                    status_label.set_text("Recording stopped");
                }
            } else {
                println!("wf-recorder is not running");
                status_label.set_text("wf-recorder is not running");
            }
        }
        Err(e) => {
            println!("Error finding wf-recorder process: {}", e);
            status_label.set_text("Error: Could not find wf-recorder process");
        }
    }
}

fn open_recordings_folder(recording_dir: &str) {
    println!("Opening recordings folder: {}", recording_dir);
    
    // First ensure the directory exists
    if let Err(e) = fs::create_dir_all(recording_dir) {
        println!("Error creating directory {}: {}", recording_dir, e);
        return;
    }
    
    let result = Command::new("xdg-open")
        .arg(recording_dir)
        .status();
    
    if result.is_err() {
        println!("xdg-open failed, trying nautilus...");
        let result = Command::new("nautilus")
            .arg(recording_dir)
            .status();
        
        if result.is_err() {
            println!("nautilus failed, trying thunar...");
            let result = Command::new("thunar")
                .arg(recording_dir)
                .status();
            
            if result.is_err() {
                println!("thunar failed, trying dolphin...");
                let result = Command::new("dolphin")
                    .arg(recording_dir)
                    .status();
                
                if result.is_err() {
                    println!("All file managers failed. Opening terminal in directory...");
                    let _ = Command::new("kitty")
                        .arg("--directory")
                        .arg(recording_dir)
                        .spawn()
                        .or_else(|_| {
                            Command::new("gnome-terminal")
                                .arg("--working-directory")
                                .arg(recording_dir)
                                .spawn()
                        });
                }
            }
        }
    }
}