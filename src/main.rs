use wkhtmltopdf::*;
fn main() {
    let template = include_bytes!("./template.html");
    let html = String::from_utf8(template.to_vec()).unwrap();
    let pdf_app = PdfApplication::new().expect("Failed to init PDF application");
    let mut pdfout = pdf_app.builder()
        .orientation(Orientation::Landscape)
        .margin(Size::Inches(2))
        .title("Awesome Foo")
        .build_from_html(&html)
        .expect("failed to build pdf");

    pdfout.save("foo.pdf").expect("failed to save foo.pdf");
}
