use gtk::prelude::*;
use gtk::{glib, Application, ApplicationWindow, Box, Button, HeaderBar, ListBox, ListBoxRow, 
          Orientation, Paned, ScrolledWindow, Label, Entry, Switch, SpinButton, Adjustment, Image, 
          MessageDialog, MessageType, ButtonsType, ResponseType};
use std::process::Command;
use std::env;
use std::fs;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::SystemTime;

const APP_ID: &str = "org.ummitos.settings";

// UmmItOS Detection
fn check_system_requirements() -> bool {

    // Initialize GTK for dialogs
    gtk::init().expect("Failed to initialize GTK");
    
    // Check Arch Linux or not. just check if pacman exists
    if !std::path::Path::new("/etc/pacman.conf").exists() {
        show_error_dialog_gtk("System Requirement Error", 
            "This application is designed for UmmItOS only.\n\nUmmItOS is required to run this application.");
        return false;
    }
    
    // Check Hyprland, check env var or if hyprctl works
    if std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_default().to_lowercase().contains("hyprland") 
        || std::process::Command::new("hyprctl").arg("version").output().map(|o| o.status.success()).unwrap_or(false) {
        println!("UmmItOS + Hyprland detected - Starting application...");
        return true;
    }
    
    show_error_dialog_gtk("Window Manager Error", 
        "This application requires Hyprland window manager.\n\nPlease start Hyprland or run this within a Hyprland session.");
    false
}

// Show GTK error dialog
fn show_error_dialog_gtk(title: &str, message: &str) {
    let dialog = MessageDialog::builder()
        .message_type(MessageType::Error)
        .buttons(ButtonsType::Ok)
        .title(title)
        .text(message)
        .modal(true)
        .build();

    dialog.set_icon_name(Some("dialog-error"));
    
    dialog.connect_response(|dialog, response| {
        if response == ResponseType::Ok {
            dialog.close();
        }
    });
    
    dialog.show();
    
    let main_context = glib::MainContext::default();
    while dialog.is_visible() {
        main_context.iteration(true);
    }
}

