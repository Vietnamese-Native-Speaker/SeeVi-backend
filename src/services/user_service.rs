pub struct UserService {
    
}

impl UserService {
    pub async fn register(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn authenticate(&mut self, username: Option<String>, email: Option<String>, password: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    //Forget password = change password
    pub async fn change_password(&mut self, _password: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_email(&mut self, _email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_username(&mut self, _username: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_name(&mut self, _first_name: String, _last_name: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_country(&mut self, _country: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_skills(&mut self, _skill: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_skills(&mut self, _skill: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_cv(&mut self, _cv: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_cv(&mut self, _cv:Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_email(&mut self, _email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_email(&mut self, _email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_about(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_avatar(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_cover_photo(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_friend(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_friend(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_education(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_education(&mut self) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_username(_username: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_email(_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_id(_user_id: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn get_friends_of_user(&self) -> Result<Vec<User>, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
}