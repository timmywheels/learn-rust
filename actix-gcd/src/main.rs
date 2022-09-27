use std::fmt::format;
use actix_web::{web, App, HttpResponse, HttpServer};
use serde::Deserialize;

fn main() {
    let server = HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(get_index))
            .route("/gcd", web::post().to(post_gcd))
    });

    println!("Listening on localhost:3000...");
    server
        .bind("127.0.0.1:3000")
        .expect("Error binding server to port 3000...")
        .run()
        .expect("Error starting server...");
}

fn post_gcd(form: web::Form<GcdParameters>) -> HttpResponse {
    if form.n == 0 || form.m == 0 {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body("Computing the GCD with 0 is boring.");
    }

    let response = format!("The greatest common divisor of the numbers {} and {} is <b>{}</b>\n", form.n, form.m, gcd(form.n, form.m));

    HttpResponse::Ok()
        .content_type("text/html")
        .body(response)

}

fn get_index() -> HttpResponse {
    HttpResponse::Ok().content_type("text/html").body(
        r#########################################################"
            <title>Greatest Common Denominator Calculator</title>
            <form action="/gcd" method="post">
                <input type="text" name="n"/>
                <input type="text" name="m"/>
                <button type="submit">Compute GCD</button>
            </form>
        "#########################################################,
    )
}

fn gcd(mut n: u64, mut m: u64) -> u64 {
    assert!(n != 0 && m != 0);
    while m != 0 {
        if m < n {
            let t = m;
            m = n;
            n = t;
        }
        m = m % n;
    }
    n
}

#[derive(Deserialize)]
struct GcdParameters {
    n: u64,
    m: u64,
}
