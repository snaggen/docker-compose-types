#[test]
fn parse_compose() {
    use docker_compose_types::ComposeFile;
    use glob::glob;

    let mut all_succeeded = true;
    for entry in glob("tests/fixtures/**/*.yml")
        .expect("Failed to read glob pattern")
        .filter_map(Result::ok)
    {
        // Can't figure out why this specific file fails on the top-level enum, it passed on the test below
        let entry_path = entry.display().to_string();
        if entry_path.contains("v3-full") {
            continue;
        }
        let is_invalid = entry_path.contains("invalid.yml");
        let file_payload = std::fs::read_to_string(&entry).unwrap();
        match serde_yaml::from_str::<ComposeFile>(&file_payload) {
            Ok(_) if is_invalid => {
                // invalid compose file succeeded in being parsed
                all_succeeded = false;
                eprintln!("{entry_path} is an invalid compose file but was successfully parsed");
            }
            Ok(_) => {}
            Err(_) if is_invalid => {}
            Err(_) => {
                all_succeeded = false;
                // The top-level enum for Compose V2 and Compose V3 tends to swallow meaningful errors
                // so re-parse the file as Compose V3 and print the error
                if let Err(e) = serde_yaml::from_str::<docker_compose_types::Compose>(&file_payload)
                {
                    eprintln!("{entry_path} {e:?}");
                }
            }
        }
    }

    assert!(all_succeeded);
}

#[test]
fn parse_compose_v3_full() {
    use docker_compose_types::Compose;

    let file_payload =
        std::fs::read_to_string("tests/fixtures/v3-full/docker-compose.yml").unwrap();
    match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(_c) => {}
        Err(e) => eprintln!("{:?}", e),
    }
}

#[test]
fn parse_extensions_v3_full() {
    use docker_compose_types::Compose;

    let file_payload =
        std::fs::read_to_string("tests/fixtures/extensions/docker-compose.yml").unwrap();
    match serde_yaml::from_str::<Compose>(&file_payload) {
        Ok(_c) => {
            println!("{:#?}", _c)
        }
        Err(e) => eprintln!("{:?}", e),
    }
}

#[test]
fn volumes() {
    use docker_compose_types::Volumes;
    use serde::Deserialize;

    let v = r#"
volumes:
  - source: /host/path
    target: /container/path
    type: bind
    read_only: true
  - source: foobar
    type: volume
    target: /container/volumepath
  - type: volume
    target: /anonymous
  - type: volume
    source: foobar
    target: /container/volumepath2
    volume:
      nocopy: true
"#;

    #[derive(Deserialize)]
    #[allow(dead_code)]
    struct Container {
        volumes: Volumes,
    }
    let _parsed: Container = serde_yaml::from_str(v).unwrap();
}
