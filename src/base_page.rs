pub fn create_base_file(title: &String) -> String {
    let base = r#"<!DOCTYPE html>
    <html>
    
    <head>
        <meta charset='utf-8'>
        <meta http-equiv='X-UA-Compatible' content='IE=edge'>
        title
        <meta name='viewport' content='width=device-width, initial-scale=1'>
    </head>
    
    <body>
    "#
    .to_string();

    let base = base.replace("title", &format!("<title>{}</title>", title));

    return base;
}

pub fn end_file() -> String {
    return String::from(
        r#"<style>
            body {
                font-family: -apple-system, BlinkMacSystemFont, "Segoe UI", Roboto, "Helvetica Neue", Arial, sans-serif;
                margin: 0;
                padding: 0;
                display: flex;
                justify-content: center;
                align-items: center;
                height: 100vh;
            }

            .column {
                max-width: 600px;
                /* Adjust the maximum width as needed */
                width: 100%;
                padding: 20px;
                text-align: center;
            }

            /* Add more styling as needed */
        </style>
        </body>

        </html>"#,
    );
}
