pub type ValidResult = Result<bool, String>;

pub trait Validate {
     fn is_valid(&self) -> ValidResult;
}