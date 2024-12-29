pub trait Buildable {
    type Builder: ConsumingBuilder<Built = Self>;

    fn builder() -> Self::Builder;
}

pub trait ConsumingBuilder {
    type Built: Buildable<Builder = Self>;

    fn build(self) -> Self::Built;
}

pub trait BuildInto<T>: ConsumingBuilder {
    fn build_into(self) -> T;
}

impl<B, T> BuildInto<T> for B
where
    B: ConsumingBuilder,
    B::Built: Into<T>,
{
    fn build_into(self) -> T {
        self.build().into()
    }
}
