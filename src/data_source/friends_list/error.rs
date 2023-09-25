use std::fmt;

#[derive(Debug)]
pub enum FriendsListError {
    UserNotFound,

    // Error when add friend fails
    AddFriendFailed,

    // Friend request not found
    FriendRequestNotFound,

    // Friend request already exists
    FriendRequestAlreadyExist,

    // Update friend request failed
    UpdateFriendRequestFailed,

    // Database error
    DatabaseError,
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

            // Display message for friend request not found
            FriendsListError::FriendRequestNotFound => {
                write!(f, "Friend request not found")
            }

            // Display message for friend request already exists
            FriendsListError::FriendRequestAlreadyExist => {
                write!(f, "Friend request already exists")
            }

            // Display message for update friend request failed
            FriendsListError::UpdateFriendRequestFailed => {
                write!(f, "Failed to update friend request")
            }

            // Display message for database error
            FriendsListError::DatabaseError => {
                write!(f, "Database error")
            }
        }
    }
}
