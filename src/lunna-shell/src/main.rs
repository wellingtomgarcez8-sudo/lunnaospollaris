use gtk::prelude::*;
use gtk::{Application, ApplicationWindow, Box as GtkBox, Button, Label, Orientation};
use gtk4_layer_shell::{Edge, KeyboardMode, Layer, LayerShell};

const APP_ID: &str = "org.lunnaos.Sel(l)enne";

fn css() -> &'static str {
    r#"
    * { font-family: Sans; }
    window { background: transparent; }
    .panel { background: rgba(7, 9, 35, 0.94); border-bottom: 1px solid rgba(150,90,255,0.45); }
    .dock { background: rgba(10, 8, 35, 0.90); border: 1px solid rgba(155,100,255,0.55); border-radius: 22px; padding: 8px; }
    .glass { background: rgba(19, 17, 55, 0.93); border: 1px solid rgba(150,100,255,0.48); border-radius: 18px; padding: 18px; }
    button { color: #f6f1ff; background: rgba(55,42,105,0.70); border: 1px solid rgba(170,120,255,0.35); border-radius: 12px; padding: 8px 13px; }
    button:hover { background: rgba(120,65,235,0.82); }
    .brand { color: #ffffff; font-size: 16px; font-weight: 700; }
    .muted { color: #bcb1dd; }
    .accent { color: #c88cff; font-weight: 700; }
    .title { color: #ffffff; font-size: 28px; font-weight: 700; }
    .card { background: rgba(27,24,70,0.88); border: 1px solid rgba(140,100,235,0.32); border-radius: 15px; padding: 14px; }
    "#
}

fn install_css() {
    let provider = gtk::CssProvider::new();
    provider.load_from_data(css());
    if let Some(display) = gtk::gdk::Display::default() {
        gtk::style_context_add_provider_for_display(
            &display,
            &provider,
            gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
        );
    }
}

fn layer_window(app: &Application, namespace: &str, layer: Layer) -> ApplicationWindow {
    let window = ApplicationWindow::builder().application(app).build();
    window.init_layer_shell();
    window.set_namespace(Some(namespace));
    window.set_layer(layer);
    window
}

fn button(label: &str) -> Button {
    Button::with_label(label)
}

fn show_launcher(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .title("LunnaOS")
        .default_width(900)
        .default_height(650)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 18);
    root.add_css_class("glass");
    root.set_margin_top(16);
    root.set_margin_bottom(16);
    root.set_margin_start(16);
    root.set_margin_end(16);

    let head = GtkBox::new(Orientation::Horizontal, 12);
    let brand = Label::new(Some("🌙  LunnaOS"));
    brand.add_css_class("brand");
    head.append(&brand);

    let search = gtk::SearchEntry::new();
    search.set_placeholder_text(Some("Buscar aplicativos, arquivos, configurações..."));
    search.set_hexpand(true);
    head.append(&search);
    root.append(&head);

    let title = Label::new(Some("Sel(l)enne 1.0"));
    title.set_xalign(0.0);
    title.add_css_class("title");
    root.append(&title);

    let subtitle = Label::new(Some("Um desktop mais leve, calmo e aberto para um amanhã mais brilhante."));
    subtitle.set_xalign(0.0);
    subtitle.add_css_class("muted");
    root.append(&subtitle);

    let grid = GtkBox::new(Orientation::Horizontal, 10);
    for (name, icon) in [
        ("🌐  Navegador", "xdg-open https://www.google.com"),
        ("📁  Arquivos", "lunna-files"),
        ("⚙  Configurações", "lunna-settings"),
        (">_  Terminal", "xterm"),
        ("🛍  Lunna Store", "lunna-store"),
        ("🎮  Jogos", "lunna-games"),
    ] {
        let b = button(&format!("{}\n{}", icon.split("  ").next().unwrap_or(""), name));
        let command = name.to_string();
        b.connect_clicked(move |_| {
            let _ = std::process::Command::new("sh").arg("-lc").arg(match command.as_str() {
                "Navegador" => "xdg-open https://www.google.com",
                "Arquivos" => "lunna-files",
                "Configurações" => "lunna-settings",
                "Terminal" => "foot",
                "Lunna Store" => "lunna-store",
                "Jogos" => "lunna-games",
                _ => "true",
            }).spawn();
        });
        grid.append(&b);
    }
    root.append(&grid);

    let recent = Label::new(Some("Recentes\n\nManual do LunnaOS.pdf\nWallpaper Sel(l)enne\nNotas da versão 1.0\nTerminal — atualizações"));
    recent.set_xalign(0.0);
    recent.add_css_class("card");
    root.append(&recent);

    win.set_child(Some(&root));
    win.present();
}

fn show_control_center(app: &Application) {
    let win = ApplicationWindow::builder()
        .application(app)
        .title("Centro de Ações — LunnaOS")
        .default_width(430)
        .default_height(650)
        .build();

    let root = GtkBox::new(Orientation::Vertical, 12);
    root.set_margin_top(18); root.set_margin_bottom(18); root.set_margin_start(18); root.set_margin_end(18);
    root.add_css_class("glass");

    let title = Label::new(Some("🌙  Centro de Ações"));
    title.add_css_class("title");
    title.set_xalign(0.0);
    root.append(&title);

    let grid = GtkBox::new(Orientation::Vertical, 8);
    for row in [
        "📶  Wi-Fi       MinhaCasa_5G",
        "ᛒ  Bluetooth   Ativado",
        "🌙  Modo Escuro    Ativado",
        "☀  Luz Noturna    Ativada",
        "◐  Não Perturbe    Desativado",
        "🎮  Modo Jogo      Ativado",
        "🔊  Volume         70%",
        "☀  Brilho         80%",
        "🔋  Bateria        87% · 3h42 restantes",
        "🎵  Horizontes de Lunna   02:14 / 04:37",
    ] {
        let b = button(row);
        b.set_hexpand(true);
        grid.append(&b);
    }
    root.append(&grid);

    win.set_child(Some(&root));
    win.present();
}

fn show_simple_app(app: &Application, title: &str, body: &str) {
    let win = ApplicationWindow::builder().application(app).title(title).default_width(920).default_height(650).build();
    let root = GtkBox::new(Orientation::Vertical, 16);
    root.set_margin_top(22); root.set_margin_bottom(22); root.set_margin_start(22); root.set_margin_end(22);
    root.add_css_class("glass");
    let h = Label::new(Some(title));
    h.add_css_class("title"); h.set_xalign(0.0);
    root.append(&h);
    let b = Label::new(Some(body));
    b.add_css_class("muted"); b.set_xalign(0.0);
    root.append(&b);
    win.set_child(Some(&root));
    win.present();
}

fn build_panel(app: &Application) {
    let win = layer_window(app, "lunna-panel", Layer::Top);
    win.set_default_size(0, 36);
    win.set_anchor(Edge::Top, true);
    win.set_anchor(Edge::Left, true);
    win.set_anchor(Edge::Right, true);
    win.set_exclusive_zone(36);
    win.set_keyboard_mode(KeyboardMode::None);

    let panel = GtkBox::new(Orientation::Horizontal, 8);
    panel.add_css_class("panel");
    panel.set_margin_start(8);
    panel.set_margin_end(8);

    let luna = button("🌙  LunnaOS");
    let a = app.clone();
    luna.connect_clicked(move |_| show_launcher(&a));
    panel.append(&luna);

    for text in ["Aplicativos", "Locais", "Sistema"] {
        panel.append(&button(text));
    }

    let spacer = GtkBox::new(Orientation::Horizontal, 0);
    spacer.set_hexpand(true);
    panel.append(&spacer);

    let center = button("Seg, 28 de Abr   22:24");
    panel.append(&center);

    let actions = button("⌕  🔔  ◈  Wi-Fi  87%");
    let a = app.clone();
    actions.connect_clicked(move |_| show_control_center(&a));
    panel.append(&actions);

    win.set_child(Some(&panel));
    win.present();
}

fn build_dock(app: &Application) {
    let win = layer_window(app, "lunna-dock", Layer::Top);
    win.set_default_size(650, 76);
    win.set_anchor(Edge::Bottom, true);
    win.set_margin(Edge::Bottom, 12);
    win.set_keyboard_mode(KeyboardMode::None);

    let dock = GtkBox::new(Orientation::Horizontal, 10);
    dock.add_css_class("dock");

    let entries = [
        ("🌐", "Navegador"),
        ("📁", "Arquivos"),
        ("⚙", "Configurações"),
        (">_", "Terminal"),
        ("🛍", "Lunna Store"),
        ("🎮", "Steam"),
        ("◉", "Câmera"),
        ("⌁", "Monitor"),
        ("🗑", "Lixeira"),
    ];

    for (icon, name) in entries {
        let b = button(icon);
        b.set_tooltip_text(Some(name));
        let a = app.clone();
        b.connect_clicked(move |_| {
            match name {
                "Configurações" => show_simple_app(&a, "Configurações", "Sistema · Bluetooth · Rede · Personalização · Aplicativos · Contas · Privacidade e Segurança · Energia · Tela · Som · Armazenamento · Sobre"),
                "Arquivos" => show_simple_app(&a, "Arquivos", "Início · Documentos · Downloads · Imagens · Músicas · Vídeos · Projetos · Público · Lixeira"),
                "Lunna Store" => show_simple_app(&a, "LunnaOS Central de Aplicativos", "Aplicativos em destaque · Categorias · Instalados · Atualizações · Biblioteca"),
                "Monitor" => show_simple_app(&a, "Gerenciador de Tarefas", "CPU · Memória · Disco · GPU · Rede · Processos"),
                _ => {}
            }
        });
        dock.append(&b);
    }

    win.set_child(Some(&dock));
    win.present();
}

fn build_app(app: &Application) {
    install_css();
    build_panel(app);
    build_dock(app);

    let args: Vec<String> = std::env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("--settings") => show_simple_app(app, "Configurações", "LunnaOS Sel(l)enne 1.0\n\nSistema · Bluetooth · Rede · Personalização · Aplicativos · Contas · Privacidade e Segurança · Energia · Tela · Som · Armazenamento · Sobre"),
        Some("--files") => show_simple_app(app, "Arquivos", "Área de Trabalho · Documentos · Downloads · Imagens · Músicas · Vídeos · Projetos · Público · Lixeira"),
        Some("--store") => show_simple_app(app, "LunnaOS Central de Aplicativos", "Descubra aplicativos, jogos e ferramentas para o LunnaOS."),
        Some("--tasks") => show_simple_app(app, "Gerenciador de Tarefas", "CPU 28% · Memória 42% · Disco 9% · GPU 36% · Rede 12%"),
        Some("--games") => show_simple_app(app, "Lunna Games", "Solitaire · Patience · Tetris · Minesweeper · Snake · Sudoku"),
        _ => {}
    }
}

fn main() {
    let app = Application::builder().application_id(APP_ID).build();
    app.connect_activate(build_app);
    app.run();
}
