use strum::{EnumCount, EnumIter, VariantMetadata, VariantNames};

#[derive(Clone, Copy, PartialEq, Eq, EnumCount, EnumIter, VariantNames)]
pub enum Page {
  Home,
  Games,
  Projects,
}

impl VariantMetadata for Page {
  const VARIANT_COUNT: usize = Self::COUNT;
  const VARIANT_NAMES: &'static [&'static str] = Self::VARIANTS;

  fn variant_name(&self) -> &'static str {
    #[allow(clippy::indexing_slicing)]
    Self::VARIANTS[*self as usize]
  }
}
