use std::fmt;

pub enum FriendsListError {
    UserNotFound,

    // Error when add friend fails
    AddFriendFailed,
}

impl fmt::Display for FriendsListError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            FriendsListError::UserNotFound => {
                write!(f, "User not found")
            }

            // Display message for add friend failed
            FriendsListError::AddFriendFailed => {
                write!(f, "Failed to add friend")
            }
        }
    }
}
