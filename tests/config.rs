#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::write;

    #[test]
    fn test_parse_config() {
        let yaml = r#"
        device: auto
        threshold: 0.9
        max_attempts: 3
        model_path:
        detector: "/tmp/yunet.onnx"
        recognizer: "/tmp/arcfacemobile.onnx"
        log_level: debug
        log_console: true
        "#;
        let path = "/tmp/test_config.yaml";
        write(path, yaml).unwrap();

        let cfg = Config::load_from_file(path).unwrap();
        assert_eq!(cfg.device, "auto");
        assert_eq!(cfg.threshold, 0.9);
        assert_eq!(cfg.model_path.detector, "/tmp/yunet.onnx");
    }
}
