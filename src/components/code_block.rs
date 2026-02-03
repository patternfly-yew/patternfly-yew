//! Code block
use yew::prelude::*;

/// Properties for [`CodeBlock`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CodeBlockProperties {
    #[prop_or_default]
    pub expandable: bool,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub actions: ChildrenWithProps<CodeBlockAction>,
}

/// Code Block component
///
/// > A **code block** is a component that contains 2 or more lines of read-only code. The code in a code block can be copied to the clipboard.
///
/// See: <https://www.patternfly.org/components/code-block>
///
/// ## Properties
///
/// Defined by [`CodeBlockProperties`].
///
/// ## Children
///
/// A code block can contain any children, but is expected to contain a [`CodeBlockCode`] component.
///
/// It may also be wrapped with a detached [`crate::prelude::ExpandableSection`] component. The
/// [`crate::prelude::ExpandableSectionToggle`] would then be a child of this component, but stay outside the nested
/// code component.
///
/// ## Example
///
/// A simple example would be:
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   html!(
///     <CodeBlock>
///       <CodeBlockCode>{r#"some code"#}</CodeBlockCode>
///     </CodeBlock>
///   )
/// }
/// ```
#[function_component(CodeBlock)]
pub fn code_block(props: &CodeBlockProperties) -> Html {
    html!(
        <div class="pf-v6-c-code-block">
            if !props.actions.is_empty() {
                <div class="pf-v6-c-code-block__header">
                    <div class="pf-v6-c-code-block__actions">
                        { for props.actions.iter() }
                    </div>
                </div>
            }

            <div class="pf-v6-c-code-block__content">
                { props.children.clone() }
            </div>
        </div>
    )
}

/// Properties for [`CodeBlockCode`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CodeBlockCodeProperties {
    #[prop_or_default]
    pub children: Html,
}

/// The actual code component of the Code Block component.
///
/// ## Properties
///
/// Defined by [`CodeBlockCodeProperties`].
#[function_component(CodeBlockCode)]
pub fn code_block_code(props: &CodeBlockCodeProperties) -> Html {
    html!(
        <pre class="pf-v6-c-code-block__pre">
            <code class="pf-v6-c-code-block__code">{ props.children.clone() }</code>
        </pre>
    )
}

/// Properties for [`CodeBlockAction`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct CodeBlockActionProperties {
    #[prop_or_default]
    pub children: Html,
}

/// An action of a [`CodeBlock`]
#[function_component(CodeBlockAction)]
pub fn code_block_action(props: &CodeBlockActionProperties) -> Html {
    html!(
        <div class="pf-v6-c-code-block__actions-item">
            { props.children.clone() }
        </div>
    )
}
