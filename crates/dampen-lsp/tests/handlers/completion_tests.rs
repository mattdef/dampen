use dampen_lsp::document::DocumentState;
use dampen_lsp::handlers::completion::completion;
use tower_lsp::lsp_types::{
    CompletionParams, Position, TextDocumentIdentifier, TextDocumentPositionParams, Url,
};

fn create_params(uri: Url, line: u32, character: u32) -> CompletionParams {
    CompletionParams {
        text_document_position: TextDocumentPositionParams {
            text_document: TextDocumentIdentifier { uri },
            position: Position { line, character },
        },
        work_done_progress_params: Default::default(),
        partial_result_params: Default::default(),
        context: None,
    }
}

fn create_doc(content: &str) -> (DocumentState, Url) {
    let uri = Url::parse("file:///test.dampen").unwrap();
    (DocumentState::new(uri.clone(), content.to_string(), 1), uri)
}

#[test]
fn test_complete_widget_names() {
    let (doc, uri) = create_doc("<");
    let params = create_params(uri, 0, 1);

    let response = completion(&doc, params).unwrap();

    if let tower_lsp::lsp_types::CompletionResponse::Array(items) = response {
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "button"));
        assert!(items.iter().any(|i| i.label == "column"));
    } else {
        panic!("Expected Array response");
    }
}

#[test]
fn test_complete_attributes() {
    let (doc, uri) = create_doc("<button ");
    let params = create_params(uri, 0, 8);

    let response = completion(&doc, params).unwrap();

    if let tower_lsp::lsp_types::CompletionResponse::Array(items) = response {
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label == "label")); // Optional
        assert!(items.iter().any(|i| i.label == "on_click")); // Event
        assert!(items.iter().any(|i| i.label == "background")); // Style
    } else {
        panic!("Expected Array response");
    }
}

#[test]
fn test_complete_values_boolean() {
    let (doc, uri) = create_doc("<button enabled=\"");
    let params = create_params(uri, 0, 17);

    let response = completion(&doc, params).unwrap();

    if let tower_lsp::lsp_types::CompletionResponse::Array(items) = response {
        assert_eq!(items.len(), 2);
        assert!(items.iter().any(|i| i.label == "true"));
        assert!(items.iter().any(|i| i.label == "false"));
    } else {
        panic!("Expected Array response");
    }
}

#[test]
fn test_complete_values_color() {
    let (doc, uri) = create_doc("<button background=\"");
    let params = create_params(uri, 0, 20);

    let response = completion(&doc, params).unwrap();

    if let tower_lsp::lsp_types::CompletionResponse::Array(items) = response {
        assert!(!items.is_empty());
        assert!(items.iter().any(|i| i.label.starts_with("#")));
    } else {
        panic!("Expected Array response");
    }
}

#[test]
fn test_complete_unknown_context() {
    let (doc, uri) = create_doc("just text");
    let params = create_params(uri, 0, 5);

    let response = completion(&doc, params).unwrap();

    if let tower_lsp::lsp_types::CompletionResponse::Array(items) = response {
        assert!(items.is_empty());
    } else {
        panic!("Expected Array response");
    }
}
