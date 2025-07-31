use mediaparser::extract_subtitles;

#[tokio::test]
async fn test_read_local_subtitles() {
    let path = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/testdata/output_with_subs.mp4"
    );
    let subtitles = extract_subtitles(path.to_string()).await;

    assert!(
        subtitles.is_ok(),
        "Erro ao ler legendas: {:?}",
        subtitles.err()
    );
    let subtitles = subtitles.unwrap();

    assert!(!subtitles.is_empty(), "Nenhuma legenda encontrada");
    let first = &subtitles[0];
    assert_eq!(first.text, "[SERENE MUSIC]");
    assert!(!first.start.is_empty(), "O campo 'start' está vazio");
}
