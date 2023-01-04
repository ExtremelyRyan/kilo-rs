

pub type StdResult<T,E> = std::result::Result<T,E>;


#[derive(Debug)]
pub enum EditorResult {
    KeyReadFail,
}