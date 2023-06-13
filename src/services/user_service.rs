pub struct UserService;

impl UserService {
    pub async fn register(database: &mut impl UserDataSource, user_input: CreateUserInput) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn authenticate(username: Option<String>, email: Option<String>, password: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    //Forget password = change password
    pub async fn change_password(user_id: Uuid, new_password: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_email(user_id: Uuid, new_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_username(user_id: Uuid, new_username: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_name(user_id: Uuid, new_first_name: String, new_last_name: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_country(user: Uesr, new_country: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_skills(user_id: Uuid, new_skill: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_skills(user_id: Uuid, to_remove_skill: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_cv(user_id: Uuid, new_cv: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_cv(user_id: Uuid, to_remove_cv:Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_email(user_id: Uuid, new_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_email(user_id: Uuid, to_remove_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_about(user_id: Uuid, new_about: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_avatar(user_id: Uuid, new_avatar: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn change_cover_photo(user_id: Uuid, new_cover_photo: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_friend(user_id: Uuid, new_friend: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_friend(user_id: Uuid, to_remove_friend: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn add_education(user_id: Uuid, new_education: Education) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn remove_education(user_id: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_username(_username: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_email(_email: String) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn find_user_by_id(user_id: Uuid) -> Result<User, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
    pub async fn get_friends_of_user(user_id: Uuid) -> Result<Vec<User>, dyn Box<impl Debug + Clone + Copy>> {
        unimplemented!()
    }
}