use yew::Classes;

pub trait AsClasses {
    fn as_classes(&self) -> Classes {
        let mut classes = Classes::new();
        self.extend(&mut classes);
        classes
    }

    fn extend(&self, classes: &mut Classes);
}

impl AsClasses for dyn ToString {
    fn as_classes(&self) -> Classes {
        Classes::from(self.to_string())
    }

    fn extend(&self, classes: &mut Classes) {
        classes.extend(Classes::from(self.to_string()))
    }
}

impl<T> AsClasses for Vec<T>
where
    T: AsClasses,
{
    fn extend(&self, classes: &mut Classes) {
        for i in self {
            classes.extend(i.as_classes());
        }
    }
}
