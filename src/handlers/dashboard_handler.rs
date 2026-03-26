use axum::response::Html;
use local_ip_address::local_ip;
use qrcodegen::{QrCode, QrCodeEcc};

// ==========================================
// HANDLERS: TEACHER DASHBOARD (QR CODE)
// ==========================================

/// Generates an SVG string from a QrCode object
fn to_svg_string(qr: &QrCode, border: i32) -> String {
    let mut svg = String::new();
    let dimension = qr.size() + border * 2;

    svg.push_str(&format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 {0} {0}\" stroke=\"none\">\n",
        dimension
    ));
    svg.push_str("\t<rect width=\"100%\" height=\"100%\" fill=\"#FFFFFF\"/>\n");
    svg.push_str("\t<path d=\"");

    for y in 0..qr.size() {
        for x in 0..qr.size() {
            if qr.get_module(x, y) {
                if x != 0 || y != 0 {
                    svg.push_str(" ");
                }
                svg.push_str(&format!("M{},{}h1v1h-1z", x + border, y + border));
            }
        }
    }

    svg.push_str("\" fill=\"#000000\"/>\n");
    svg.push_str("</svg>\n");
    svg
}

/// GET /admin
/// Serves the Teacher's Dashboard with the connection URL and QR Code
pub async fn get_dashboard() -> Html<String> {
    let port = 3000;

    // Attempt to get the local network IP (e.g., Hotspot IP)
    // Fallback to localhost if no network is found to prevent crashing
    let ip = local_ip()
        .map(|ip| ip.to_string())
        .unwrap_or_else(|_| "127.0.0.1".to_string());

    let connection_url = format!("http://{}:{}", ip, port);

    // Generate the QR Code with High Error Correction
    let qr = QrCode::encode_text(&connection_url, QrCodeEcc::High).unwrap();
    let svg_image = to_svg_string(&qr, 4);

    // Build the HTML response
    let html_content = format!(
        r#"
        <!DOCTYPE html>
        <html lang="en">
        <head>
            <meta charset="UTF-8">
            <meta name="viewport" content="width=device-width, initial-scale=1.0">
            <title>Teacher Dashboard - Offline Quiz</title>
            <style>
                body {{
                    font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
                    background-color: #f4f7f6;
                    color: #333;
                    text-align: center;
                    padding-top: 5vh;
                    margin: 0;
                }}
                .container {{
                    background: white;
                    max-width: 600px;
                    margin: 0 auto;
                    padding: 40px;
                    border-radius: 12px;
                    box-shadow: 0 4px 15px rgba(0,0,0,0.1);
                }}
                h1 {{ color: #2c3e50; margin-bottom: 10px; }}
                p {{ font-size: 1.1em; color: #555; }}
                .url-box {{
                    background: #e9ecef;
                    padding: 15px;
                    font-size: 1.5em;
                    font-weight: bold;
                    color: #0056b3;
                    border-radius: 8px;
                    margin: 20px 0;
                    word-break: break-all;
                }}
                .qr-container {{
                    margin: 30px auto;
                    width: 300px;
                    height: 300px;
                    border: 4px solid #fff;
                    box-shadow: 0 0 10px rgba(0,0,0,0.1);
                    border-radius: 8px;
                    overflow: hidden;
                }}
                .footer {{ margin-top: 30px; font-size: 0.9em; color: #888; }}
                a {{ color: #0056b3; text-decoration: none; font-weight: bold; }}
            </style>
        </head>
        <body>
            <div class="container">
                <h1>Offline Quiz Server</h1>
                <p>Ask your students to connect to your Wi-Fi Hotspot and scan the QR Code below:</p>
                
                <div class="qr-container">
                    {svg_image}
                </div>
                
                <p>Or manually type this address in their browser:</p>
                <div class="url-box">
                    {connection_url}
                </div>
                
                <div class="footer">
                    <p>API Documentation available at <a href="/swagger-ui" target="_blank">/swagger-ui</a></p>
                </div>
            </div>
        </body>
        </html>
        "#,
        svg_image = svg_image,
        connection_url = connection_url
    );

    Html(html_content)
}
