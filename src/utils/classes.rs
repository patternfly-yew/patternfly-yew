use yew::Classes;

pub trait AsClasses {
    fn as_classes(&self) -> Classes;
}

impl AsClasses for dyn ToString {
    fn as_classes(&self) -> Classes {
        Classes::from(self.to_string())
    }
}

impl<T> AsClasses for Vec<T>
where
    T: AsClasses,
{
    fn as_classes(&self) -> Classes {
        let mut result = Classes::new();
        for i in self {
            result.extend(i.as_classes());
        }
        result
    }
}