fn main() -> glib::ExitCode {
    // Check system requirements before starting
    if !check_system_requirements() {
        return glib::ExitCode::FAILURE;
    }
    
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

    // Add sidebar items with icons
    let sidebar_items = vec![
        ("System", "preferences-system-symbolic"),
        ("Record", "media-record-symbolic"),
        ("About", "help-about-symbolic"),
    ];

    for (name, icon_name) in sidebar_items {
        let row = ListBoxRow::new();
        
        // Create a horizontal box for icon + label
        let item_box = Box::builder()
            .orientation(Orientation::Horizontal)
            .spacing(8)
            .margin_top(8)
            .margin_bottom(8)
            .margin_start(12)
            .margin_end(12)
            .build();

        // Create icon
        let icon = Image::builder()
            .icon_name(icon_name)
            .icon_size(gtk::IconSize::Normal)
            .build();

        // Create label
        let label = Label::builder()
            .label(name)
            .halign(gtk::Align::Start)
            .build();
        
        item_box.append(&icon);
        item_box.append(&label);
        row.set_child(Some(&item_box));
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
        .width_chars(30)
        .max_width_chars(40)
        .build();

    // Buttons container
    let buttons_container = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(4)
        .build();

    // Path validation button with checkmark icon
    let validate_btn = Button::builder()
        .icon_name("emblem-ok-symbolic")
        .tooltip_text("Validate directory path")
        .build();

    // Apply button to save the path
    let apply_btn = Button::builder()
        .icon_name("document-save-symbolic")
        .tooltip_text("Apply this directory path")
        .build();

    // Add CSS classes for styling
    validate_btn.add_css_class("suggested-action");
    apply_btn.add_css_class("accent");

    // Current applied path (shared state)
    let current_path = Rc::new(RefCell::new(default_dir.clone()));

    // Connect validation logic
    {
        let dir_entry_clone = dir_entry.clone();
        let validate_btn_clone = validate_btn.clone();
        
        validate_btn.connect_clicked(move |_| {
            let path = dir_entry_clone.text().to_string();
            validate_directory_path(&path, &validate_btn_clone);
        });
    }

    // Connect apply logic
    {
        let dir_entry_clone = dir_entry.clone();
        let current_path_clone = current_path.clone();
        let apply_btn_clone = apply_btn.clone();
        
        apply_btn.connect_clicked(move |_| {
            let path = dir_entry_clone.text().to_string();
            apply_directory_path(&path, &current_path_clone, &apply_btn_clone);
        });
    }

    buttons_container.append(&validate_btn);
    buttons_container.append(&apply_btn);

    dir_row.append(&dir_label);
    dir_row.append(&dir_entry);
    dir_row.append(&buttons_container);
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
        .sensitive(false)
        .build();

    // Open recordings folder button
    let open_folder_btn = Button::builder()
        .label("Open Recordings Folder")
        .build();

    // Connect start button
    {
        let status_label_clone = status_label.clone();
        let current_path_clone = current_path.clone();
        let is_recording_clone = is_recording.clone();
        let start_btn_clone = start_btn.clone();
        let stop_btn_clone = stop_btn.clone();
        
        start_btn.connect_clicked(move |_| {
            if *is_recording_clone.borrow() {
                status_label_clone.set_text("Error: Recording already in progress");
                return;
            }
            
            let recording_dir = current_path_clone.borrow().clone();
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
        let current_path_clone = current_path.clone();
        let is_recording_clone = is_recording.clone();
        let start_btn_clone = start_btn.clone();
        let stop_btn_clone = stop_btn.clone();
        
        stop_btn.connect_clicked(move |_| {
            if !*is_recording_clone.borrow() {
                status_label_clone.set_text("Error: No recording in progress");
                return;
            }
            
            let recording_dir = current_path_clone.borrow().clone();
            stop_recording(&recording_dir, &status_label_clone);
            *is_recording_clone.borrow_mut() = false;
            start_btn_clone.set_sensitive(true);
            stop_btn_clone.set_sensitive(false);
        });
    }

    // Connect open folder button
    {
        let current_path_clone = current_path.clone();
        open_folder_btn.connect_clicked(move |_| {
            let recording_dir = current_path_clone.borrow().clone();
            open_recordings_folder(&recording_dir);
        });
    }

    buttons_box.append(&start_btn);
    buttons_box.append(&stop_btn);
    buttons_box.append(&open_folder_btn);

    section_box.append(&buttons_box);

    let file_list_section = create_file_list_section(&current_path);
    section_box.append(&file_list_section);

    section_box
}

fn create_file_list_section(current_path: &Rc<RefCell<String>>) -> gtk::Expander {
    let expander = gtk::Expander::builder()
        .label("Recording Files")
        .margin_top(16)
        .build();

    // Content container
    let content_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(4)
        .margin_top(8)
        .build();

    // Header with refresh button
    let header_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .margin_bottom(8)
        .build();

    // Header with icon and text
    let header_content = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(6)
        .hexpand(true)
        .build();

    let header_icon = Image::builder()
        .icon_name("folder-videos-symbolic")
        .icon_size(gtk::IconSize::Normal)
        .build();

    let files_label = Label::builder()
        .label("Available recording files:")
        .halign(gtk::Align::Start)
        .build();

    let refresh_btn = Button::builder()
        .icon_name("view-refresh-symbolic")
        .tooltip_text("Refresh file list")
        .build();

    header_content.append(&header_icon);
    header_content.append(&files_label);
    header_box.append(&header_content);
    header_box.append(&refresh_btn);
    content_box.append(&header_box);

    // Scrollable area for file list
    let scrolled = ScrolledWindow::builder()
        .height_request(200)
        .hscrollbar_policy(gtk::PolicyType::Never)
        .vscrollbar_policy(gtk::PolicyType::Automatic)
        .build();

    let files_listbox = gtk::ListBox::builder()
        .selection_mode(gtk::SelectionMode::None)
        .build();

    scrolled.set_child(Some(&files_listbox));
    content_box.append(&scrolled);

    // Function to refresh file list
    let refresh_files = {
        let current_path_clone = current_path.clone();
        let files_listbox_clone = files_listbox.clone();
        
        Rc::new(move || {
            // Clear existing items
            while let Some(child) = files_listbox_clone.first_child() {
                files_listbox_clone.remove(&child);
            }

            let recording_dir = current_path_clone.borrow().clone();
            let files = list_recording_files(&recording_dir);

            if files.is_empty() {
                // Show empty state
                let empty_row = create_empty_file_row();
                files_listbox_clone.append(&empty_row);
            } else {
                // Add file rows with refresh callback (we'll pass a dummy callback for now)
                for file_info in files {
                    let file_row = create_file_row(file_info, &recording_dir);
                    files_listbox_clone.append(&file_row);
                }
            }
        })
    };

    // Connect refresh button
    {
        let refresh_files_clone = refresh_files.clone();
        refresh_btn.connect_clicked(move |_| {
            refresh_files_clone();
        });
    }

    // Auto-refresh when expander is opened
    {
        let refresh_files_clone = refresh_files.clone();
        expander.connect_expanded_notify(move |expander| {
            if expander.is_expanded() {
                refresh_files_clone();
            }
        });
    }

    {
        let refresh_files_clone = refresh_files.clone();
        let expander_clone = expander.clone();
        
        glib::timeout_add_seconds_local(1, move || {
            if expander_clone.is_expanded() {
                refresh_files_clone();
            }
            glib::ControlFlow::Continue
        });
    }

    // Initial load
    refresh_files();

    expander.set_child(Some(&content_box));
    expander
}

// File information structure
#[derive(Debug, Clone)]
struct FileInfo {
    name: String,
    size: String,
    modified: String,
}

// List recording files in directory
fn list_recording_files(recording_dir: &str) -> Vec<FileInfo> {
    use std::fs;
    use std::time::SystemTime;
    
    let mut files = Vec::new();
    
    if let Ok(entries) = fs::read_dir(recording_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_file() {
                    let file_name = entry.file_name().to_string_lossy().to_string();
                    
                    // Only include video files
                    if file_name.ends_with(".mp4") || 
                       file_name.ends_with(".mkv") || 
                       file_name.ends_with(".webm") ||
                       file_name.contains("wf-recorder") {
                        
                        let size = format_file_size(metadata.len());
                        let modified = format_modified_time(metadata.modified().unwrap_or(SystemTime::UNIX_EPOCH));
                        
                        files.push(FileInfo {
                            name: file_name,
                            size,
                            modified,
                        });
                    }
                }
            }
        }
    }
    
    // Sort by modification time (newest first)
    files.sort_by(|a, b| b.modified.cmp(&a.modified));
    files
}

// Create a file row widget (keeping original implementation)
fn create_file_row(file_info: FileInfo, recording_dir: &str) -> Box {
    let row_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(8)
        .margin_start(4)
        .margin_end(4)
        .margin_top(2)
        .margin_bottom(2)
        .build();

    // File icon
    let file_icon = Image::builder()
        .icon_name("video-x-generic-symbolic")
        .icon_size(gtk::IconSize::Large)
        .build();

    // File info container
    let info_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(2)
        .hexpand(true)
        .build();

    // File name (truncated if too long)
    let display_name = if file_info.name.len() > 40 {
        format!("{}...", &file_info.name[..37])
    } else {
        file_info.name.clone()
    };

    let name_label = Label::builder()
        .label(&format!("<b>{}</b>", glib::markup_escape_text(&display_name)))
        .use_markup(true)
        .halign(gtk::Align::Start)
        .ellipsize(gtk::pango::EllipsizeMode::End)
        .build();

    let details_label = Label::builder()
        .label(&format!("{} • {}", file_info.size, file_info.modified))
        .halign(gtk::Align::Start)
        .build();
    
    details_label.add_css_class("dim-label");

    info_box.append(&name_label);
    info_box.append(&details_label);

    // Action buttons
    let actions_box = Box::builder()
        .orientation(Orientation::Horizontal)
        .spacing(4)
        .build();

    let play_btn = Button::builder()
        .icon_name("media-playback-start-symbolic")
        .tooltip_text("Play video")
        .build();

    let delete_btn = Button::builder()
        .icon_name("user-trash-symbolic")
        .tooltip_text("Delete file")
        .build();

    // Connect play button
    {
        let file_path = format!("{}/{}", recording_dir, file_info.name);
        play_btn.connect_clicked(move |_| {
            let _ = std::process::Command::new("xdg-open")
                .arg(&file_path)
                .spawn();
        });
    }

    // Connect delete button
    {
        let file_path = format!("{}/{}", recording_dir, file_info.name);
        let file_name = file_info.name.clone();
        delete_btn.connect_clicked(move |_| {
            show_delete_confirmation(&file_path, &file_name);
        });
    }

    actions_box.append(&play_btn);
    actions_box.append(&delete_btn);

    row_box.append(&file_icon);
    row_box.append(&info_box);
    row_box.append(&actions_box);

    row_box
}

// Create empty state row
fn create_empty_file_row() -> Box {
    let row_box = Box::builder()
        .orientation(Orientation::Vertical)
        .spacing(8)
        .margin_top(20)
        .margin_bottom(20)
        .halign(gtk::Align::Center)
        .build();

    let empty_icon = Image::builder()
        .icon_name("folder-videos-symbolic")
        .icon_size(gtk::IconSize::Large)
        .build();
    empty_icon.add_css_class("dim-icon");

    let empty_label = Label::builder()
        .label("No recording files found")
        .build();
    empty_label.add_css_class("dim-label");

    let hint_label = Label::builder()
        .label("Start recording to see files here")
        .build();
    hint_label.add_css_class("dim-label");

    row_box.append(&empty_icon);
    row_box.append(&empty_label);
    row_box.append(&hint_label);

    row_box
}

// Format file size
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.1} {}", size, UNITS[unit_index])
    }
}

