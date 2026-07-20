use rust_system_design::tiny_url::TinyUrlService;

fn main() {
    let mut service = TinyUrlService::new();

    let link = service
        .create_link("academy", "https://jeresoft.academy/cursos/system-design")
        .expect("la URL del curso es válida");

    let destination = service
        .resolve(&link.code)
        .expect("el enlace recién creado debe resolver");

    println!("{} -> {}", link.short_url, destination);
}
