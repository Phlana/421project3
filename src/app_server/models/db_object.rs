pub trait DBObject {
    fn new() -> Self ;
    fn find_all() -> Result<String,String> ;
    fn find_by_id(id: String) -> Result<Self, String> where Self: Sized;
    fn remove_by_id(id: String) -> Result<Self, String> where Self: Sized;
    fn save(&self) -> Result<(),String>;
}