use opencc_rust::converter;

#[test]
fn taiwan_quality_and_idempotence() {
    for locale in ["tw2", "twp"] {
        let c = converter("cn", locale).unwrap();
        for line in include_str!("taiwan-quality.tsv").lines() {
            let parts: Vec<_> = line.split('\t').collect();
            if parts[0] != "both" && parts[0] != locale {
                continue;
            }
            let (input, expected) = (parts[1], parts[2]);
            assert_eq!(c.convert(input), expected, "{locale}: {input}");
            assert_eq!(
                c.convert(expected),
                expected,
                "idempotence {locale}: {expected}"
            );
            assert!(!expected.contains(['賬', '羣']));
        }
    }
}
