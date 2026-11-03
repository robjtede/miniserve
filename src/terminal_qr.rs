//! Terminal QR output for fonts that do not contain Unicode block glyphs.

use fast_qr::qr::QRCode;

pub fn render(qr: &QRCode) -> String {
    let mut output = String::new();

    // Four light modules form the quiet zone. Two spaces per module keep
    // the code square with the usual terminal cell proportions.
    for y in 0..qr.size + 8 {
        for x in 0..qr.size + 8 {
            let dark = (4..qr.size + 4).contains(&y)
                && (4..qr.size + 4).contains(&x)
                && qr[y - 4][x - 4].value();

            // QR colors must remain explicit even when log colors are disabled.
            output.push_str(if dark { "\x1b[40m  " } else { "\x1b[107m  " });
        }

        output.push_str("\x1b[0m\n");
    }

    output
}

#[cfg(test)]
mod tests {
    use super::*;
    use fast_qr::QRBuilder;

    #[test]
    fn output_does_not_require_unicode_glyphs() {
        let qr = QRBuilder::new("http://192.168.1.16:8080").build().unwrap();

        assert!(render(&qr).is_ascii());
    }

    #[test]
    fn output_preserves_modules_and_quiet_zone() {
        let qr = QRBuilder::new("http://192.168.1.16:8080").build().unwrap();
        let output = render(&qr);
        let rows: Vec<_> = output.lines().collect();

        assert_eq!(rows.len(), qr.size + 8);

        for (y, row) in rows.iter().enumerate() {
            let cells: Vec<_> = row
                .strip_suffix("\x1b[0m")
                .unwrap()
                .split_inclusive("  ")
                .collect();

            assert_eq!(cells.len(), qr.size + 8);

            for (x, cell) in cells.iter().enumerate() {
                let expected = if x >= 4 && y >= 4 && x < qr.size + 4 && y < qr.size + 4 {
                    qr[y - 4][x - 4].value()
                } else {
                    false
                };

                assert_eq!(
                    *cell,
                    if expected {
                        "\x1b[40m  "
                    } else {
                        "\x1b[107m  "
                    }
                );
            }
        }
    }
}
