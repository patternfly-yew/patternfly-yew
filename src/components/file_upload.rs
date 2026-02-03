use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileUploadProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub drag_over: bool,
    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// File upload component
///
/// > A **file upload** component allows the users to upload a single file into the browser.
///
/// See: <https://www.patternfly.org/components/file-upload>
///
/// # Properties
///
/// Defined in [`FileUploadProperties`].
#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProperties) -> Html {
    let mut class = classes!("pf-v6-c-file-upload");

    if props.drag_over {
        class.push(classes!("pf-m-drag-hover"));
    }

    html! (
        <div {class} ref={props.r#ref.clone()}>
            { props.children.clone() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileUploadSelectProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(FileUploadSelect)]
pub fn file_upload_select(props: &FileUploadSelectProperties) -> Html {
    html!(
        <div class="pf-v6-c-file-upload__file-select">
            { props.children.clone() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileUploadDetailsProperties {
    /// The details section, supposed to be a single [`crate::components::TextArea`].
    #[prop_or_default]
    pub children: Html,

    /// Set flag if upload is processing.
    ///
    /// This will render a spinner on the detail component section.
    #[prop_or_default]
    pub processing: bool,

    /// If the validation state is invalid
    #[prop_or_default]
    pub invalid: bool,
}

#[function_component(FileUploadDetails)]
pub fn file_upload_select(props: &FileUploadDetailsProperties) -> Html {
    html!(
        <div class="pf-v6-c-file-upload__file-details">
            { props.children.clone() }
            if props.processing {
                <div class="pf-v6-c-file-upload__file-details-spinner">
                    <span
                        class="pf-v6-c-spinner pf-m-lg"
                        role="progressbar"
                        aria-label="Loading..."
                    >
                        <span class="pf-v6-c-spinner__clipper"></span>
                        <span class="pf-v6-c-spinner__lead-ball"></span>
                        <span class="pf-v6-c-spinner__tail-ball"></span>
                    </span>
                </div>
            }
        </div>
    )
}
