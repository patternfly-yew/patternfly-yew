use yew::Classes;

pub trait AsClasses {
    fn as_classes(&self) -> Classes {
        let mut classes = Classes::new();
        self.extend(&mut classes);
        classes
    }

    fn extend(&self, classes: &mut Classes);
}

impl AsClasses for String {
    fn extend(&self, classes: &mut Classes) {
        classes.push(self)
    }
}

impl AsClasses for &str {
    fn extend(&self, classes: &mut Classes) {
        classes.push(self.to_string())
    }
}

impl AsClasses for u16 {
    fn extend(&self, classes: &mut Classes) {
        classes.push(self.to_string())
    }
}

impl AsClasses for dyn ToString {
    fn as_classes(&self) -> Classes {
        Classes::from(self.to_string())
    }

    fn extend(&self, classes: &mut Classes) {
        classes.extend(Classes::from(self.to_string()))
    }
}

impl<T: AsClasses> AsClasses for Option<T> {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Some(a) => a.extend(classes),
            None => {}
        }
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

pub trait ExtendClasses<A: AsClasses> {
    fn extend_from(&mut self, from: &A);
}

impl<A: AsClasses> ExtendClasses<A> for Classes {
    fn extend_from(&mut self, from: &A) {
        from.extend(self)
    }
}
