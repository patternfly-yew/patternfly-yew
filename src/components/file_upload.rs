use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileUploadProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub drag_over: bool,
    #[prop_or_default]
    pub r#ref: NodeRef,
}

/// File upload component
///
/// > A **file upload** component allows the users to upload a single file into the browser.
///
/// See: <https://www.patternfly.org/v4/components/file-upload>
///
/// # Properties
///
/// Defined in [`FileUploadProperties`].
#[function_component(FileUpload)]
pub fn file_upload(props: &FileUploadProperties) -> Html {
    let mut class = classes!("pf-v5-c-file-upload");

    if props.drag_over {
        class.push(classes!("pf-m-drag-hover"));
    }

    html! (
        <div {class} ref={props.r#ref.clone()}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileUploadSelectProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(FileUploadSelect)]
pub fn file_upload_select(props: &FileUploadSelectProperties) -> Html {
    html!(
        <div class="pf-v5-c-file-upload__file-select">
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct FileUploadDetailsProperties {
    /// The details section, supposed to be a single [`crate::components::TextArea`].
    #[prop_or_default]
    pub children: Children,

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
        <div class="pf-v5-c-file-upload__file-details">
            { for props.children.iter() }
            if props.processing {
                <div class="pf-v5-c-file-upload__file-details-spinner">
                    <span
                        class="pf-v5-c-spinner pf-m-lg"
                        role="progressbar"
                        aria-label="Loading..."
                    >
                        <span class="pf-v5-c-spinner__clipper"></span>
                        <span class="pf-v5-c-spinner__lead-ball"></span>
                        <span class="pf-v5-c-spinner__tail-ball"></span>
                    </span>
                </div>
            }
        </div>
    )
}
