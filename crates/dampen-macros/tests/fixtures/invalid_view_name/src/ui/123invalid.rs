use dampen_macros::UiModel;

#[derive(UiModel, Default)]
pub struct InvalidModel {
    pub value: String,
}
