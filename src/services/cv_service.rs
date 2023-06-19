pub struct CVService {
    
}

impl CVService {
    pub async fn change_title(&mut self, title: String) -> Result<CV, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_item_interactions(&mut self) -> Result<CV, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_description(&mut self, _description: String) -> Result<CV, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_tags(&mut self, _tag: String) -> Result<CV, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_tags(&mut self, _tag: String) -> Result<CV, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_suggested_cvs(&self) -> Result<Vec<CV>, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
}