// Format modification time
fn format_modified_time(system_time: SystemTime) -> String {
    use chrono::{DateTime, Local};
    
    let datetime: DateTime<Local> = system_time.into();
    datetime.format("%Y-%m-%d %H:%M").to_string()
}

// Show delete confirmation dialog
fn show_delete_confirmation(file_path: &str, file_name: &str) {
    use std::fs;
    
    // Simple confirmation via dialog
    let confirmation = std::process::Command::new("zenity")
        .args(&[
            "--question",
            "--text",
            &format!("Are you sure you want to delete '{}'?\n\nThis action cannot be undone.", file_name),
            "--title",
            "Delete Recording File"
        ])
        .status();
    
    if let Ok(status) = confirmation {
        if status.success() {
            // User confirmed deletion
            match fs::remove_file(file_path) {
                Ok(_) => {
                    println!("Deleted file: {} (Auto-refresh will update list within 3 seconds)", file_path);
                },
                Err(e) => {
                    eprintln!("Failed to delete file {}: {}", file_path, e);
                }
            }
        }
    }
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

fn apply_directory_path(path: &str, current_path: &Rc<RefCell<String>>, button: &Button) {
    println!("Applying directory path: {}", path);
    
    // First validate the path
    match fs::create_dir_all(path) {
        Ok(_) => {
            // Path is valid, apply it
            *current_path.borrow_mut() = path.to_string();
            
            button.set_icon_name("emblem-ok-symbolic");
            button.set_tooltip_text(Some("Directory path applied successfully"));
            button.add_css_class("success");
            button.remove_css_class("destructive-action");
            
            // Send success notification
            let _ = Command::new("hyprctl")
                .arg("notify")
                .arg("2")
                .arg("3000")
                .arg("rgb(00FF00)")
                .arg(&format!("fontsize:35 Applied recording directory: {}", path))
                .spawn();
                
            println!("Recording directory applied: {}", path);
        }
        Err(e) => {
            // Path is invalid, can't apply
            println!("Cannot apply invalid path: {}", e);
            button.set_icon_name("dialog-error-symbolic");
            button.set_tooltip_text(Some(&format!("Cannot apply invalid path: {}", e)));
            button.remove_css_class("success");
            button.add_css_class("destructive-action");
            
            // Send error notification
            let _ = Command::new("hyprctl")
                .arg("notify")
                .arg("0")
                .arg("3000")
                .arg("rgb(FF0000)")
                .arg(&format!("fontsize:35 Cannot apply invalid path: {}", e))
                .spawn();
        }
    }
}

fn validate_directory_path(path: &str, button: &Button) {
    println!("Validating directory path: {}", path);
    
    // Check if path is valid and can be created
    match fs::create_dir_all(path) {
        Ok(_) => {
            // Path is valid and directory exists/was created
            button.set_icon_name("emblem-ok-symbolic");
            button.set_tooltip_text(Some("Directory path is valid"));
            button.add_css_class("suggested-action");
            button.remove_css_class("destructive-action");
            
            // Send success notification
            let _ = Command::new("hyprctl")
                .arg("notify")
                .arg("2")
                .arg("3000")
                .arg("rgb(00FF00)")
                .arg(&format!("fontsize:35 Directory validated: {}", path))
                .spawn();
        }
        Err(e) => {
            // Path is invalid
            println!("Directory validation failed: {}", e);
            button.set_icon_name("dialog-error-symbolic");
            button.set_tooltip_text(Some(&format!("Invalid path: {}", e)));
            button.remove_css_class("suggested-action");
            button.add_css_class("destructive-action");
            
            // Send error notification
            let _ = Command::new("hyprctl")
                .arg("notify")
                .arg("0")
                .arg("3000")
                .arg("rgb(FF0000)")
                .arg(&format!("Invalid directory path: {}", e))
                .spawn();
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