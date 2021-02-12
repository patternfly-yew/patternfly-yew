use yew::Classes;

pub trait AsClasses {
    fn as_classes(&self) -> Classes;
}

impl AsClasses for dyn ToString {
    fn as_classes(&self) -> Classes {
        Classes::from(self.to_string())
    }
}
